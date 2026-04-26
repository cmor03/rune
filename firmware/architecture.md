# Rune Firmware Architecture

Deep dive into how the firmware is structured and how the pieces fit together.

## Single Binary Design

Rune runs as a single userspace process on Linux. No microservices, no separate daemons for each feature, no IPC over sockets. One binary, one process, multiple internal modules communicating through in-process message passing.

Why: simplicity. The T113-S4 has 128MB of RAM and dual Cortex-A7 cores. Process isolation buys us little here and costs startup time, memory, and complexity.

## Async Event Loop

The application is built around an async event loop. If using Rust, this means **tokio**. If using C, a hand-rolled event loop with epoll.

All modules register event handlers with a central dispatcher. The dispatcher routes events to the appropriate handler. Events include:

- Button presses (GPIO interrupts via /dev/input or /sys/class/gpio)
- Audio data ready (ALSA PCM buffer filled)
- Network responses (WebSocket messages, HTTP responses)
- BLE messages (relayed from ESP32 over UART)
- Timer events (display refresh, heartbeat, etc.)

No module polls. Everything is event-driven.

## Module Boundaries

### Display

```
Framebuffer (memory) --> SPI --> E-ink panel (3.7")
```

- Maintains a framebuffer in memory (1-bit or 4-bit grayscale depending on panel mode)
- On update: diff the new framebuffer against the previous one
- Send only changed regions via SPI (partial refresh) for text updates and UI changes
- Periodically perform a full refresh to clear e-ink ghosting artifacts
- SPI interface via Linux `spidev` (`/dev/spidev0.0`)
- GPIO pins for panel BUSY, RESET, and DC (data/command) signals

### Audio

```
I2S Mic --> ALSA capture --> PCM buffer --> [encode/stream to cloud]
[cloud/file decode] --> PCM buffer --> ALSA playback --> I2S Amp
```

- Uses ALSA directly (`libasound` / `alsa-sys` crate) -- no PulseAudio, no PipeWire
- Capture path: I2S MEMS microphone -> ALSA PCM capture device -> raw PCM buffer
- Playback path: PCM buffer -> ALSA PCM playback device -> I2S amplifier -> speaker
- Audio format: 16-bit signed LE, 16kHz mono (for voice), 44.1kHz stereo (for music)
- The PCM buffer is the handoff point between the audio module and voice/music modules

### Voice

- Opens a WebSocket connection to the configured AI provider endpoint
- Streams raw or encoded audio from the mic capture buffer
- Receives text and/or audio responses
- Routes responses to the display module (text) or audio module (TTS playback)
- Handles connection lifecycle: connect, authenticate, stream, disconnect, reconnect on failure

### Music

- **Local playback:** Decodes audio files (MP3, FLAC, OGG) from local storage, feeds PCM to ALSA playback
- **librespot integration:** Spotify Connect client, receives audio stream, feeds PCM to ALSA playback
- Playback controls (play, pause, skip, volume) are routed from button events through the dispatcher
- Metadata (track name, artist, album art) is sent to the display module

### Books

- Parses ePub files into structured content (chapters, text, metadata)
- Paginates text to fit the e-ink display dimensions
- Renders each page to the display framebuffer
- Tracks reading position, bookmarks, progress
- Page turns triggered by button events

### Notifications

```
Phone --> BLE --> ESP32 --> UART --> T113 --> Display
```

- ESP32 handles BLE connection to phone (ANCS for iOS, GATT notifications for Android)
- Notification data is serialized and sent over UART to the T113
- T113 parses the notification, formats it, sends it to the display module
- Notification history stored locally

### Camera

- USB UVC camera accessed via V4L2 (`/dev/video0`)
- Captures single frames on demand (not continuous streaming)
- Captured frames can be sent to the cloud for visual analysis or stored locally
- Resolution and format negotiated via V4L2 ioctls

## ESP32 UART Protocol

The T113 and ESP32 communicate over UART using a simple binary protocol.

### Frame Format

```
+------+------+----------+------------+---------+------+
| 0xAA | 0x55 | cmd_byte | length_u16 | payload | crc8 |
+------+------+----------+------------+---------+------+
  1B     1B      1B         2B (LE)     N bytes    1B
```

- **Header:** `0xAA 0x55` (magic bytes, used for framing/sync)
- **cmd_byte:** Command identifier
- **length_u16:** Payload length in bytes, little-endian
- **payload:** Variable-length data, format depends on command
- **crc8:** CRC-8 over cmd_byte + length + payload

### Commands

| Command | Byte | Direction | Description |
|---------|------|-----------|-------------|
| `HEARTBEAT` | `0x01` | T113 -> ESP32 | Keep-alive ping |
| `WIFI_SCAN` | `0x10` | T113 -> ESP32 | Request available WiFi networks |
| `WIFI_CONNECT` | `0x11` | T113 -> ESP32 | Connect to WiFi (SSID + password in payload) |
| `WIFI_STATUS` | `0x12` | T113 -> ESP32 | Request current WiFi connection status |
| `BLE_NOTIFY` | `0x20` | ESP32 -> T113 | Incoming BLE notification from phone |
| `BLE_STATUS` | `0x21` | T113 <-> ESP32 | BLE connection status query/update |

### Responses

Responses mirror the command byte with the **high bit set** (OR with `0x80`). So a response to `WIFI_SCAN` (`0x10`) is `0x90`.

| Response | Byte | Description |
|----------|------|-------------|
| `HEARTBEAT_ACK` | `0x81` | ESP32 is alive |
| `WIFI_SCAN_RESULT` | `0x90` | List of discovered networks |
| `WIFI_CONNECT_RESULT` | `0x91` | Connection success/failure |
| `WIFI_STATUS_RESULT` | `0x92` | Current connection info |
| `BLE_NOTIFY_ACK` | `0xA0` | T113 received the notification |
| `BLE_STATUS_RESULT` | `0xA1` | BLE connection state |

### UART Parameters

- Baud rate: 115200
- Data bits: 8
- Parity: None
- Stop bits: 1
- Flow control: None
- Device: `/dev/ttyS1` (on T113 side)

## Configuration

Configuration lives in a TOML file at `/etc/rune/config.toml`.

```toml
[ai]
provider = "https://api.example.com/v1/voice"
api_key = "sk-..."

[wifi]
ssid = "MyNetwork"
password = "hunter2"

[display]
full_refresh_interval = 30  # full refresh every N partial refreshes
orientation = "portrait"

[audio]
volume = 70  # 0-100

[logging]
level = "warn"  # error, warn, info, debug
```

On first boot with no config file, the device enters a setup mode where it creates a WiFi AP and serves a configuration page.

## Logging

- Structured logging to `/var/log/rune.log`
- Integrated with **journald** (accessible via `journalctl -u rune`)
- Log rotation handled by journald or logrotate

### Log Levels

| Level | When to use | Default on? |
|-------|-------------|-------------|
| `error` | Something is broken and needs attention | Always |
| `warn` | Something is wrong but the system can continue | Yes (default) |
| `info` | General operational information | No |
| `debug` | Detailed internal state, only useful during development | No |

In development, set `level = "debug"` in the config. In production, leave it at `"warn"`.

## Memory Layout

With 128MB RAM on the T113-S4, memory budget is tight:

- Kernel + drivers: ~20MB
- Rune binary + heap: ~30MB target
- Display framebuffer: ~200KB (depending on bit depth)
- Audio buffers: ~1MB
- File I/O / ePub content: ~10MB
- Headroom: ~67MB

No swap. If we run out of memory, the OOM killer takes us out. Keep allocations predictable and bounded.
