# Kernel

Linux kernel configuration and patches for the Allwinner T113-S4.

Currently using **mainline Linux 6.x** built via Buildroot. The T113-S4 is based on the sun8i family and has improving mainline support, but some patches may still be needed for specific peripherals.

## What will go here

- Custom device tree overlays for:
  - **SPI display** -- 3.7" e-ink panel connected via SPI0
  - **I2S audio** -- MEMS microphone (capture) and amplifier (playback)
  - **UART to ESP32** -- UART1 at 115200 baud for co-processor communication
- Kernel config fragments (defconfig additions)
- Any out-of-tree patches needed for T113 peripheral support

## Current status

Not yet populated. Kernel configuration is currently managed entirely through Buildroot. As custom device tree work begins, overlays and patches will land here.

See `firmware/buildroot/` for the current kernel build configuration.
