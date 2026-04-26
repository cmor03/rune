# Prototype Wiring Guide

How to connect everything for the Rune breadboard prototype. All connections are between the MangoPi MQ-R (T113-S3) dev board, ESP32-S3-DevKitC, and peripheral breakouts.

Read all of this before you start wiring. Some buses are shared and order matters.

---

## Voltage Rails

Two voltage rails in the system:

- **3.3V** -- Supplied by the MangoPi MQ-R's onboard regulator. Powers the e-ink module, mic breakout, ESP32 (via its own USB or 3.3V pin), and all logic-level signals.
- **5V** -- Supplied by the MP1584 buck converter (input from LiPo via TP4056 output). Powers the USB camera module and the MAX98357A amp (VBUS/VIN pin). The buck converter input comes from the TP4056 output (~4.2V-3.0V), set the buck to a stable 5V.

Do not power the camera from the MangoPi's USB port during development -- it may brownout the T113. Use the separate 5V buck.

---

## Subsystem 1: E-Ink Display (SPI0)

The Waveshare 3.7" e-paper module connects via SPI with extra GPIO for control signals.

### Pin Assignments

| E-Ink Module Pin | T113 Pin (MQ-R Header) | Function |
|-----------------|----------------------|----------|
| DIN (MOSI) | SPI0_MOSI (PC2) | SPI data out |
| CLK (SCLK) | SPI0_CLK (PC0) | SPI clock |
| CS | SPI0_CS0 (PC3) | Chip select (active low) |
| DC | PD0 (GPIO) | Data/command select |
| RST | PD1 (GPIO) | Reset (active low) |
| BUSY | PD2 (GPIO) | Busy signal from display |
| GND | GND | Ground |
| VCC | 3.3V | Power |

### Connection Diagram

```
MangoPi MQ-R                    Waveshare 3.7" E-Paper
+------------------+            +------------------+
|                  |            |                  |
|  SPI0_MOSI (PC2) |---->>-----| DIN              |
|  SPI0_CLK  (PC0) |---->>-----| CLK              |
|  SPI0_CS0  (PC3) |---->>-----| CS               |
|  GPIO      (PD0) |---->>-----| DC               |
|  GPIO      (PD1) |---->>-----| RST              |
|  GPIO      (PD2) |----<<-----| BUSY             |
|                  |            |                  |
|  3.3V            |---->>-----| VCC              |
|  GND             |---->>-----| GND              |
+------------------+            +------------------+
```

---

## Subsystem 2: Audio -- I2S Mic and Amp (I2S0, Shared Bus)

The SPH0645 mic and MAX98357A amp share the same I2S0 bus. This works because the mic is output-only (sends data to T113) and the amp is input-only (receives data from T113). They share BCLK and LRCLK but use separate data lines.

### Pin Assignments

| Signal | T113 Pin (MQ-R Header) | SPH0645 Mic Pin | MAX98357A Amp Pin |
|--------|----------------------|-----------------|-------------------|
| BCLK | I2S0_BCLK (PB6) | BCLK | BCLK |
| LRCLK | I2S0_LRCK (PB3) | LRCLK (WS) | LRC |
| DIN (to T113) | I2S0_DIN (PB5) | DOUT | -- |
| DOUT (from T113) | I2S0_DOUT (PB4) | -- | DIN |
| GND | GND | GND | GND |
| VCC (mic) | 3.3V | 3.3V | -- |
| VIN (amp) | 5V (buck) | -- | VIN |

The amp GAIN pin controls volume. Leave floating for 9dB, tie to GND for 12dB, or tie to VIN for 15dB. Start with floating.

The amp SD (shutdown) pin: tie to VIN through a 100k resistor to enable, or drive with a GPIO if you want software mute.

### Connection Diagram

```
                        Shared I2S0 Bus
                    +-------+-------+
                    |               |
MangoPi MQ-R       |  SPH0645 Mic  |  MAX98357A Amp
+--------------+    |  +----------+ |  +----------+
|              |    |  |          | |  |          |
| I2S0_BCLK   |----+--| BCLK     | +--| BCLK     |
| I2S0_LRCK   |----+--| WS       | +--| LRC      |
| I2S0_DIN    |--<<---| DOUT     |    |          |
| I2S0_DOUT   |-->>----------------->>| DIN      |
|              |       |          |    |          |
| 3.3V         |-------| 3.3V     |    |          |
| GND          |--+----| GND      |    | GND      |---+
|              |  |    +----------+    +----+-----+   |
+--------------+  |                         |         |
                  +-------------------------+---------+
                                            |
                  5V (buck) ----------------| VIN
                                            |
                                        [Speaker]
                                         8ohm 1W
```

---

## Subsystem 3: ESP32-S3 Communication (UART2)

The ESP32-S3 handles BLE notification mirroring and optional wireless fallback. It communicates with the T113 over UART.

### Pin Assignments

| Signal | T113 Pin (MQ-R Header) | ESP32-S3 DevKitC Pin |
|--------|----------------------|---------------------|
| TX | UART2_TX (PD10) | GPIO18 (RX) |
| RX | UART2_RX (PD11) | GPIO17 (TX) |
| GND | GND | GND |

