# Bill of Materials

Two BOMs are listed here: one for the current breadboard prototype, one for the eventual production board. The prototype BOM uses off-the-shelf dev boards and breakouts so you can get a working system without designing a custom PCB. The production BOM is an estimate for a custom 4-layer board at modest volume.

---

## Prototype BOM (~$210)

Everything you need to build a working Rune prototype on breadboards.

| # | Item | Link / Part Number | Unit Cost | Supplier | Notes |
|---|------|--------------------|-----------|----------|-------|
| 1 | MangoPi MQ-R T113-S3 dev board | MangoPi MQ-R / T113-S3 | ~$25 | AliExpress, MangoPi resellers | T113-S3 is usable for prototype work while the production design targets T113-S4. Exposes SPI, I2S, UART, USB, and GPIO on headers. |
| 2 | ESP32-S3-DevKitC-1 | ESP32-S3-DevKitC-1-N8R2 or N16R8 | ~$10 | Espressif, Digi-Key, Mouser | Development stand-in for the ESP32-S3-WROOM module. Handles BLE notification experiments and UART protocol work. |
| 3 | Waveshare 3.7" e-paper module | Waveshare 3.7inch e-Paper HAT | ~$35 | Waveshare, Amazon, PiShop | Includes driver board and FPC cable. SPI interface. Prototype firmware should use spidev first. |
| 4 | Adafruit SPH0645 I2S MEMS mic breakout | Adafruit 3421 | ~$7 | Adafruit, Digi-Key | I2S output, 3.3V logic. INMP441 breakouts are an acceptable substitute if pinout is documented. |
| 5 | Adafruit MAX98357A I2S amp | Adafruit 3006 | ~$6 | Adafruit, Digi-Key | I2S input, drives 4-8 ohm speakers directly. Shares BCLK/LRCLK with the microphone. |
| 6 | 8 ohm 1W speaker | Generic 28-40mm 8 ohm speaker | ~$2 | Adafruit, SparkFun, AliExpress | Use a small full-range speaker. Confirm the final enclosure depth before committing to a part. |
| 7 | USB UVC camera module | USB UVC module, OV2640/OV5640 class | ~$15 | Amazon, AliExpress | Must enumerate as standard USB Video Class. Avoid bare CSI/MIPI modules for the first prototype. |
| 8 | EC11 rotary encoder | EC11, 15mm shaft, push switch | ~$3 | Digi-Key, AliExpress | Main navigation control. Debounce in software or with RC filtering during hardware revision. |
| 9 | Tactile switch assortment | 6x6mm through-hole tactile switches | ~$5 | Amazon, AliExpress | Used for voice, back, page, and shutter actions during layout exploration. |
| 10 | 2000mAh LiPo + TP4056 USB-C charger | Single-cell LiPo, TP4056 Type-C module | ~$12 | Adafruit, SparkFun, Amazon | Prototype power system only. Verify polarity before connecting to any board. |
| 11 | MP1584 adjustable buck converter | MP1584EN module | ~$5 | Amazon, AliExpress | Set to 5V before connecting load. Powers camera and any 5V-only prototype peripherals. |
| 12 | Breadboards, jumpers, pin headers | Full-size breadboards, M-M/M-F/F-F jumpers | ~$20 | Amazon, Adafruit | Use short wires for SPI and I2S. Long jumpers make bring-up much harder. |
| 13 | 3x 32GB SanDisk Extreme microSD cards | SDSQXAF-032G or equivalent | ~$25 | SanDisk, Amazon | One rootfs card, one experimental card, one known-good recovery card. |
| 14 | USB-C TTL serial adapter | 3.3V USB-UART, CH340/CP2102/FT232 | ~$12 | Adafruit, SparkFun, Amazon | Required for boot logs and recovery. Do not use 5V UART levels. |
| 15 | Passives, hookup wire, heat shrink | 10k/4.7k resistors, 100nF/10uF capacitors | ~$28 | Digi-Key, Mouser, Amazon | Pull-ups, pull-downs, decoupling, quick fixes, and cleaner breadboard power routing. |

**Prototype total: ~$210**

Prices are approximate and will vary by region and supplier. Most of these are available from Adafruit, SparkFun, AliExpress, or Mouser.

---

## Production BOM (~$45-65 at 100+ units)

Target BOM for a custom 4-layer PCB at 100+ unit volume.

| # | Item | Link / Part Number | Unit Cost | Supplier | Notes |
|---|------|--------------------|-----------|----------|-------|
| 1 | Allwinner T113-S4 | T113-S4 BGA | ~$5-8 | LCSC, Shenzhen distributors | Dual Cortex-A7 with integrated DDR. Contract assembly required; hand assembly is not realistic. |
| 2 | ESP32-S3-WROOM-1 module | ESP32-S3-WROOM-1-N8R2 | ~$3-5 | Espressif, Digi-Key, Mouser | Pre-certified WiFi/BLE module. Reduces RF layout and certification risk. |
| 3 | Custom 4-layer PCB | 78x78mm, 4-layer FR4, ENIG | ~$4-8 | JLCPCB, PCBWay, MacroFab | Final stackup and impedance constraints are pending PCB design. |
| 4 | 16GB eMMC | BGA-153 eMMC 5.1 | ~$3-5 | Kingston, Micron, Kioxia distributors | Replaces microSD for durability and lower profile. Exact vendor pending validation. |
| 5 | 3.7" raw e-paper panel | Waveshare raw panel or equivalent | ~$8-12 | Waveshare, display distributors | Panel only, no driver board. Requires FPC connector and validated waveform/LUT handling. |
| 6 | AXP2101 PMIC | AXP2101 | ~$1.50-3 | LCSC, Shenzhen distributors | Battery charging, rails, sequencing, and power-key behavior. Register configuration must be validated on hardware. |
| 7 | 1000mAh LiPo battery | Custom or standard 1S cell, JST-PH | ~$3-5 | Battery vendor TBD | Capacity and dimensions must fit the 78x78x13mm enclosure with swelling clearance. |
| 8 | USB-C connector | Mid-mount 16-pin USB-C receptacle | ~$0.30-0.80 | GCT, Amphenol, LCSC | Charging and data. Needs CC resistors and ESD protection. |
| 9 | Custom enclosure | Injection molded or printed production shell | ~$5-12 | Manufacturer TBD | 3D printing is suitable for early units; injection molding only makes sense at higher volume. |
| 10 | Passives, connectors, FPC cables | MLCCs, resistors, ESD, ferrites, FPC | ~$5-10 | Digi-Key, Mouser, LCSC | Includes decoupling, display connector, battery connector, test pads, and board-to-board details. |

**Production target: $45-65 per unit at 100+ quantity**

---

## Prototype vs. Production: What Changes

The prototype uses dev boards (MangoPi MQ-R, ESP32-S3-DevKitC) that each include their own voltage regulators, USB connectors, flash storage, and debug interfaces. This is convenient but bulky and expensive.

The production board puts the T113-S4 and ESP32-S3 directly on a custom PCB. The AXP2101 PMIC replaces the separate buck converter and TP4056 charger. eMMC replaces the microSD card. The e-ink panel connects via FPC instead of a breakout board.

The result is roughly 4x smaller and 3-4x cheaper per unit, but requires PCB design, BGA soldering, and proper power sequencing -- none of which you need to worry about for prototyping.
