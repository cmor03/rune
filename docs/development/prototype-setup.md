# Prototype Assembly Guide

Breadboard prototype for Rune. This gets every subsystem wired and testable before committing to a PCB.

## What You're Building

A 200x150mm baseplate holding:
- MangoPi MQ-R (T113-S4 SBC)
- ESP32-S3-DevKitC-1
- Waveshare 3.7" e-ink display
- SPH0645 I2S mic breakout
- MAX98357A I2S amp + speaker
- USB UVC camera
- EC11 rotary encoder + tactile buttons
- LiPo battery with charge/regulation circuit

## Step 1: Baseplate and Board Mounting

Mount the MangoPi MQ-R and ESP32-S3-DevKitC-1 to a 200x150mm acrylic or 3D-printed baseplate using M2 standoffs (6mm height is good). Leave room between them for breadboard space. Place the breadboard(s) in the center.

Orientation: MangoPi on the left, ESP32 on the right, breadboard in the middle. E-ink display gets its own area at the top — don't crowd it, the ribbon cable is fragile.

[insert photo of completed prototype]

## Step 2: Power Circuit

```
LiPo 3.7V → TP4056 charge module → MP1584 buck converter (set to 5V) → 5V breadboard rail
                                                                          ↓
                                                            Dev board 3.3V regulators → 3.3V rail
```

### Wiring

1. Connect LiPo positive to TP4056 `BAT+` pad, negative to `BAT-`.
2. Connect TP4056 `OUT+` to MP1584 `VIN+`, TP4056 `OUT-` to MP1584 `VIN-`.
3. **Before connecting anything else**, adjust the MP1584 potentiometer with a multimeter on the output until it reads 5.0V. Do this with no load attached.
4. Connect MP1584 `VOUT+` to the breadboard 5V rail, `VOUT-` to the GND rail.
5. Power MangoPi from the 5V rail via its 5V pin (not USB — you want to control the power path).
6. Power ESP32 from the 5V rail via its 5V/VIN pin.
7. Both boards have onboard 3.3V regulators. Use their 3.3V output pins to feed a 3.3V breadboard rail for the display and other 3.3V peripherals.

> **WARNING: LiPo polarity.** Triple-check positive and negative before connecting the battery. Reversed polarity will instantly destroy the TP4056 and possibly start a fire. LiPo cells from different vendors use different JST connector polarities — don't trust the connector, measure with a multimeter.

## Step 3: E-Ink Display (SPI)

Wire the Waveshare 3.7" e-paper HAT to T113 SPI0:

| E-Ink Pin | MangoPi Pin | Function |
|-----------|-------------|----------|
| VCC       | 3.3V rail   | Power    |
| GND       | GND rail    | Ground   |
| DIN (MOSI)| SPI0_MOSI   | Data in  |
| CLK (SCLK)| SPI0_CLK    | Clock    |
| CS        | SPI0_CS0    | Chip select |
| DC        | GPIO (pick one, e.g., PB2) | Data/command select |
| RST       | GPIO (pick one, e.g., PB3) | Reset (active low) |
| BUSY      | GPIO (pick one, e.g., PB4) | Busy signal (low = busy) |

Use short jumper wires for SPI — long wires cause signal integrity problems at higher clock speeds.

> **WARNING: E-ink ribbon cable.** The FPC ribbon connecting the glass panel to the driver board is extremely fragile. Don't bend it sharply, don't force it into the connector, and make sure the latch is open before inserting. If the ribbon tears, the display is dead — there's no repairing it.

[insert photo of display wiring]

## Step 4: Audio (I2S)

The mic and amp share BCLK and LRCLK from T113 I2S0. They have separate data lines.

### Mic (SPH0645 / INMP441)

| Mic Pin  | MangoPi Pin  | Function |
|----------|--------------|----------|
| VDD      | 3.3V rail    | Power    |
| GND      | GND rail     | Ground   |
| BCLK     | I2S0_BCLK    | Bit clock |
| LRCLK/WS | I2S0_LRCK   | Word select |
| DOUT     | I2S0_DIN     | Mic data out → T113 data in |
| L/R SEL  | GND          | Left channel (tie to VDD for right) |

### Amp (MAX98357A)

| Amp Pin  | MangoPi Pin  | Function |
|----------|--------------|----------|
| VIN      | 5V rail      | Power (needs 5V for speaker drive) |
| GND      | GND rail     | Ground   |
| BCLK     | I2S0_BCLK    | Bit clock (same wire as mic) |
| LRCLK    | I2S0_LRCK    | Word select (same wire as mic) |
| DIN      | I2S0_DOUT    | T113 data out → amp data in |
| GAIN     | (floating)   | 9dB gain (or tie to GND for 12dB, VDD for 6dB) |
| SD       | 3.3V rail    | Shutdown pin — pull HIGH to enable |

