# Userspace Application

The main Rune application lives here. Written in **Rust** (preferred) or C.

This is the single binary that provides all user-facing functionality:

- Voice queries (WebSocket streaming to cloud AI)
- ePub reading (parsing, pagination, rendering)
- Music playback (local files, librespot/Spotify Connect)
- Notification display (BLE relay from phone via ESP32)
- Photo capture (USB UVC camera via V4L2)

## Design

One process, multiple internal modules, async event loop. No separate daemons or services. See [firmware/architecture.md](../architecture.md) for module design, event routing, and protocol details.

## Current status

Not yet populated. Module interfaces are being defined.