Cross TX/RX as shown. Both sides run at 3.3V logic, no level shifter needed.

### Connection Diagram

```
MangoPi MQ-R                    ESP32-S3-DevKitC
+------------------+            +------------------+
|                  |            |                  |
|  UART2_TX (PD10) |---->>-----| GPIO18 (RX)      |
|  UART2_RX (PD11) |----<<-----| GPIO17 (TX)      |
|                  |            |                  |
|  GND             |------------| GND              |
+------------------+            +------------------+

Baud rate: 115200 (default), can increase to 921600 after init.
```

---

## Subsystem 4: USB Camera

The camera connects to the MangoPi's USB host port. It must be a UVC-class USB camera -- the T113 on MQ-R does not expose a CSI interface.

### Connection Diagram

```
MangoPi MQ-R                    USB Camera Module
+------------------+            +------------------+
|                  |            |                  |
|  USB_HOST D+     |---->>-----| D+               |
|  USB_HOST D-     |---->>-----| D-               |
|                  |            |                  |
|  GND             |------------| GND              |
+------------------+            +------------------+
                                        |
                  5V (buck) ------------| VBUS
```

Power the camera from the 5V buck converter, not from the MangoPi's USB VBUS. Some camera modules draw 200-400mA and will cause brownouts on the dev board regulator.

If the MQ-R has a USB-A host connector, you can plug in directly but still consider external 5V power for the camera if you see stability issues.

---

## Subsystem 5: Buttons and Rotary Encoder (GPIO)

### Pin Assignments

| Input | T113 Pin (MQ-R Header) | Notes |
|-------|----------------------|-------|
| Encoder A | PD3 (GPIO) | Enable internal pull-up or add external 10k to 3.3V |
| Encoder B | PD4 (GPIO) | Enable internal pull-up or add external 10k to 3.3V |
| Encoder Push | PD5 (GPIO) | Active low with pull-up |
| Button 1 (action) | PD6 (GPIO) | Active low with pull-up |
| Button 2 (back) | PD7 (GPIO) | Active low with pull-up |
| Button 3 (aux) | PD8 (GPIO) | Active low with pull-up |

All buttons wire between the GPIO pin and GND. Use 10k pull-up resistors to 3.3V if the T113 internal pull-ups are not reliable (they sometimes aren't).

### Connection Diagram

```
              3.3V
               |
              [10k]  (pull-up, one per signal)
               |
T113 GPIO -----+---/ /--- GND
           (PD3-PD8)  (switch)


EC11 Rotary Encoder Detail:
+-------+
|  A  B |--- to PD3, PD4 (with pull-ups)
|  C    |--- to GND (common)
| SW    |--- to PD5 (with pull-up)
+-------+
```

Add 100nF debounce caps from each switch pin to GND if you get noisy readings. Software debouncing (5-10ms) usually works fine for buttons; the encoder may need hardware caps.

---

## Common Gotchas

### E-Ink BUSY Signal Polarity

The Waveshare 3.7" BUSY pin is **active high** -- it goes HIGH when the display is busy and LOW when ready. Some e-ink driver code assumes active-low BUSY. Check your driver and invert if needed. Getting this wrong means your code will either never wait (screen corruption) or wait forever (hang).

### ESP32-S3 Boot Mode Pins

The ESP32-S3 has two pins that affect boot mode:

- **GPIO0**: Must be HIGH during reset for normal boot. The DevKitC has a pull-up and a BOOT button. Do not connect anything to GPIO0 unless you know what you're doing.
- **GPIO46**: Must be LOW during reset for SPI boot (normal flash boot). On most modules this is handled internally. Do not drive GPIO46 high during reset.

If the ESP32 won't boot or enters download mode unexpectedly, check that nothing on your breadboard is pulling these pins.

### Ribbon Cable Orientation

The Waveshare e-ink FPC cable has contacts on one side only. If the display shows nothing (not even a flicker during refresh), the cable is probably flipped. The contacts face down toward the PCB on the display side, and the orientation on the breakout board is marked with a small arrow or printed text. Check both ends.

### I2S Word Select for Mono Devices

Both the SPH0645 and MAX98357A are mono devices. They use the LRCLK (word select) signal to determine which channel they operate on:

- **SPH0645 mic**: The SEL pin determines channel. Tie SEL to GND for left channel (data on LRCLK low phase), or tie to 3.3V for right channel. Default (floating) is usually left.
- **MAX98357A amp**: The SD pin also selects channel when tied to specific resistor dividers, but by default it outputs the left channel + right channel averaged. Check the datasheet if you need a specific channel.

If you hear nothing from the amp or get silence from the mic, the channel selection may not match what the T113 I2S driver is configured for. Make sure the driver and hardware agree on left/right channel assignment.

### Power Sequencing

No strict power sequencing is required for the prototype since each dev board has its own regulator. Just make sure:

1. The 5V buck converter is stable before powering on the camera.
2. The ESP32 boots before the T113 tries to talk to it over UART (a 2-second delay in the T113 startup script is sufficient).
3. Do not hot-plug the e-ink FPC cable while powered on.
