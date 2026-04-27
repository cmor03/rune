# Raspberry Pi UI Setup

These instructions use a Raspberry Pi 5 or Raspberry Pi 4 as a stand-in for
Rune's final Linux hardware. The goal is not to turn the Pi into Rune. The goal
is to build a portable UI development loop that exercises the same Linux
interfaces Rune will use on the T113 board: framebuffer/DRM for pixels,
evdev/GPIO for input, ALSA for audio, V4L2 for camera capture, and ordinary
files for local state.

If you have a Pi 4 already, use it. If you are starting from a blank Pi 5, this
guide takes you from empty SD card to a headless UI mock target.

---

## Setup Goals

By the end of this setup you should have:

1. A Raspberry Pi running a minimal Linux userspace without a desktop.
2. SSH access from your development machine.
3. SPI, I2C, GPIO, ALSA, and V4L2 enabled.
4. A place to run Rune UI mock binaries under `systemd`.
5. A workflow that can later move from Pi to MangoPi/T113 with minimal changes.

The important habit is this: write Rune UI code against small interfaces, not
against Raspberry Pi APIs. The Pi is a temporary Linux target. Linux is the
contract.

---

## Hardware

Minimum:

- Raspberry Pi 5 or Raspberry Pi 4
- 32GB microSD card, A1/A2 rated if possible
- USB-C power supply rated for your Pi
- Ethernet cable or WiFi network
- Development machine with SSH

Useful for UI bring-up:

- GPIO-header LCD or HAT already attached to the Pi
- Small HDMI monitor for first visual tests
- USB keyboard for emergency local login
- USB UVC camera
- USB speaker, I2S amp, or HDMI audio
- Rotary encoder and push buttons
- Logic analyzer for SPI/GPIO debugging

The Pi 5 is faster and has better I/O headroom. The Pi 4 is still completely
adequate for UI mocking because Rune's real display is only 320x480 and should
not be animation-heavy.

---

## Portability Rules

Follow these rules while prototyping on the Pi:

- Render into a fixed 320x480 canvas even if the Pi is connected to HDMI.
- Keep display output behind a backend interface:
  - `png` for screenshot tests on a dev machine
  - `drm` or `fbdev` for Pi HDMI/fullscreen preview
  - `fbdev` or `drm` for a Pi header-connected LCD
  - direct `spidev` only if the LCD has no usable kernel framebuffer/DRM driver
  - final T113 e-ink backend later
- Read input through evdev or a small GPIO adapter layer.
- Use ALSA directly for audio capture/playback.
- Use V4L2 directly for camera capture.
- Do not depend on X11, Wayland, SDL, Chromium, Electron, or a desktop session.
- Avoid Pi-only GPIO libraries in the main application. If you use one for a
  hardware test, isolate it in a tiny adapter.

Good architecture for the UI layer:

```text
Rune app logic
     |
     v
320x480 UI scene / framebuffer
     |
     +--> PNG backend for screenshots
     +--> DRM/fbdev backend for Pi HDMI
     +--> fbdev/DRM backend for Pi header LCD
     +--> SPI e-ink backend for T113 hardware
```

---

## Part 1: Flash Raspberry Pi OS Lite

Use **Raspberry Pi OS Lite 64-bit** unless you have a reason to test 32-bit.
This gives you the Raspberry Pi kernel and firmware support without a desktop
environment.

On macOS, install Raspberry Pi Imager:

```bash
brew install --cask raspberry-pi-imager
```

Open Raspberry Pi Imager:

1. Choose your Pi model.
2. Choose **Raspberry Pi OS Lite (64-bit)**.
3. Choose the microSD card.
4. Open OS customization.
5. Set hostname to `rune-pi`.
6. Enable SSH.
7. Add your SSH public key.
8. Configure WiFi if you are not using Ethernet.
9. Set locale/timezone.
10. Write the image.

If you prefer command-line imaging, download the Lite image from Raspberry Pi,
then flash it with `rpi-imager`, `balenaEtcher`, or `dd`. Be careful with `dd`;
one wrong device path can erase a disk.

---

## Part 2: First Boot and SSH

Insert the SD card, connect Ethernet if using it, and power the Pi.

Find it on the network:

```bash
ping rune-pi.local
```

SSH in:

```bash
ssh pi@rune-pi.local
```

If you created a custom user in Imager, replace `pi` with that username.

Update the base system:

```bash
sudo apt update
sudo apt full-upgrade -y
sudo reboot
```

