# Rune Hardware

Hardware design files for Rune, a 78x78x13mm pocket AI companion.

**Processor:** Allwinner T113-S4 (dual Cortex-A7, 128MB DDR3)
**Co-processor:** ESP32-S3 (WiFi/BLE, notification link, optional wireless fallback)
**Display:** 3.7" monochrome e-ink (480x280)

## Status

Prototype phase. We are building on dev boards and breadboards. The custom PCB design has not started yet.

## License

All hardware files in this directory are licensed under [CERN-OHL-S v2.0](https://ohwr.org/cern_ohl_s_v2.txt). You may redistribute and modify these designs under the terms of that license.

## Directory Layout

```
hardware/
  BOM.md              Bill of materials (prototype and production)
  wiring.md           Prototype wiring guide and pin assignments
  schematics/         KiCad schematic files (placeholder, not yet started)
  pcb/                KiCad PCB layout files (placeholder, not yet started)
  enclosure/          3D-printed enclosure design
    stl/              STL files for printing (placeholder)
```

## Key Documents

- [BOM.md](BOM.md) -- Parts list and costs for both prototype and production builds
- [wiring.md](wiring.md) -- How to connect everything on the prototype breadboard