> **WARNING: I2S wiring order matters.** Connect BCLK and LRCLK first, then data lines. If you connect data before clocks, some I2S devices latch into a bad state and need a power cycle. Also: DOUT from the mic goes to the T113's data IN pin, and the T113's data OUT pin goes to the amp's DIN. Don't mix these up — the naming is from each device's perspective.

Connect the speaker to the amp's `+` and `-` output terminals. Polarity doesn't matter for a single speaker, but be consistent if you add a second one later.

[insert photo of audio wiring]

## Step 5: Camera (USB UVC)

Simple — it's USB.

1. Connect the USB UVC camera module to the MangoPi's USB host port.
2. If the MangoPi only exposes a micro-USB OTG port, use a USB OTG adapter to convert to USB-A host, then plug the camera in.

That's it. No wiring, no configuration at this stage. The kernel handles UVC natively.

## Step 6: ESP32-S3 Co-Processor (UART)

Wire UART2 from MangoPi to UART on ESP32:

| MangoPi Pin | ESP32 Pin | Function |
|-------------|-----------|----------|
| UART2_TX    | RX (GPIO44) | T113 transmit → ESP32 receive |
| UART2_RX    | TX (GPIO43) | ESP32 transmit → T113 receive |
| GND         | GND       | Common ground (essential) |

**TX/RX crossover**: T113 TX goes to ESP32 RX, and vice versa. This is the most common wiring mistake.

Both boards are powered from the same 5V rail (through their own regulators), so they share a common ground already via the breadboard. But run an explicit GND jumper wire between them — don't rely on the breadboard rail alone for signal integrity.

> **WARNING: ESP32 boot mode.** The ESP32-S3 has a BOOT button (GPIO0). If GPIO0 is held low during power-on, the chip enters download mode instead of running your firmware. Make sure nothing is pulling GPIO0 low. If the ESP32 seems dead after flashing, power cycle it with GPIO0 floating/high.

## Step 7: Input Controls

### EC11 Rotary Encoder

| Encoder Pin | MangoPi Pin | Function |
|-------------|-------------|----------|
| A           | GPIO (e.g., PB5) | Encoder channel A |
| B           | GPIO (e.g., PB6) | Encoder channel B |
| SW (switch) | GPIO (e.g., PB7) | Push button |
| GND         | GND rail    | Ground   |
| VCC         | 3.3V rail   | Power (for some encoder modules) |

Add 10kΩ pull-up resistors from A, B, and SW to 3.3V if your encoder module doesn't have them onboard.

### Tactile Buttons

Wire each button between a GPIO pin and GND. Enable internal pull-ups in the device tree or add external 10kΩ pull-ups to 3.3V. Pressing the button pulls the pin low.

[insert photo of controls wiring]

## Step 8: Test Each Subsystem in Isolation

Do **not** power everything on at once for the first time. Test each piece:

1. **Power**: measure 5V and 3.3V rails under no load, then with just one board at a time.
2. **Serial console**: connect USB-TTL adapter to MangoPi UART0, boot Linux, verify you get a shell.
3. **Display**: run a test program that draws to the e-ink (see `e-ink-bringup.md`).
4. **Audio**: record from mic, play through amp (see `audio-bringup.md`).
5. **Camera**: check `lsusb` and grab a test frame (see `camera-bringup.md`).
6. **ESP32 UART**: send test bytes from T113, verify echo from ESP32 (see `esp32-coprocessor.md`).
7. **Encoder/buttons**: read GPIO values while turning/pressing.

Only after each subsystem works individually should you power everything together.

## Common Mistakes Checklist

- [ ] LiPo polarity verified with multimeter before connecting
- [ ] MP1584 output adjusted to 5.0V before connecting to any board
- [ ] E-ink ribbon cable seated fully, latch closed
- [ ] SPI wires are short (under 15cm)
- [ ] I2S BCLK and LRCLK connected before data lines
- [ ] Mic DOUT goes to T113 DIN (not the amp)
- [ ] Amp DIN comes from T113 DOUT (not the mic)
- [ ] Amp SD pin pulled HIGH (not floating — floating = shutdown on some variants)
- [ ] Amp VIN is 5V, not 3.3V
- [ ] UART TX/RX crossed between T113 and ESP32
- [ ] ESP32 GPIO0 not pulled low (would enter download mode)
- [ ] Common GND wire between all boards
- [ ] Pull-up resistors on encoder and button GPIOs

[insert photo of completed prototype]
