# System Architecture

Rune is a three-tier system: device hardware, on-device software, and cloud services. This document covers all three.

---

## Hardware

### Processors

**Allwinner T113-S4** -- Main processor. Dual-core Cortex-A7 @ 1.2GHz, 128MB DDR3 integrated. Runs Linux and the main userspace application. Handles display rendering, audio processing, ePub layout, and USB camera capture.

**ESP32-S3** -- Co-processor. Handles WiFi (802.11 b/g/n), BLE 5.0, and acts as a network interface for the T113-S4. Runs its own firmware independently. Communicates with the T113 over UART.

### Display

3.7" monochrome e-ink, 480x280 pixels. Connected to the T113-S4 via SPI. Partial refresh supported for UI elements; full refresh for page turns and major screen changes. The display controller is driven directly from userspace via spidev.

### Audio

- **Microphone**: MEMS mic (SPH0645 or INMP441), I2S interface, connected to T113-S4.
- **Amplifier**: MAX98357A Class D amp, I2S interface, shared bus with mic. Drives a small mono speaker.
- **I2S bus**: Single shared bus on the T113-S4. Mic and amp are on the same I2S peripheral -- the firmware manages direction switching between capture and playback.

### Camera

USB UVC camera module. Connected to the T113-S4 via USB. Captured through standard V4L2 interface. Only powered when actively capturing a photo.

### Power

- **PMIC**: AXP2101. Manages battery charging, voltage rails, and power sequencing for both processors.
- **Battery**: 1000mAh LiPo single cell.
- **Charging**: USB-C, 5V input.
- **Rails**: 3.3V for logic, 1.8V for DDR3, configurable rails for display and peripherals.

### Physical Interface

Five buttons: up, down, select, back, action. No touchscreen. The button matrix connects directly to T113-S4 GPIO.

---

## Bus Topology

```
                    +------------------+
                    |    T113-S4       |
                    |  (Main CPU)      |
                    |                  |
          SPI -----| PA/PB GPIOs      |
          (display) |                  |
                    | I2S0             |----- MEMS Mic (SPH0645/INMP441)
                    | (shared bus)     |----- Amp (MAX98357A) -> Speaker
                    |                  |
          UART1 ----| TX/RX           |----- ESP32-S3 (co-processor)
                    |                  |
          USB ------| USB Host        |----- UVC Camera Module
                    |                  |
          I2C ------| I2C0            |----- AXP2101 (PMIC)
                    |                  |
          GPIO -----| PG/PH GPIOs     |----- 5x Buttons
                    +------------------+
```

**SPI** -- Display only. The e-ink controller receives framebuffer data and command bytes over SPI. Directly accessed from userspace via `/dev/spidevX.X`.

**I2S** -- Audio. Single I2S peripheral shared between mic input and amp output. Half-duplex in practice: the firmware switches between capture mode (voice query recording) and playback mode (music, TTS responses).

**UART** -- T113 to ESP32 communication. Custom binary protocol for network requests, BLE notification forwarding, and WiFi management commands. 115200 baud default, upgradeable to 921600.

**USB** -- Camera only. The UVC module enumerates as a standard USB video device. No hub, no other USB peripherals on the internal bus. The external USB-C port is wired to the PMIC for charging and to the T113 for FEL mode flashing.

**I2C** -- PMIC communication. The T113 reads battery state, sets voltage rails, and manages power sequencing through the AXP2101's I2C registers.

---

## Software Stack

### Linux (T113-S4)

- **Kernel**: Linux 6.x mainline with device tree patches for T113-S4 board support. Custom DTS defines pin muxing, SPI/I2S/UART peripherals, USB host, and GPIO button input.
- **Root filesystem**: Buildroot. Minimal image -- no systemd, no desktop environment. BusyBox init, custom startup scripts.
- **Userspace application**: Single binary (Rust or C) that owns the display, audio, buttons, and UART to ESP32. No window manager. The application renders directly to the e-ink framebuffer.

### ESP32-S3 Firmware

- Runs on ESP-IDF (FreeRTOS).
- Manages WiFi connection lifecycle (scan, connect, reconnect).
- Handles BLE peripheral role for phone notification sync (ANCS on iOS, GATT on Android).
- Proxies HTTP/WebSocket requests from the T113 -- the T113 sends a structured request over UART, the ESP32 executes it over WiFi, and returns the response over UART.
- OTA-updatable independently from the T113 firmware.

### Userspace Application Structure

```
+------------------------------------------+
|           Main Event Loop                |
|  (button input, UART rx, timers)         |
+-----+--------+--------+--------+--------+
      |        |        |        |        |
  +---v--+ +---v--+ +---v--+ +---v--+ +---v--+
  |Voice | |ePub  | |Music | |Notif | |Camera|
  |Query | |Reader| |Player| |Sync  | |      |
  +---+--+ +---+--+ +---+--+ +---+--+ +---+--+
      |        |        |        |        |
      v        v        v        v        v
  ESP32/   Direct   I2S/    ESP32/   USB/
  Cloud    render   ALSA    BLE      V4L2
```

