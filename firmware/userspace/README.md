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

For early UI work before Rune dev hardware is ready, use the [Raspberry Pi UI Setup](../../docs/development/raspberry-pi-ui-setup.md). It keeps the UI on Linux interfaces that can move from Pi to T113: framebuffer/DRM, spidev, evdev/GPIO, ALSA, and V4L2.

The first portable UI renderer lives in [rune-ui](rune-ui/). It renders a fixed
480x280 grayscale framebuffer and includes initial screens plus wake, voice, and
camera animation states.

## Current status

Early UI renderer scaffold is present. Hardware display backends are still being
defined.
