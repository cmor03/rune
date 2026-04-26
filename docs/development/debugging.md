# Debugging Tools and Techniques

How to diagnose problems when things don't work.

## Logic Analyzer (Saleae Clone)

A $15 8-channel Saleae Logic clone is the single most useful debugging tool for this project. It lets you see exactly what's happening on the wire.

### Software Setup

Use either:
- **PulseView** (open source, part of sigrok): `brew install --cask pulseview` on macOS, or `apt install pulseview` in the VM.
- **Saleae Logic 2** (proprietary, free for personal use): download from https://www.saleae.com/downloads/

Both support the cheap clones. Saleae Logic 2 is polished. PulseView is free and works fine.

### Inspecting SPI (E-Ink Display)

Connect 4 channels:

| Channel | Signal | What to Look For |
|---------|--------|------------------|
| CH0     | SCLK   | Clock should be steady at your configured frequency (e.g., 4 MHz) |
| CH1     | MOSI   | Data bytes matching your command/data sequence |
| CH2     | CS     | Should go LOW before each transaction, HIGH after |
| CH3     | DC     | LOW during command bytes, HIGH during data bytes |

In PulseView/Logic 2:
1. Add an SPI protocol decoder.
2. Set CLK = CH0, MOSI = CH1, CS = CH2.
3. Trigger on CS falling edge.
4. Capture and decode.

Verify the init command sequence matches the Waveshare reference. If CS never goes low, the SPI driver isn't opening the device. If SCLK is flat, the SPI peripheral isn't configured.

### Inspecting I2S (Audio)

Connect 3 channels:

| Channel | Signal | What to Look For |
|---------|--------|------------------|
| CH0     | BCLK   | Bit clock. Frequency = sample_rate x bits_per_channel x 2 |
| CH1     | LRCLK  | Word select. Frequency = sample_rate (e.g., 16 kHz) |
| CH2     | DATA   | DOUT from mic or DIN to amp |

In PulseView/Logic 2:
1. Add an I2S protocol decoder (available in both tools).
2. Set CLK = CH0, WS = CH1, DATA = CH2.
3. Capture during recording or playback.

Check:
- BCLK frequency: at 16kHz sample rate, 32-bit stereo, BCLK should be 16000 x 32 x 2 = 1.024 MHz.
- LRCLK frequency: should equal the sample rate (16000 Hz).
- LRCLK transitions: HIGH = right channel, LOW = left channel (standard I2S).
- Data: should be non-zero during recording if there's sound. All zeros = mic not outputting.

### Inspecting UART (ESP32 Comms)

Connect 2 channels:

| Channel | Signal | What to Look For |
|---------|--------|------------------|
| CH0     | TX (from T113) | Commands being sent |
| CH1     | TX (from ESP32) | Responses being sent |

In PulseView/Logic 2:
1. Add a UART decoder on each channel.
2. Set baud rate to 115200, 8N1.
3. Capture during a command exchange.

Verify:
- Sync bytes (0xAA 0x55) are present.
- Command byte matches what you sent.
- Response arrives after command.
- If you see nothing on one channel, TX/RX are swapped.

## Multimeter

### Voltage Rails

Check these regularly, especially when something stops working:

| Rail | Expected | Tolerance | Where to Measure |
|------|----------|-----------|------------------|
| LiPo | 3.0-4.2V | varies with charge | Battery terminals |
| 5V   | 5.0V     | +/- 0.2V  | Breadboard 5V rail |
| 3.3V | 3.3V     | +/- 0.1V  | Breadboard 3.3V rail |

If 5V is low (below 4.8V), the MP1584 needs readjustment or the load is too high.

If 3.3V is low, the dev board regulator might be overloaded (too many peripherals on 3.3V).

### Continuity

Use continuity mode (beep mode) to verify:
- Jumper wires are actually connected end-to-end (breadboard connections fail silently).
- Ground is shared between all boards.
- Pull-up resistors are connected correctly.

### Pull-Up Verification

To verify a pull-up resistor on a GPIO:
1. Measure voltage on the GPIO pin with the button not pressed. Should read ~3.3V.
2. Press the button. Should read ~0V.
3. If it reads 0V in both states, the pull-up is missing or the GPIO is configured as output-low.

## USB Current Meter

A USB current meter (inline between power supply and board) tells you:

- **Total system draw**: a fully loaded prototype with display updating, audio playing, camera active, and ESP32 running should be under 1A at 5V (5W). If it's higher, something's wrong (short, misconfigured regulator).
- **Idle draw**: should be 100-300mA. If it's much higher at idle, check for stuck peripherals or short circuits.
- **Power spikes**: e-ink full refresh draws a brief spike. Audio playback at high volume draws more. The MP1584 should handle it, but verify.

## Common Failure Modes

### Board Doesn't Boot

```
Symptom: no output on serial console after power-on.

Step 1: Is the board powered?
  → Check for power LED on the MangoPi.
  → Measure 5V rail with multimeter.
  → If no power: check LiPo charge, MP1584 output, cable connections.

Step 2: Is the serial adapter working?
  → ls /dev/tty.*serial* on macOS. Device should appear.
  → If not: try different USB port, install CH340 driver.

Step 3: Are TX/RX correct?
  → Swap TX and RX wires.
  → Try different UART pins (make sure you're on UART0, not UART2).

Step 4: Is the SD card good?
  → Try a different SD card.
  → Re-flash with balenaEtcher (not dd — Etcher verifies the write).
  → Try a known-good image.

Step 5: Is the baud rate correct?
  → Must be 115200. If you see garbled text, baud rate is wrong.
  → Try: picocom -b 115200 /dev/tty.usbserial-*
```