Reconnect after reboot:

```bash
ssh pi@rune-pi.local
```

Check the kernel and model:

```bash
uname -a
cat /proc/device-tree/model
```

---

## Part 3: Keep Linux Lightweight

Raspberry Pi OS Lite already avoids the desktop stack. Do not install a desktop
environment for Rune UI work. The final device will not have one.

Install only the tools needed for bring-up:

```bash
sudo apt install -y \
  git \
  build-essential \
  pkg-config \
  libasound2-dev \
  libudev-dev \
  libevdev-dev \
  libdrm-dev \
  libgbm-dev \
  v4l-utils \
  alsa-utils \
  i2c-tools \
  gpiod \
  libgpiod-dev \
  python3 \
  python3-pip \
  picocom \
  rsync
```

Optional tools:

```bash
sudo apt install -y \
  strace \
  linux-perf \
  htop \
  jq \
  imagemagick \
  fbset
```

The stock Raspberry Pi kernel is fine for this setup. Building a custom kernel
is not useful until you need a specific display driver or low-level timing
change. For now, keep the kernel boring and make the UI portable.

### Make It Boot Like an Appliance

Do this before trying to rebuild or strip the kernel. The big wins come from
removing desktop/session services and starting only the Rune UI service.

Set console boot:

```bash
sudo systemctl set-default multi-user.target
```

Disable services that are useful on a general Pi but not needed for the UI
prototype:

```bash
sudo systemctl disable --now bluetooth.service 2>/dev/null || true
sudo systemctl disable --now hciuart.service 2>/dev/null || true
sudo systemctl disable --now avahi-daemon.service 2>/dev/null || true
sudo systemctl disable --now triggerhappy.service 2>/dev/null || true
```

Keep SSH enabled while developing:

```bash
sudo systemctl enable --now ssh
```

Optional boot quieting, once SSH works reliably:

```bash
sudo cp /boot/firmware/cmdline.txt /boot/firmware/cmdline.txt.rune-backup
sudo sed -i 's/ quiet//g' /boot/firmware/cmdline.txt
sudo sed -i 's/$/ quiet loglevel=3 vt.global_cursor_default=0/' /boot/firmware/cmdline.txt
```

Do not remove networking yet. You need SSH for iteration and command injection.
Once the UI is stable, Buildroot is the right place to produce a truly minimal
root filesystem for the final board.

---

## Part 4: Enable Interfaces

Open Raspberry Pi configuration:

```bash
sudo raspi-config
```

Enable:

- SPI
- I2C
- SSH
- Camera support if your OS image exposes it there

For a headless device, keep boot mode as console autologin disabled. Rune should
start as a service, not as an interactive shell side effect.

Reboot:

```bash
sudo reboot
```

After reconnecting, verify devices:

```bash
ls -l /dev/spidev*
ls -l /dev/i2c-*
gpioinfo
aplay -l
arecord -l
v4l2-ctl --list-devices
```

Expected early results:

- `/dev/spidev0.0` or similar appears after SPI is enabled.
- `/dev/i2c-1` appears after I2C is enabled.
- `gpioinfo` lists one or more GPIO chips.
- Audio devices depend on HDMI, USB audio, or I2S configuration.
- V4L2 devices appear only after a USB camera is connected.

---

## Part 5: Clone Rune on the Pi

For UI iteration you can either build on the Pi or cross-compile elsewhere and
copy artifacts over. Start by cloning the repo directly so the Pi has docs,
assets, and scripts available.

```bash
mkdir -p ~/src
cd ~/src
git clone [GitHub URL TBD] rune
cd rune
```

If the repo is not public yet, copy it from your development machine:

```bash
rsync -az --delete \
  --exclude .git \
  /path/to/rune/ \
  pi@rune-pi.local:/home/pi/src/rune/
```

---

## Part 6: Install Rust for Native UI Prototypes

The final T113 target will be cross-compiled, but native Pi builds are useful
for fast UI prototyping.

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Log out and back in, or source Cargo's environment:

```bash
. "$HOME/.cargo/env"
```

Install normal development components:

```bash
rustup component add rustfmt clippy
```

Check:

```bash
rustc --version
cargo --version
```

The first native Pi build starts with the portable UI renderer:

```bash
cd ~/src/rune/firmware/userspace/rune-ui
cargo build
```

For release testing:

```bash
cargo build --release
```

Render the current portable UI mock to image frames:

