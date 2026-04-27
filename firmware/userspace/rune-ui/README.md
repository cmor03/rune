# Rune UI Renderer

This is the first portable Rune UI implementation. It is intentionally not a
web app. It renders Rune screens into a fixed 480x280 grayscale framebuffer so
the same UI can run in three places:

- On a development machine as image files
- On a Raspberry Pi LCD through a future `fbdev`/`drm` backend
- On Rune hardware through a future e-ink SPI backend

The renderer is dependency-free for now. That keeps the UI easy to cross-compile
and makes every pixel path explicit.

The visual references from the supplied mock live in `reference/`. They are not
runtime dependencies; they are there to keep the implemented screens pointed at
the intended tone.

## Screens

Implemented in this first pass:

- `sleep`
- `wake`
- `home`
- `voice`
- `reader`
- `music`
- `notifications`
- `camera`

## Render Frames

From this directory:

```bash
cargo run --bin rune-ui-demo -- --screen home --out target/ui-frames
```

Render an animation sequence:

```bash
cargo run --bin rune-ui-demo -- --screen wake --frames 24 --out target/wake
cargo run --bin rune-ui-demo -- --screen voice --frames 24 --out target/voice
cargo run --bin rune-ui-demo -- --screen camera --frames 24 --out target/camera
```

The output files are binary PGM images. Most image viewers can open them. On
macOS, Preview can open `.pgm`; ImageMagick can convert them:

```bash
magick target/wake/wake-000.pgm wake-000.png
```

Write a single frame to a Raspberry Pi LCD exposed as `/dev/fb1`:

```bash
cargo run --bin rune-ui-demo -- \
  --screen home \
  --backend fbdev \
  --fb /dev/fb1 \
  --format rgb565
```

Use `--format xrgb8888` if the framebuffer is configured for 32-bit pixels.

If `/dev/fb1` does not exist, probe the Pi display stack first:

```bash
./scripts/pi-display-probe.sh
```

The fbdev backend only works after the LCD has been registered by Linux as a
framebuffer device.

Animation timing notes are in [ANIMATION.md](ANIMATION.md).

## Porting Plan

The UI code should stay split like this:

- `canvas`: fixed framebuffer and primitives
- `screens`: Rune screen layout and visual states
- `animation`: frame timing and sprite movement
- `display`: output sinks and future hardware backends

The next backend to improve for the Raspberry Pi LCD is likely one of:

- `fbdev`: detect dimensions, line length, and pixel format instead of requiring flags
- `drm`: draw the 480x280 canvas to a KMS plane
- `lcd-spi`: direct SPI LCD driver only if the LCD does not expose framebuffer
  or DRM devices

Do not put Pi GPIO numbers or LCD controller commands inside `screens`. They
belong in backend-specific modules.
