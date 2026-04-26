# Schematics

This directory will contain KiCad schematic files for the Rune custom PCB once the design is complete.

## Current Status

We are in the prototype phase using off-the-shelf dev boards (MangoPi MQ-R, ESP32-S3-DevKitC) and breakout modules on breadboards. There are no custom schematics yet.

For current prototype connections, see [../wiring.md](../wiring.md).

## Planned Files

When the custom PCB design begins, this directory will contain:

- `rune.kicad_sch` -- Top-level schematic
- `power.kicad_sch` -- Power management (AXP2101 PMIC, battery charging, voltage rails)
- `t113.kicad_sch` -- T113-S4 and supporting circuitry (DDR3, eMMC, oscillators)
- `esp32.kicad_sch` -- ESP32-S3-WROOM-1 module and antenna keep-out
- `peripherals.kicad_sch` -- Display, audio, camera, buttons, USB-C
- `rune.kicad_sym` -- Custom symbol library (if needed)