```bash
cargo run --bin rune-ui-demo -- --screen home --out target/ui-home
cargo run --bin rune-ui-demo -- --screen wake --frames 24 --out target/ui-wake
```

If your LCD appears as `/dev/fb0`, try writing one frame directly:

```bash
cargo run --bin rune-ui-demo -- \
  --screen home \
  --backend fbdev \
  --fb /dev/fb0 \
  --format rgb565
```

If the output appears as repeated side-by-side slices, the framebuffer is
landscape while Rune is rendering portrait. Rotate the framebuffer output:

```bash
cargo run --bin rune-ui-demo -- \
  --screen home \
  --backend fbdev \
  --fb /dev/fb0 \
  --format rgb565 \
  --rotate 90
```

If the image is upside down or mirrored, use `--rotate 270`.

Do not optimize the UI for the Pi's extra CPU. The Pi is a convenience target;
the production device is a small ARM Linux system with a slow display.

---

## Part 7: UI Backend Strategy

Before targeting the attached LCD, build a mock backend that saves frames as PGM
image files. This makes visual tests possible on any machine without pulling in
an image encoding dependency.

Current renderer behavior:

```bash
cargo run --bin rune-ui-demo -- --screen home --out /tmp/rune-ui
```

Future Pi fullscreen DRM backend behavior should look like this:

```bash
rune-ui-demo --backend drm
```

For a GPIO-header LCD that appears as a Linux framebuffer, the current direct
test path is:

```bash
rune-ui-demo --backend fbdev --fb /dev/fb0
```

For a GPIO-header LCD that appears as a DRM device, the future command should
look like this:

```bash
# or
rune-ui-demo --backend drm --connector auto
```

These command names are illustrative. The important part is that the app takes
backend configuration from flags or a config file, not from hardcoded board
assumptions.

Recommended module split:

```text
firmware/userspace/
  src/
    ui/              # Layout, screens, typography, 320x480 canvas
    input/           # Buttons, encoder, keyboard test adapter
    display/
      mod.rs         # DisplayBackend trait
      png.rs         # Screenshot backend
      drm.rs         # HDMI/fullscreen backend
      fbdev.rs       # Linux framebuffer backend for small attached screens
      lcd_spi.rs     # Optional direct SPI LCD backend if no kernel driver exists
    audio/           # ALSA capture/playback
    camera/          # V4L2 capture
```

The UI should not know whether it is drawing to PNG, HDMI, the Pi LCD, or the
final e-ink panel. It should produce a frame and request a refresh.

---

## Part 8: HDMI Preview Without a Desktop

For the first UI mock, HDMI is the easiest display target. You can run without a
desktop by drawing directly through DRM/KMS or fbdev.

Check DRM devices:

```bash
ls -l /dev/dri/
modetest -c 2>/dev/null || true
```

If `modetest` is missing:

```bash
sudo apt install -y libdrm-tests
```

For a very simple preview path, generate a PNG on the Pi and inspect it from
your development machine:

```bash
scp pi@rune-pi.local:/tmp/rune-frame.png .
open rune-frame.png
```

For fullscreen preview, the eventual app should open DRM directly and scale the
320x480 Rune canvas to the connected HDMI mode. Scaling is only for preview.
Layout decisions should still be made at 320x480.

---

## Part 9: Direct Header LCD

If your screen is connected directly to the Pi's GPIO header, identify what kind
of screen it is before writing any Rune code for it. The connector does not tell
you the software path; the controller and transport do.

If `rune-ui-demo --backend fbdev --fb /dev/fb0 ...` returns `No such file or
directory`, the renderer is working but Linux has not exposed `/dev/fb0`. That
usually means one of three things:

- The LCD is exposed as a different framebuffer, often `/dev/fb1`.
- The LCD is exposed through DRM/KMS under `/dev/dri/` instead of fbdev.
- The LCD driver/overlay is not loaded, so Linux does not know the screen exists.

Common possibilities:

| Screen type | Typical Linux surface | Rune backend to use |
|-------------|-----------------------|---------------------|
| SPI TFT HAT | `/dev/fb0` through a kernel overlay, or `/dev/spidev0.0` if driven directly | `fbdev` first, direct `spidev` only if needed |
| DPI parallel display | `/dev/dri/card*` or `/dev/fb0` after a device-tree overlay | `drm` or `fbdev` |
| DSI ribbon display | `/dev/dri/card*` | `drm` |

Find the display devices Linux already exposes:

