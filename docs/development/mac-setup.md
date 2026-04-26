# Mac Development Setup

Setting up an Apple Silicon Mac for Rune development. The key insight: you need a Linux VM for building, but you work on macOS.

## Why a Linux VM

The T113-S4 is an ARM Cortex-A7 (armv7). Cross-compiling for it requires:

- ARM cross-compilation toolchains (`arm-linux-gnueabihf-gcc`)
- Buildroot (which generates the entire Linux system image)
- Various Linux-specific build tools

These toolchains don't run reliably on macOS, even on ARM Macs. Homebrew ARM toolchains are incomplete. Buildroot explicitly requires a Linux host. Fighting this wastes time. Use a Linux VM.

Your workflow: edit code in macOS (your editor, your shortcuts, your setup) → build inside the Linux VM → deploy to the device.

## Install OrbStack (Recommended)

OrbStack is a lightweight Linux VM manager for macOS. It's faster and uses less resources than Docker Desktop or UTM for this use case.

```bash
# Install via Homebrew
brew install orbstack

# Or download from https://orbstack.dev
```

### Alternative: UTM

If you prefer a full VM with a GUI:

```bash
brew install --cask utm
```

Download the Ubuntu 22.04 ARM64 ISO from https://ubuntu.com/download/server/arm and create a VM manually. The rest of this guide assumes OrbStack, but the commands inside the VM are the same.

## Create the Ubuntu VM

```bash
# Create an Ubuntu 22.04 VM
orb create ubuntu:22.04 rune-dev

# Enter the VM
orb shell rune-dev
```

### VM Settings

OrbStack defaults are fine for most work, but if you're doing full Buildroot builds, you may want more resources:

- **RAM**: 4GB minimum, 8GB recommended for Buildroot parallel builds
- **Disk**: 30GB minimum. Buildroot output trees get large. 50GB is more comfortable.
- **CPUs**: OrbStack shares host CPUs by default. No change needed.

### Shared Folders

OrbStack automatically mounts your macOS home directory at `/Users/<username>` inside the VM. This means your repo is already accessible:

```bash
# Inside the VM, your repo is at the same path
ls /Users/coltonmorris/Desktop/rune/repo/rune
```

No need to manually set up shared folders or syncing.

## Install Build Dependencies (Inside VM)

```bash
# Enter the VM
orb shell rune-dev

# Update packages
sudo apt update && sudo apt upgrade -y

# Core build tools
sudo apt install -y \
  build-essential \
  gcc-arm-linux-gnueabihf \
  g++-arm-linux-gnueabihf \
  device-tree-compiler \
  libncurses-dev \
  flex \
  bison \
  bc \
  libssl-dev \
  python3 \
  python3-pip \
  git \
  unzip \
  wget \
  curl \
  cpio \
  rsync \
  file \
  patch \
  cmake \
  ninja-build

# Verify the cross-compiler works
arm-linux-gnueabihf-gcc --version
# Should output something like:
# arm-linux-gnueabihf-gcc (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0
```

## Install Rust (Inside VM)

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Reload shell
source "$HOME/.cargo/env"

# Add the ARMv7 cross-compilation target
rustup target add armv7-unknown-linux-gnueabihf

# Tell cargo where the linker is for armv7
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml << 'EOF'
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF

# Verify
rustup target list --installed
# Should include: armv7-unknown-linux-gnueabihf

# Test cross-compilation
cargo init /tmp/test-cross && cd /tmp/test-cross
cargo build --target armv7-unknown-linux-gnueabihf
file target/armv7-unknown-linux-gnueabihf/debug/test-cross
# Should say: ELF 32-bit LSB pie executable, ARM, EABI5
```

## macOS Host Tools

Back on your Mac (not inside the VM):

### xfel (FEL Mode Flasher)

xfel talks to Allwinner chips in FEL mode (USB boot/recovery mode). You run this from macOS because the USB connection is to your Mac.

```bash
# Try Homebrew first
brew install xfel

