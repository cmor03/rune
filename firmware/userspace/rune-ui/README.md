# Rune UI Renderer

This is the first portable Rune UI implementation. It is intentionally not a
web app. It renders Rune screens into a fixed 320x480 grayscale framebuffer so
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

Write a single frame to a Raspberry Pi LCD exposed as `/dev/fb0`:

```bash
cargo run --bin rune-ui-demo -- \
  --screen home \
  --backend fbdev \
  --fb /dev/fb0 \
  --format rgb565
```

Use `--format xrgb8888` if the framebuffer is configured for 32-bit pixels.
If the image appears split into repeated horizontal/side-by-side bands, Linux is
probably exposing the LCD as landscape `480x320` while Rune is rendering
portrait `320x480`. Try rotating the output:

```bash
cargo run --bin rune-ui-demo -- \
  --screen home \
  --backend fbdev \
  --fb /dev/fb0 \
  --format rgb565 \
  --rotate 90
```

If that is upside down or mirrored, try `--rotate 270`.

If the orientation is correct but the image appears duplicated or stacked, the
framebuffer stride or virtual size is different from the visible image. Inspect:

```bash
cat /sys/class/graphics/fb0/virtual_size
cat /sys/class/graphics/fb0/stride 2>/dev/null || true
cat /sys/class/graphics/fb0/bits_per_pixel
```

Then override geometry if needed:

```bash
cargo run --bin rune-ui-demo -- \
  --screen home \
  --backend fbdev \
  --fb /dev/fb0 \
  --format rgb565 \
  --rotate 90 \
  --fb-width 480 \
  --fb-height 320 \
  --stride 960
```

If `/dev/fb0` does not exist, probe the Pi display stack first:

```bash
./scripts/pi-display-probe.sh
```

The fbdev backend only works after the LCD has been registered by Linux as a
framebuffer device.

## Command Loop

Run the UI continuously:

```bash
cargo run --bin rune-ui-demo -- \
  --serve \
  --backend fbdev \
  --fb /dev/fb0 \
  --format rgb565 \
  --rotate 90
```

Send wheel/button commands from another shell:

```bash
cargo run --bin rune-ui-demo -- --send UP
cargo run --bin rune-ui-demo -- --send DOWN
cargo run --bin rune-ui-demo -- --send PRESS
cargo run --bin rune-ui-demo -- --send PHOLD
```

`UP` and `DOWN` move launcher focus, `PRESS` opens or returns, and `PHOLD`
jumps to the voice screen.

## Boot Service

Install the prototype service on the Pi:

```bash
sudo install -m 0755 target/release/rune-ui-demo /usr/local/bin/rune-ui
sudo cp systemd/rune-ui.service /etc/systemd/system/rune-ui.service
sudo systemctl daemon-reload
sudo systemctl enable --now rune-ui.service
```

View logs:

```bash
journalctl -u rune-ui.service -f
```

Animation timing notes are in [ANIMATION.md](ANIMATION.md).

## Porting Plan

The UI code should stay split like this:

- `canvas`: fixed framebuffer and primitives
- `screens`: Rune screen layout and visual states
- `animation`: frame timing and sprite movement
- `display`: output sinks and future hardware backends

The next backend to improve for the Raspberry Pi LCD is likely one of:

- `fbdev`: detect dimensions, line length, and pixel format instead of requiring flags
- `drm`: draw the 320x480 canvas to a KMS plane
- `lcd-spi`: direct SPI LCD driver only if the LCD does not expose framebuffer
  or DRM devices

Do not put Pi GPIO numbers or LCD controller commands inside `screens`. They
belong in backend-specific modules.
