# Rune Firmware

Firmware for the Rune pocket AI companion. Runs Linux on Allwinner T113-S4 with an ESP32-S3 co-processor.

## Architecture

Rune uses a **single binary userspace application** running on a minimal Linux system, plus a separate firmware image for the ESP32-S3 co-processor.

- **T113-S4 (main processor):** Runs Linux 6.x (mainline kernel via Buildroot). One userspace binary handles all application logic -- voice queries, ePub reading, music playback, notification display, and photo capture.
- **ESP32-S3 (co-processor):** Handles BLE for phone notification mirroring and serves as WiFi fallback. Communicates with the T113 over UART.

## Languages

| Component | Language | Rationale |
|-----------|----------|-----------|
| Userspace app | Rust (preferred) | Safety, async (tokio), single binary deployment |
| Userspace app | C (fallback) | If Rust cross-compilation proves too painful |
| Kernel modules | C | Required by Linux |
| ESP32 firmware | C/C++ | Required by esp-idf |

## Key Dependencies

- **ALSA** -- audio capture (mic) and playback (amp) via I2S
- **spidev** -- e-ink display control over SPI
- **V4L2** -- USB UVC camera capture
- **BlueZ** -- BLE stack, proxied through ESP32 for notification mirroring
- **WebSocket client** -- streaming voice to/from cloud AI
- **ePub parser** -- book rendering and pagination

## Module Boundaries

The single binary is organized into these internal modules:

| Module | Responsibility |
|--------|---------------|
| `display` | E-ink framebuffer management, SPI communication, partial/full refresh |
| `audio` | ALSA capture/playback, PCM buffering, I2S routing |
| `voice` | Button-triggered recording, WebSocket streaming to cloud AI, response handling |
| `music` | Local file playback, librespot integration |
| `books` | ePub parsing, text pagination, page rendering |
| `notifications` | BLE notification relay via ESP32 UART protocol |
| `camera` | V4L2 capture from USB UVC camera |

## Documentation

- **[build.md](build.md)** -- how to build the firmware from source
- **[flash.md](flash.md)** -- how to get firmware onto the device
- **[architecture.md](architecture.md)** -- deep dive into firmware design and internals

## Project Structure

```
firmware/
  kernel/       -- Linux kernel config, device tree overlays, patches
  userspace/    -- Main Rune application (Rust or C)
  esp32/        -- ESP32-S3 co-processor firmware (esp-idf)
  buildroot/    -- Buildroot configuration for the Linux rootfs
```

## Current Status

Early development. Hardware interfaces are being defined, module boundaries are stabilizing, and the build pipeline is not yet automated. Expect everything to change.