The application is a single process with a main event loop. Each function module handles its own state but shares the display, audio, and network resources through the main loop's arbitration.

---

## Cloud Services

### Voice Query

Default backend: OpenAI Realtime API over WebSocket. The flow:

```
User presses action button
        |
        v
T113 starts I2S mic capture
        |
        v
Audio streamed over UART -> ESP32 -> WebSocket -> OpenAI
        |
        v
Response streamed back: ESP32 -> UART -> T113
        |
        v
T113 plays audio response via I2S amp
T113 renders text summary on e-ink
```

The AI provider is swappable. The UART-to-WebSocket bridge on the ESP32 doesn't care what's on the other end. Users can point it at any compatible API endpoint by changing a config file.

### Music

Two modes:

**Local playback**: MP3/FLAC/OGG files stored on the T113's onboard storage (SD card or SPI flash). Decoded on the T113, output via I2S to the amp.

**Spotify**: Uses librespot (open-source Spotify Connect client). The T113 runs librespot, which handles Spotify authentication and audio streaming. Audio decoded locally and output via I2S. Requires a Spotify Premium account.

```
Spotify servers <-> ESP32 (WiFi) <-> UART <-> T113 (librespot) -> I2S -> Speaker
```

### Phone Notifications

```
Phone (iOS/Android)
        |
        v  BLE
ESP32-S3 (ANCS / GATT notification service)
        |
        v  UART
T113-S4 (parse, render to e-ink)
```

The ESP32 maintains a BLE connection to the user's phone. On iOS, it subscribes to ANCS (Apple Notification Center Service). On Android, it uses a companion app that exposes notifications over a GATT service. Notification data is forwarded to the T113 for display.

### ePub Reading

Fully offline. ePub files are loaded onto the device via USB mass storage mode. The T113 parses the ePub, lays out text for the 480x280 display, and renders pages directly to the e-ink framebuffer. No cloud dependency.

### Photo Capture

Fully offline. The UVC camera captures a frame via V4L2 when the user presses the shutter (action button in camera mode). The image is stored locally. Optional: transfer photos to phone over BLE file transfer, or upload via WiFi to a user-configured endpoint.

---

## Data Flow Diagrams

### Voice Query

```
[Button Press] -> [T113: start mic capture]
                         |
                   [I2S mic data]
                         |
                   [T113: buffer audio, send over UART]
                         |
                   [ESP32: open WebSocket to AI provider]
                         |
                   [ESP32: stream audio to cloud]
                         |
                   [Cloud: process, generate response]
                         |
                   [ESP32: receive response over WebSocket]
                         |
                   [ESP32: forward to T113 over UART]
                         |
                   [T113: play audio via I2S amp]
                   [T113: render text on e-ink]
```

### ePub Reading

```
[USB Mass Storage] -> [ePub file on local storage]
                              |
[Button: open book] -> [T113: parse ePub, extract chapters]
                              |
                       [T113: lay out text for 480x280]
                              |
                       [T113: render page to e-ink via SPI]
                              |
[Button: next page] -> [T113: render next page]
```

### Music Playback (Spotify)

```
[User selects playlist] -> [T113: librespot connects via ESP32]
                                    |
                             [ESP32: WiFi to Spotify servers]
                                    |
                             [Audio data: Spotify -> ESP32 -> UART -> T113]
                                    |
                             [T113: decode audio]
                                    |
                             [T113: output via I2S -> MAX98357A -> Speaker]
                                    |
                             [T113: render track info on e-ink]
```

### Notification Sync

```
[Phone] -- BLE --> [ESP32: receive notification via ANCS/GATT]
                          |
                   [ESP32: serialize notification, send over UART]
                          |
                   [T113: parse notification]
                          |
                   [T113: render on e-ink display]
                          |
                   [Button: dismiss] -> [T113: send dismiss command over UART]
                                               |
                                        [ESP32: forward dismiss over BLE]
```

---

## Deliberately Not In Scope

These are not planned, not on a roadmap, and not welcome as feature requests:

- **No browser.** The web is an attention trap. Rune doesn't go there.
- **No app store.** Five functions. That's the product.
- **No video playback.** E-ink can't do it well, and we wouldn't want it to.
- **No on-device transcription.** The T113's dual Cortex-A7 cores cannot run a useful speech model. Voice goes to the cloud.
- **No cellular modem.** Rune is a companion device. It uses your phone's connection via BLE or connects to WiFi directly.
- **No touchscreen.** Buttons are deliberate. Touchscreens invite infinite scrolling.
- **No GPS.** Not a navigation device.
- **No NFC payments.** Not a wallet.