```bash
cd ~/src/rune/firmware/userspace/rune-ui
./scripts/pi-display-probe.sh
```

Or run the underlying checks manually:

```bash
ls -l /dev/fb* 2>/dev/null || echo "no fbdev devices"
ls -l /dev/dri/ 2>/dev/null || echo "no DRM devices"
ls -l /dev/spidev* 2>/dev/null || echo "no spidev devices"
for fb in /sys/class/graphics/fb*; do
  test -e "$fb" || continue
  echo "$fb: $(cat "$fb/name" 2>/dev/null || true)"
  cat "$fb/virtual_size" 2>/dev/null || true
  cat "$fb/bits_per_pixel" 2>/dev/null || true
done
dmesg | grep -Ei 'drm|fb|spi|display|panel' | tail -80
```

Check the active Pi overlays:

```bash
grep -nE '^(dtparam|dtoverlay)' /boot/firmware/config.txt
```

On older images the file may be `/boot/config.txt` instead:

```bash
test -f /boot/firmware/config.txt && echo /boot/firmware/config.txt || echo /boot/config.txt
```

Do not guess the overlay. Look up the screen's exact model and controller first
from the silkscreen, order page, or vendor repo. Useful LCD controller
identifiers look like `ILI9341`, `ST7789`, `GC9A01`, `HX8357`, `ILI9486`, or a
Waveshare SKU. Two LCDs can use the same 40-pin Pi header and require completely
different drivers.

### Inland TFT3.5 Touch Shield

The Inland TFT3.5 Touch Shield sold for Raspberry Pi is a 3.5" 480x320 SPI TFT
with a resistive touch overlay. It is commonly compatible with the ILI9486 LCD
and XPT2046 touch-controller family.

If the screen lights up white but `ls -l /dev/fb*` shows no framebuffer device,
the Pi sees SPI but has not loaded an LCD driver. Start with the in-kernel
overlay path because it is reversible and does not run a vendor install script:

```bash
sudo cp /boot/firmware/config.txt /boot/firmware/config.txt.rune-backup
sudo tee -a /boot/firmware/config.txt >/dev/null <<'EOF'

# Inland TFT3.5 / PiScreen-style SPI LCD test
dtoverlay=piscreen,speed=18000000,drm,rotate=270
EOF
sudo reboot
```

After reboot:

```bash
ls -l /dev/fb* 2>/dev/null || echo "no fbdev devices"
ls -l /dev/dri/ 2>/dev/null || echo "no DRM devices"
dmesg | grep -Ei 'piscreen|ili|st77|drm|fb|spi|panel|lcd' | tail -120
```

If a framebuffer appears, try Rune:

```bash
cd ~/src/rune/firmware/userspace/rune-ui
cargo run --bin rune-ui-demo -- \
  --screen home \
  --backend fbdev \
  --fb /dev/fb0 \
  --format rgb565
```

The Rune UI mock is portrait `320x480`. The Inland panel is physically
`480x320`, so the overlay must rotate the LCD into portrait. If the image is
sideways or split into repeated horizontal bands, edit the overlay line and try
the other portrait rotation:

```text
dtoverlay=piscreen,speed=18000000,drm,rotate=90
```

Then reboot and check:

```bash
cat /sys/class/graphics/fb0/virtual_size 2>/dev/null || true
```

The value should be `320,480` for the current Rune LCD prototype path. If it is
`480,320`, Linux is still exposing the LCD in landscape.

If no framebuffer appears but DRM reports an active connector, the next Rune
step is a DRM backend rather than fbdev.

If the `piscreen` overlay does not work, restore the backup before trying a
vendor driver:

```bash
sudo cp /boot/firmware/config.txt.rune-backup /boot/firmware/config.txt
sudo reboot
```

The vendor-supported path for this display family is usually the GoodTFT
`LCD-show` scripts:

```bash
cd ~
sudo rm -rf LCD-show
git clone https://github.com/goodtft/LCD-show.git
chmod -R 755 LCD-show
cd LCD-show
sudo ./MHS35-show
```

Treat this as a fallback. Those scripts can rewrite boot configuration and may
disable or reroute HDMI output. Keep SSH access working and keep the
`config.txt.rune-backup` file so you can recover quickly.

If the screen exposes a framebuffer such as `/dev/fb0`, test it without Rune:

