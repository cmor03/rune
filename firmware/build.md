# Building the Rune Firmware

## Prerequisites

You need a **Linux development environment**. This means either a native Linux machine or a Linux VM. macOS cannot natively cross-compile ARM Linux binaries (see note below).

Required tools:

- Git
- ARM cross-compile toolchain: `gcc-arm-linux-gnueabihf` (for C) or Rust with the `armv7` target (for Rust)
- For ESP32: `esp-idf` toolchain (v5.x)
- Standard build tools: `make`, `cmake`, `pkg-config`

### Apple Silicon Note

If you're on an M-series Mac, you **must** use a Linux VM for building. Recommended options:

- **OrbStack** (lightweight, fast, recommended)
- **UTM** (full VM, more flexibility)

macOS cross-compilation toolchains for ARM Linux are unreliable and poorly supported. Don't fight it -- just use a VM.

## Clone the Repo

```bash
git clone <repo-url>
cd rune
```

## Building the Userspace App (Rust)

### 1. Add the ARM target

```bash
rustup target add armv7-unknown-linux-gnueabihf
```

### 2. Configure cross-linker

Create or update `.cargo/config.toml` in the project root:

```toml
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

Make sure `arm-linux-gnueabihf-gcc` is on your PATH. On Debian/Ubuntu:

```bash
sudo apt install gcc-arm-linux-gnueabihf
```

### 3. Build

```bash
cargo build --target armv7-unknown-linux-gnueabihf --release
```

### 4. Build artifacts

The compiled binary lands at:

```
target/armv7-unknown-linux-gnueabihf/release/rune
```

This is the single binary that gets deployed to the device.

## Building the Userspace App (C Fallback)

If using the C implementation instead of Rust:

```bash
make CROSS_COMPILE=arm-linux-gnueabihf- ARCH=arm
```

Output binary will be in the build directory as configured by the Makefile.

## Building the ESP32 Firmware

The ESP32-S3 co-processor firmware lives in `firmware/esp32/` and uses the esp-idf build system.

### 1. Install esp-idf

Follow the [esp-idf installation guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/get-started/). Then source the environment:

```bash
. $HOME/esp/esp-idf/export.sh
```

### 2. Build

```bash
cd firmware/esp32/
idf.py set-target esp32s3
idf.py build
```

Build artifacts will be in `firmware/esp32/build/`.

## Building the Linux System (Buildroot)

See `firmware/buildroot/` for Buildroot configuration. Full system image build (kernel + rootfs + bootloader) is handled by Buildroot:

```bash
cd firmware/buildroot/
make rune_t113s4_defconfig
make
```

This produces a complete SD card image. Buildroot builds take a while on first run (30-60 minutes depending on hardware).

## Troubleshooting

**Linker errors during Rust build:** Make sure `gcc-arm-linux-gnueabihf` is installed and the `.cargo/config.toml` linker path is correct.

**esp-idf not found:** Source the esp-idf export script in every new shell session, or add it to your shell profile.

**Buildroot build fails:** Ensure all Buildroot host dependencies are installed. Run `make check-host` to verify.
