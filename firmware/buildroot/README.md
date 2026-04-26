# Buildroot Configuration

Buildroot configuration for the Rune Linux system. Buildroot generates the complete **rootfs**, **kernel image**, and **bootloader** for the T113-S4.

## Target Configuration

- **SoC:** Allwinner T113-S4
- **CPU:** ARM Cortex-A7 (single core)
- **C library:** musl libc (small footprint)
- **Rootfs:** minimal, stripped down for embedded use

## Included Packages

Essential packages included in the rootfs:

- **ALSA utils** -- audio device configuration and testing
- **wpa_supplicant** -- WiFi connection management
- **openssh-server** -- remote access for development
- Custom package for the **Rune userspace application**

## Building

```bash
cd firmware/buildroot/
make rune_t113s4_defconfig
make
```

Output images land in `output/images/`:

- `sdcard.img` -- complete SD card image (flash with dd or balenaEtcher)
- `sunxi-spl.bin` -- SPL for FEL mode flashing
- `u-boot.bin` -- bootloader
- `uImage` -- kernel image
- `rootfs.squashfs` -- root filesystem

First build takes 30-60 minutes. Subsequent rebuilds are incremental.

## Current status

Not yet populated. Buildroot defconfig and package definitions will land here as the system image is brought up. See `docs/development/first-boot.md` for initial device setup once the image is ready.