```bash
sudo apt install -y fbi
convert -size 320x480 xc:white \
  -fill black -pointsize 28 -gravity center \
  -annotate 0 'Rune framebuffer test' /tmp/rune-fb-test.png
sudo fbi -T 1 -d /dev/fb0 --noverbose /tmp/rune-fb-test.png
```

If you only see `/dev/fb0`, inspect it before writing to it:

```bash
cat /sys/class/graphics/fb0/name
cat /sys/class/graphics/fb0/virtual_size 2>/dev/null || true
cat /sys/class/graphics/fb0/bits_per_pixel 2>/dev/null || true
```

On a headless Pi with a header LCD, `/dev/fb0` might be the LCD. On a Pi with
HDMI active, `/dev/fb0` may be HDMI instead. The `name`, size, and what changes
on the physical screen will tell you which one you have.

Try Rune against the framebuffer that actually exists:

```bash
cargo run --bin rune-ui-demo -- \
  --screen home \
  --backend fbdev \
  --fb /dev/fb0 \
  --format rgb565
```

If the command succeeds but the image is garbled, the framebuffer probably uses
a different pixel format. Try:

```bash
cargo run --bin rune-ui-demo -- \
  --screen home \
  --backend fbdev \
  --fb /dev/fb0 \
  --format xrgb8888
```

If `fbi` does not support your framebuffer cleanly, use the vendor sample only
long enough to prove the wiring. Then come back to a Linux backend.

For a TFT-style screen, set Rune's config to the attached framebuffer:

```toml
[display]
width = 480
height = 280
backend = "fbdev"
device = "/dev/fb0"
scale = "fit"
rotate = 0
```

If there is no framebuffer and the LCD must be driven directly over SPI, keep
that implementation behind a separate backend:

```toml
[display]
width = 480
height = 280
backend = "lcd-spi"
spi = "/dev/spidev0.0"
dc_gpio = 25
rst_gpio = 17
backlight_gpio = 18
```

If your mock UI already exists, import it at the UI layer, not the display
driver layer. The first conversion target should be a 320x480 PNG screenshot.
Once the screen states look right as PNGs, route the same frame through the Pi
screen backend.

What to capture about your current screen:

- Exact product name and vendor link, if available
- Controller chip or Waveshare SKU
- Whether it is SPI TFT, DPI LCD, DSI LCD, HDMI, or another panel type
- Current `/boot/firmware/config.txt` display-related lines
- Output of `ls -l /dev/fb* /dev/dri/* /dev/spidev*`
- A photo of the back of the screen showing labels and jumpers

Those details decide whether Rune should talk to the screen through `fbdev`,
`drm`, or direct `spidev`. For your current regular LCD, start with `fbdev` or
`drm`; direct SPI is the fallback path.

---

## Part 10: Input Mocking

Start with a keyboard because it is fast:

| Rune action | Keyboard key |
|-------------|--------------|
| Up | Arrow up |
| Down | Arrow down |
| Select | Enter |
| Back | Escape |
| Action / voice | Space |

Then wire real controls to GPIO.

Example rotary encoder and buttons:

| Control | Pi GPIO | Notes |
|---------|---------|-------|
| Encoder A | GPIO5 | Pull-up enabled |
| Encoder B | GPIO6 | Pull-up enabled |
| Encoder press | GPIO13 | Active low |
| Back button | GPIO19 | Active low |
| Action button | GPIO26 | Active low |

Use `gpiomon` to test:

```bash
gpiomon gpiochip0 5 6 13 19 26
```

Use evdev for the application-facing input path if possible. A tiny adapter can
convert GPIO edges into Rune input events. The UI should receive semantic events
like `Next`, `Previous`, `Select`, `Back`, and `Action`, not raw pin numbers.

---

## Part 11: Audio and Camera Sanity Checks

Audio devices:

```bash
aplay -l
arecord -l
speaker-test -t sine -f 440 -c 1
```

Record a voice-query-shaped sample:

```bash
arecord -D default -f S16_LE -r 16000 -c 1 -d 5 /tmp/rune-voice.wav
aplay /tmp/rune-voice.wav
```

Camera:

```bash
v4l2-ctl --list-devices
v4l2-ctl --device /dev/video0 --all
v4l2-ctl --device /dev/video0 \
  --set-fmt-video=width=640,height=480,pixelformat=MJPG \
  --stream-mmap \
  --stream-count=1 \
  --stream-to=/tmp/rune-camera.jpg
```

The UI mock does not need full voice or camera features on day one. It only
needs enough plumbing that screens can be tested honestly.

---