# If not available, build from source
git clone https://github.com/xboot/xfel.git /tmp/xfel
cd /tmp/xfel
make
sudo cp xfel /usr/local/bin/
```

### picocom (Serial Terminal)

```bash
brew install picocom
```

### balenaEtcher (SD Card Flasher)

```bash
brew install --cask balenaetcher
```

Or download from https://etcher.balena.io. Used to write Linux images to microSD cards.

### Screen (Alternative Serial Terminal)

Already installed on macOS. Useful as a backup:

```bash
# Usage (see first-boot.md for details)
screen /dev/tty.usbserial-* 115200
```

## CH340 Serial Driver

Most USB-TTL serial adapters use the CH340 chip. On modern macOS (Ventura and later), the driver is usually built in. Check:

1. Plug in the serial adapter.
2. Open **System Information** → **USB**. You should see "USB-SERIAL CH340" or similar.
3. Check for the device:

```bash
ls /dev/tty.usbserial-*
# or
ls /dev/tty.wchusbserial-*
```

If nothing shows up:

1. Download the macOS driver from https://www.wch-ic.com/downloads/CH341SER_MAC_ZIP.html
2. Install it. You may need to allow the kernel extension in **System Settings → Privacy & Security**.
3. Reboot.
4. Check again with `ls /dev/tty.*serial*`.

If you're using a CP2102-based adapter instead of CH340, it typically works out of the box on macOS.

## SSH Into the VM

For comfortable development, SSH from your macOS terminal or editor into the VM.

### OrbStack

OrbStack makes this trivial — the VM is already accessible:

```bash
# SSH directly
ssh rune-dev@orb

# Or use the OrbStack hostname
ssh rune-dev.orb.local
```

### VS Code Remote SSH

1. Install the "Remote - SSH" extension in VS Code.
2. Connect to `rune-dev.orb.local` (OrbStack) or `localhost:2222` (UTM with port forwarding).
3. Open the repo folder inside the VM.
4. You now have full editor features (autocomplete, syntax highlighting, etc.) with build tools running natively in Linux.

### Terminal Multiplexer

Inside the VM, tmux is useful for keeping build sessions alive:

```bash
sudo apt install -y tmux

# Start a new session
tmux new -s rune

# Detach: Ctrl+B, then D
# Reattach: tmux attach -t rune
```

## Port Forwarding and Network Access

### OrbStack

OrbStack VMs are on a bridged network. The device (MangoPi) and the VM can see each other directly if they're on the same network. No port forwarding needed.

```bash
# From inside the VM, you can SCP directly to the device
scp binary root@rune.local:/usr/bin/

# From macOS, you can also reach the device
ssh root@rune.local
```

### UTM

If using UTM, set up port forwarding for SSH:

```
Host port 2222 → Guest port 22
```

For device access, either bridge the VM network adapter or use macOS as a proxy.

## Directory Structure Recommendation

```
~/Desktop/rune/repo/rune/     # The repo (accessible from both macOS and VM)
├── firmware/                   # Rust firmware code (edit on macOS, build in VM)
├── esp32/                      # ESP32 firmware (can build on macOS with esp-idf)
├── buildroot/                  # Buildroot external tree (build in VM only)
└── docs/                       # Documentation (edit on macOS)
```

## Quick Reference

| Task | Where | Command |
|------|-------|---------|
| Edit code | macOS | Your editor of choice |
| Cross-compile Rust | Linux VM | `cargo build --target armv7-unknown-linux-gnueabihf` |
| Build full system image | Linux VM | `make` in Buildroot |
| Flash SD card | macOS | balenaEtcher |
| Serial console | macOS | `picocom -b 115200 /dev/tty.usbserial-*` |
| FEL mode recovery | macOS | `xfel` |
| Deploy binary to device | Linux VM or macOS | `scp binary root@rune.local:/usr/bin/` |
| Flash ESP32 | macOS or Linux VM | `idf.py flash` |
