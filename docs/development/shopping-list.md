# Shopping List

Everything needed to build a Rune prototype from scratch. Prices are approximate (USD, 2024-2025). Buy extras of cheap stuff — you'll fry something eventually.

## Dev Boards (~$80)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| MangoPi MQ-R (T113-S4) | 2 | $30 | Main processor. Spare because these are hard to get quickly. |
| ESP32-S3-DevKitC-1 (N8R2 or N16R8) | 2 | $20 | Co-processor. Get the S3 variant with USB-C. |
| microSD cards (32GB, Class 10/A1) | 4 | $30 | Two per board. Cheap cards fail constantly — get name brand (Samsung, SanDisk). |

## Mac Connection Essentials (~$85)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| USB-C to TTL serial adapter (CH340 or CP2102) | 2 | $20 | For UART console to MangoPi. Get two — one for T113, one for ESP32. |
| USB-C hub with USB-A ports | 1 | $25 | Your Mac probably only has USB-C. You need USB-A for the serial adapters and camera. |
| USB-A to micro-USB cables | 3 | $10 | For powering dev boards, flashing ESP32. |
| microSD to USB-C card reader | 1 | $15 | For flashing SD cards from your Mac. |
| USB-C to USB-C cable (data) | 1 | $15 | For ESP32 flashing if using USB-C devkit variant. |

## Display (~$35)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| Waveshare 3.7" e-paper HAT (480x280) | 1 | $28 | The specific model matters. Get the 3.7" with 4-gray. |
| FPC extension cable (24-pin, 0.5mm pitch) | 2 | $7 | In case you need distance between driver board and panel. Extras because they tear. |

## Audio (~$23)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| Adafruit SPH0645 I2S MEMS mic breakout | 1 | $7 | Or INMP441 breakout as alternative (~$3 on AliExpress, slower shipping). |
| Adafruit MAX98357A I2S amp breakout | 1 | $6 | 3W class D amp. Plenty loud for a small speaker. |
| 8Ω 1W speaker (40mm) | 2 | $10 | Small form factor. Get two — easy to blow one during testing. |

## Camera (~$22)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| USB UVC camera module (OV2640 or similar) | 1 | $15 | Any generic USB camera works. Must be UVC-compliant (most are). |
| USB OTG adapter (micro-USB male to USB-A female) | 2 | $7 | MangoPi's USB port may be OTG. Need adapter to use as host. |

## Input Controls (~$25)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| EC11 rotary encoders (5-pack) | 1 | $8 | With push switch. Get the pack — the detent feel varies and you'll want to pick the best one. |
| Tactile switch assortment (various sizes) | 1 | $8 | 6x6mm through-hole are best for breadboarding. |
| Knob caps for EC11 | 3 | $9 | Aluminum knurled knobs feel great. 6mm shaft with D-cut. |

## Power (~$32)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| LiPo cells, 3.7V 2000mAh | 2 | $16 | Bigger than the final 1000mAh target. Easier to prototype with more capacity. Get cells with JST-PH connectors and built-in protection circuits. |
| TP4056 USB-C charge modules | 3 | $6 | Cheap and they fail. Buy extras. Make sure it's the USB-C variant. |
| MP1584 adjustable buck converters | 3 | $10 | Set to 5V output. Buy extras — potentiometer adjustment is fiddly and you might overshoot. |

## Wiring & Prototyping (~$76)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| Full-size breadboards (830 tie-point) | 3 | $18 | Don't cheap out. Bad breadboards have intermittent contacts that will waste hours. |
| Jumper wire kit (M-M, M-F, F-F) | 1 | $12 | Get a big assortment. You'll use more than you think. |
| Pin header strips (male, 2.54mm) | 2 packs | $6 | For soldering onto breakout boards. |
| Pin header strips (female, 2.54mm) | 2 packs | $6 | For making adapter cables. |
| Prototyping perfboard (various sizes) | 1 pack | $10 | For semi-permanent sub-assemblies when breadboard gets unwieldy. |
| JST-PH connector kit (2-pin) | 1 | $8 | For battery connections. Match the LiPo connector. |
| JST-XH connector kit (2-6 pin) | 1 | $8 | For other board-to-board connections. |
| Heat shrink tubing assortment | 1 | $8 | For insulating solder joints. |