## Part 12: Run the UI as a Service

Create a system user:

```bash
sudo useradd --system --create-home --groups audio,video,gpio,spi,i2c rune
```

Create a config directory:

```bash
sudo mkdir -p /etc/rune
sudo tee /etc/rune/rune.toml >/dev/null <<'EOF'
[display]
width = 480
height = 280
backend = "png"

[input]
backend = "keyboard"

[paths]
data_dir = "/var/lib/rune"
EOF
```

Create data directory:

```bash
sudo mkdir -p /var/lib/rune
```

Example service file:

```bash
sudo tee /etc/systemd/system/rune-ui.service >/dev/null <<'EOF'
[Unit]
Description=Rune UI mock
After=network-online.target
Wants=network-online.target

[Service]
Environment=RUNE_CONFIG=/etc/rune/rune.toml
ExecStart=/usr/local/bin/rune-ui --serve --backend fbdev --fb /dev/fb0 --format rgb565 --rotate 90 --socket /tmp/rune-ui.sock
Restart=on-failure
RestartSec=2

[Install]
WantedBy=multi-user.target
EOF
```

Install the current UI mock binary as the service entry point:

```bash
sudo install -m 0755 target/release/rune-ui-demo /usr/local/bin/rune-ui
sudo systemctl daemon-reload
sudo systemctl enable --now rune-ui.service
```

Check logs:

```bash
journalctl -u rune-ui.service -f
```

During early bring-up, this service runs as root so it can open framebuffer and
device files without fighting Linux permissions. Once the UI path is stable, add
a dedicated `rune` user and narrow device access.

Send simulated wheel/button input from the shell:

```bash
rune-ui --send UP --socket /tmp/rune-ui.sock
rune-ui --send DOWN --socket /tmp/rune-ui.sock
rune-ui --send PRESS --socket /tmp/rune-ui.sock
rune-ui --send PHOLD --socket /tmp/rune-ui.sock
```

These commands map to the intended physical controls:

| Command | Meaning |
|---------|---------|
| `UP` | Scroll wheel up |
| `DOWN` | Scroll wheel down |
| `PRESS` | Short press |
| `PHOLD` | Press and hold |

---

## Part 13: Fast Iteration Loop

From your development machine:

```bash
cargo build --release
scp target/release/rune-ui-demo pi@rune-pi.local:/tmp/rune-ui
ssh pi@rune-pi.local \
  'sudo install -m 0755 /tmp/rune-ui /usr/local/bin/rune-ui && sudo systemctl restart rune-ui'
```

For asset and config changes:

```bash
rsync -az assets/ pi@rune-pi.local:/var/lib/rune/assets/
ssh pi@rune-pi.local 'sudo systemctl restart rune-ui'
```

For development logs:

```bash
ssh pi@rune-pi.local 'journalctl -u rune-ui.service -n 100 --no-pager'
```

This is the same mental loop as the T113 prototype: build, copy, restart, look
at the display, repeat.

---

## Part 14: What Ports Cleanly to T113

Should port cleanly:

- 320x480 UI layout
- Screen state machine
- Button/encoder semantics
- PNG screenshot tests
- ALSA voice/music plumbing
- V4L2 camera capture
- TOML configuration
- `systemd` or init-script service shape, with small changes if Buildroot uses
  BusyBox init instead

Will need board-specific work:

- SPI bus path and maximum clock speed
- GPIO chip names and line numbers
- LCD/e-ink display backend and refresh behavior
- I2S device-tree configuration
- Power management and suspend/resume
- ESP32 UART device path

Do not wait for the final board to make UI decisions. Do wait for the final
board before claiming timing, battery life, or display refresh performance.

---

## Setup Checklist

- [ ] Pi boots Raspberry Pi OS Lite.
- [ ] SSH works by hostname.
- [ ] SPI and I2C are enabled.
- [ ] `gpioinfo` works.
- [ ] Audio input/output is visible through ALSA.
- [ ] USB camera appears through V4L2.
- [ ] Rune repo is present on the Pi.
- [ ] UI can render a 320x480 PNG frame.
- [ ] Direct header-connected LCD is identified as `fbdev`, `drm`, or direct `spidev`.
- [ ] UI can run under a service manager.
- [ ] Keyboard input can drive the UI state machine.
- [ ] Optional: GPIO buttons can drive the same input events.

Once these are true, the Pi is doing its job: it is a cheap, available Linux
target that keeps the UI honest while the real hardware catches up.
