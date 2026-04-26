# PCB Layout

This directory will contain KiCad PCB layout files for the Rune custom board.

## Current Status

We are prototyping on breadboards with dev boards. The custom PCB design has not started.

## Design Goals

- **Board outline:** 78x78mm, matching the enclosure footprint
- **Layer count:** 4-layer stackup (signal / GND / power / signal)
- **Component placement:** All components on the top side for easier pick-and-place assembly
- **Test points:** Exposed test pads for SPI bus, I2S bus, UART (T113 debug and ESP32 link), 3.3V, 5V, VBAT, and GND
- **Connectors:** FPC connector for e-ink panel, JST-PH for battery, mid-mount USB-C
- **Antenna:** ESP32-S3-WROOM-1 module placed at board edge with proper keep-out zone (no copper under antenna)

## Planned Files

- `rune.kicad_pcb` -- PCB layout
- `rune.kicad_pro` -- KiCad project file
- `rune-footprints.pretty/` -- Custom footprint library
- `gerbers/` -- Manufacturing output files
- `fab/` -- Assembly drawings, pick-and-place files, drill files