## Soldering & Bench Tools (~$253)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| Pinecil soldering iron (V2) | 1 | $40 | USB-C powered, portable, excellent temperature control. The best budget iron. |
| Solder, leaded 63/37, 0.5mm | 1 roll | $15 | Leaded is dramatically easier to work with than lead-free. Wash hands after. |
| Flux pen (rosin, no-clean) | 2 | $12 | Makes every joint better. Buy two because you'll lose one. |
| Solder wick (2.5mm) | 2 | $8 | For fixing mistakes. Flux it before use. |
| Brass tip cleaner | 1 | $8 | Better than a wet sponge. Doesn't thermal-shock the tip. |
| Helping hands / PCB holder | 1 | $25 | Third hand with alligator clips. Get one with heavy base, not the cheap wobbly ones. |
| Silicone soldering mat | 1 | $15 | Protects your desk, heat resistant, has little compartments for screws. |
| Wire strippers (AWG 20-30) | 1 | $15 | Self-adjusting type is worth the money. |
| Flush cutters | 1 | $10 | For trimming leads and cutting wire. |
| ESD tweezers (set of 6) | 1 | $12 | Pointed, curved, flat. You'll use these constantly. |
| Magnifying lamp (LED, adjustable arm) | 1 | $35 | For inspecting solder joints and reading tiny text on ICs. |
| Tip tinner | 1 | $8 | For restoring oxidized soldering tips. |
| USB-C PD power supply (65W) | 1 | $30 | Powers the Pinecil. Also useful for powering dev boards. |
| Silicone wire (22-26 AWG, stranded) | 1 set | $20 | More flexible than solid core. Better for point-to-point wiring on perfboard. |

## Diagnostic Tools (~$30)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| Saleae Logic clone (8-channel, 24MHz) | 1 | $15 | For inspecting SPI, I2S, and UART signals. Use with PulseView (sigrok) or Saleae Logic 2. |
| Multimeter (auto-ranging, basic) | 1 | $15 | For checking voltages and continuity. Doesn't need to be fancy. |

## 3D Printing Supplies (~$76)

| Item | Qty | ~Price | Notes |
|------|-----|--------|-------|
| PLA filament (1.75mm, 1kg) | 2 | $40 | One black, one white. Enough for many iterations of the enclosure. |
| M2 brass heat-set inserts (50-pack) | 1 | $10 | Press into 3D-printed parts with soldering iron for durable screw holes. |
| M2x4mm screws (50-pack) | 1 | $6 | For securing PCBs to enclosure. |
| M2x6mm standoffs (male-female, 50-pack) | 1 | $8 | For mounting boards with clearance underneath. |
| M2 nuts (50-pack) | 1 | $4 | For standoffs. |
| M2 washers (50-pack) | 1 | $4 | Prevents screws from cracking PLA. |
| Calipers (digital, 150mm) | 1 | $4 | Essential for measuring components for CAD. |

---

## Total: ~$737

This covers everything from zero — including tools. If you already have a soldering iron, multimeter, wire strippers, etc., subtract the bench tools section (~$253) and you're at ~$484.

---

## Staged Ordering Plan

Don't buy everything at once. Order in phases so you can make progress without waiting for parts you don't need yet.

### Phase 0: Get Linux Booting (~$95)

Buy first. Everything else depends on this working.

- MangoPi MQ-R (1x)
- microSD cards (2x)
- USB-C to TTL serial adapter (1x)
- microSD to USB-C card reader
- USB-C hub
- USB-A to micro-USB cable (1x)

**Goal**: boot Linux, get a shell over serial, connect to WiFi, SSH in.

### Phase 1: Display + Audio (~$58)

Once Linux is running and you can SSH in.

- Waveshare 3.7" e-paper HAT
- FPC extension cable
- SPH0645 I2S mic breakout
- MAX98357A I2S amp breakout
- Speaker (1x)

**Goal**: draw text on the e-ink display, record audio from mic, play audio through speaker.

### Phase 2: Camera + ESP32 + Controls (~$75)

Once display and audio work.

- ESP32-S3-DevKitC-1 (1x)
- USB UVC camera module
- USB OTG adapter
- EC11 rotary encoders (5-pack)
- Tactile switch assortment
- Knob caps

**Goal**: capture a photo, establish UART comms with ESP32, read encoder input.

### Phase 3: Enclosure + Finishing (~$509)

Once all subsystems work on the breadboard.

- Spare dev boards (MangoPi + ESP32)
- Spare SD cards
- Spare serial adapter
- Remaining power components (LiPo cells, TP4056s, buck converters)
- All wiring/prototyping supplies
- All soldering/bench tools (if you don't have them)
- Diagnostic tools
- 3D printing supplies

**Goal**: move from breadboard to permanent prototype, design and print enclosure.

### If You Already Have Tools

Skip the soldering/bench tools section entirely if you have:
- A temperature-controlled soldering iron
- Solder and flux
- Wire strippers and flush cutters
- A multimeter

That brings Phase 3 down significantly and the total to ~$484.