### Display Stays Blank

```
Symptom: no change on e-ink after running test program.

Step 1: Check SPI signals with logic analyzer.
  → Is CS going low? If not: wrong CS pin, or spidev not configured.
  → Is SCLK toggling? If not: SPI peripheral not enabled in device tree.
  → Is MOSI sending data? If not: software bug in the test program.

Step 2: Check BUSY line.
  → If BUSY is stuck LOW: display never finished init. Check RST sequence.
  → If BUSY is always HIGH: might not be connected (floating high).

Step 3: Check RST.
  → The reset sequence must pulse RST LOW then back HIGH.
  → If RST is stuck LOW, the display is held in reset.
  → Verify with logic analyzer or multimeter.

Step 4: Check DC pin.
  → Must be LOW when sending commands, HIGH when sending data.
  → If stuck in one state: wrong GPIO number in code, or GPIO not exported.

Step 5: Check power.
  → Measure VCC on the e-ink driver board. Should be 3.3V.
  → E-ink displays draw very little current, so power is rarely the issue.
```

### Audio Is Garbled

```
Symptom: recorded audio is noise/static, or playback sounds wrong.

Step 1: Verify BCLK frequency.
  → Use logic analyzer. Should be sample_rate x bits x 2.
  → For 16kHz, 32-bit stereo: 1.024 MHz.
  → Wrong frequency = wrong clock tree config in device tree.

Step 2: Check LRCLK polarity.
  → Standard I2S: data changes on BCLK falling edge, LRCLK low = left channel.
  → If LRCLK is inverted, channels are swapped (usually not catastrophic but can cause confusion).

Step 3: Confirm I2S master/slave config.
  → T113 should be master (drives BCLK and LRCLK).
  → Mic and amp should be slaves (receive BCLK and LRCLK).
  → If T113 is configured as slave, clocks won't run (no external master to drive them).

Step 4: Check bit format.
  → SPH0645: 24-bit data in 32-bit frame, MSB-first, left-justified or I2S standard.
  → MAX98357A: accepts 16-bit or 32-bit I2S.
  → Mismatch = garbled audio or silence.

Step 5: Check if it's a channel issue.
  → Record in stereo (-c 2), inspect both channels.
  → Mic might be on right channel only if L/R SEL is tied to VDD.
```

### WiFi Won't Connect

```
Symptom: wlan0 exists but can't connect to network.

Step 1: Check firmware blob.
  → ls /lib/firmware/rtl8723ds/
  → If empty or missing: the firmware needs to be added to the image.
  → dmesg | grep -i firmware — look for "firmware load failed" messages.

Step 2: Verify wpa_supplicant config.
  → cat /etc/wpa_supplicant.conf
  → SSID and password must be correct. Special characters need escaping.

Step 3: Manual connection test.
  → wpa_supplicant -i wlan0 -c /etc/wpa_supplicant.conf -d
  → The -d flag enables debug output. Look for authentication failures.

Step 4: Check antenna.
  → The MangoPi MQ-R has an onboard antenna or u.FL connector.
  → If using external antenna, make sure it's connected.
  → If using onboard antenna, make sure nothing is shielding it (metal enclosure, etc.).

Step 5: Frequency band.
  → RTL8723DS is 2.4GHz only. 5GHz networks won't appear.
```

### ESP32 Not Responding on UART

```
Symptom: T113 sends commands, no response from ESP32.

Step 1: Check UART TX/RX crossover.
  → T113 TX must go to ESP32 RX, and vice versa.
  → Swap wires and test again.

Step 2: Verify baud rate.
  → Both sides must be 115200 (or whatever you configured).
  → Wrong baud rate = garbage or nothing.

Step 3: Check ESP32 boot mode.
  → GPIO0 must be HIGH (or floating) during power-on for normal boot.
  → If GPIO0 is LOW, ESP32 enters download mode (silent on UART).
  → Power cycle the ESP32 with GPIO0 disconnected.

Step 4: Check if firmware is flashed.
  → Connect ESP32 via USB-C to your Mac.
  → idf.py monitor — if you see boot messages, firmware is running.
  → If you see nothing, re-flash.

Step 5: Loopback test.
  → Flash ESP32 with simple echo firmware.
  → Send bytes from T113, verify they come back.
```

## Where to Find Help

### linux-sunxi

- Wiki: https://linux-sunxi.org/ (T113 page, SPI, I2S, GPIO docs)
- Mailing list: linux-sunxi@lists.linux.dev
- IRC: #linux-sunxi on OFTC

The linux-sunxi community has deep knowledge of Allwinner SoCs. If you have a kernel or device tree question, this is the place.

### MangoPi Community

- Forum: https://forum.mangopi.org/
- GitHub: https://github.com/mangopi-sbc

Board-specific questions (pinout, power, USB OTG behavior).

### ESP32

- ESP-IDF documentation: https://docs.espressif.com/projects/esp-idf/
- ESP32 forum: https://esp32.com/
- GitHub issues: https://github.com/espressif/esp-idf/issues

BLE, WiFi, UART questions.

### General Embedded Linux

- Bootlin's embedded Linux training materials (free): https://bootlin.com/training/
- LKML (Linux kernel mailing list) for driver-level questions
- Stack Overflow [embedded] tag for quick questions

### Rune Project

- Project discussions on the repo (GitHub Discussions or Issues)
