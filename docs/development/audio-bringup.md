# Audio Bringup: I2S Mic and Speaker

Getting the SPH0645 microphone and MAX98357A amplifier working over I2S on the T113-S4.

## Mic Wiring (SPH0645 / INMP441)

| Mic Pin   | T113 (MangoPi) Pin | Notes |
|-----------|---------------------|-------|
| VDD       | 3.3V                | Power. Both SPH0645 and INMP441 are 3.3V. |
| GND       | GND                 | Ground |
| BCLK      | I2S0_BCLK           | Bit clock |
| LRCLK/WS  | I2S0_LRCK           | Word select (left/right clock) |
| DOUT      | I2S0_DIN            | Mic data OUT goes to T113 data IN |
| L/R SEL   | GND                 | Tie to GND for left channel. Tie to VDD for right channel. |

The L/R SEL pin determines which stereo channel the mic outputs on. Since we only have one mic, it doesn't matter much — just be consistent and know which channel to read.

**SPH0645 vs INMP441**: functionally interchangeable for this project. SPH0645 (Adafruit breakout) is easier to get in the US. INMP441 boards are cheaper from AliExpress. Both output I2S, both work at 3.3V.

## Amp Wiring (MAX98357A)

| Amp Pin | T113 (MangoPi) Pin | Notes |
|---------|---------------------|-------|
| VIN     | 5V                  | Power. Must be 5V, not 3.3V. The amp needs 5V for the output stage. |
| GND     | GND                 | Ground |
| BCLK    | I2S0_BCLK           | Same wire as mic BCLK (shared bus) |
| LRCLK   | I2S0_LRCK           | Same wire as mic LRCLK (shared bus) |
| DIN     | I2S0_DOUT           | T113 data OUT goes to amp data IN |
| GAIN    | (floating)          | Leave floating for 9dB gain. Tie to GND for 12dB. Tie to VDD for 6dB. |
| SD      | 3.3V (via 10kΩ)     | Shutdown pin. HIGH = enabled. LOW = shutdown/mute. Pull HIGH to enable. |

Connect the speaker wires to the amp's `+` and `-` output terminals.

## Shared I2S Bus

This is the key thing to understand:

```
                    T113 I2S0
                   ┌──────────┐
                   │  BCLK  ──┼──────┬──────── BCLK (mic)
                   │  LRCK  ──┼──────┬──────── LRCLK (mic)
                   │  DIN   ──┼──────│──────── DOUT (mic)
                   │  DOUT  ──┼──────│──────── DIN (amp)
                   └──────────┘      │
                                     ├──────── BCLK (amp)
                                     └──────── LRCLK (amp)
```

BCLK and LRCLK are shared — they run from the T113 to both the mic and amp. The T113 is the I2S master, so it drives these clock lines.

The data lines are separate:
- **Mic DOUT → T113 DIN**: mic sends audio data to the T113 (capture)
- **T113 DOUT → Amp DIN**: T113 sends audio data to the amp (playback)

Do **not** connect mic DOUT to amp DIN. They are independent data paths.

## Device Tree Configuration

The T113 I2S0 peripheral needs to be enabled in the device tree. The exact node name and properties depend on the kernel version and BSP.

### Tina Linux / Allwinner BSP

```dts
&i2s0 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&i2s0_pins>;
};
```

You may also need a `simple-audio-card` or `sound` node to bind the I2S interface to an ALSA sound card:

```dts
sound {
    compatible = "simple-audio-card";
    simple-audio-card,name = "rune-audio";
    simple-audio-card,format = "i2s";
    simple-audio-card,bitclock-master = <&cpu_dai>;
    simple-audio-card,frame-master = <&cpu_dai>;

    cpu_dai: simple-audio-card,cpu {
        sound-dai = <&i2s0>;
    };

    simple-audio-card,codec {
        sound-dai = <&dummy_codec>;
    };
};

dummy_codec: dummy-codec {
    compatible = "linux,spdif-dit";  /* dummy codec for I2S devices */
    #sound-dai-cells = <0>;
};
```

This tells the kernel there's a sound card with I2S CPU interface and a dummy codec (since the SPH0645 and MAX98357A don't have a control interface — they're I2S only).

### Mainline Linux

On mainline, you might use the `simple-audio-card` binding with `max98357a` codec:

```dts
sound {
    compatible = "simple-audio-card";
    simple-audio-card,name = "rune-audio";

    simple-audio-card,dai-link@0 {
        format = "i2s";
        bitclock-master = <&cpu_dai>;
        frame-master = <&cpu_dai>;

        cpu_dai: cpu {
            sound-dai = <&i2s0>;
        };

        codec {
            sound-dai = <&max98357a>;
        };
    };
};

max98357a: audio-amp {
    compatible = "maxim,max98357a";
    #sound-dai-cells = <0>;
    sdmode-gpios = <&pio PB 5 GPIO_ACTIVE_HIGH>;  /* SD pin GPIO */
};
```

After modifying the device tree, recompile and flash.

## Test Capture (Microphone)

```bash
# List sound cards
arecord -l
# Should show your I2S card

# List PCM devices
arecord -L

# Record 5 seconds of audio
arecord -D hw:0,0 -f S32_LE -r 16000 -c 1 -d 5 /tmp/test.wav
```

Format notes:
- `-f S32_LE`: SPH0645 outputs 24-bit data in a 32-bit frame. Use S32_LE and extract 24 bits.
- `-r 16000`: 16kHz sample rate. This is what the voice pipeline uses.
- `-c 1`: mono. The mic only outputs on one channel (left or right depending on L/R SEL pin).

If `arecord` errors with "no soundcards found", the device tree isn't configured correctly or the I2S driver didn't load. Check `dmesg | grep -i i2s` and `dmesg | grep -i sound`.

## Test Playback (Speaker)

```bash
# Play back the recording
aplay -D hw:0,0 /tmp/test.wav

# Or generate a test tone
speaker-test -D hw:0,0 -t sine -f 440 -l 1
# Plays a 440Hz sine wave (A4) for one period
```

If `aplay` doesn't exist on your image, use `speaker-test` or `tinyplay` (from tinyalsa).

## Common Issues

### No Sound Cards Detected

```bash
cat /proc/asound/cards
# Should show at least one card
```

If empty:
- I2S0 not enabled in device tree (`status = "okay"`)
- Sound card node missing from device tree
- I2S kernel module not loaded (`lsmod | grep i2s`, or check if it's built-in)

### "Device or resource busy"

Another process has the ALSA device open. Check with `fuser /dev/snd/*` and kill it.

### Wrong Sample Rate

The T113 I2S clock tree can be finicky. Not all sample rates are achievable depending on the PLL configuration. 16000Hz and 48000Hz are usually fine. If you get errors about unsupported sample rates, check the clock tree configuration.

```bash
# Check supported formats
arecord -D hw:0,0 --dump-hw-params
```

### Mono vs Stereo Confusion

The SPH0645 outputs on one channel only (left or right, depending on L/R SEL). When recording with `-c 1` (mono), ALSA may still read both channels and mix them. If you get silence, try `-c 2` (stereo) and check both channels — your audio might be on the right channel only.

```bash
# Record stereo, then inspect
arecord -D hw:0,0 -f S32_LE -r 16000 -c 2 -d 5 /tmp/test_stereo.wav
# Copy to your Mac and open in Audacity to see which channel has audio
```

### No Sound from Amp

1. **SD pin not pulled high**: the MAX98357A has a shutdown pin (SD). If floating or low, the amp is muted. Pull it to 3.3V via a 10kΩ resistor or drive it from a GPIO.
2. **VIN not 5V**: the amp needs 5V, not 3.3V. At 3.3V it may technically work but with very low output and possible distortion.
3. **Speaker not connected**: obvious, but check. Also verify the speaker impedance is 4-8Ω.
4. **GAIN pin misconfigured**: leave floating for 9dB. If you accidentally tied it to both GND and VDD (e.g., bad breadboard connection), the amp will behave unpredictably.

### Garbled Audio

1. **BCLK/LRCLK swapped**: these are different signals. BCLK is fast (bit rate x bits per sample), LRCLK is slow (sample rate). If swapped, you get noise.
2. **Wrong I2S format**: the T113 might be configured for left-justified or DSP mode instead of standard I2S. Check the device tree format setting.
3. **Wrong bit depth**: SPH0645 is 24-bit in a 32-bit frame. MAX98357A accepts 16 or 32-bit. Make sure the ALSA format matches.
4. **BCLK polarity wrong**: some devices sample on rising edge, others on falling. Check datasheets if basic config doesn't work.

### Debugging with Logic Analyzer

Hook up the Saleae Logic clone to BCLK, LRCLK, and one of the data lines.

In PulseView or Saleae Logic 2:
1. Set up I2S protocol decoder
2. Verify BCLK frequency = sample_rate x bits_per_channel x 2 (e.g., 16000 x 32 x 2 = 1.024 MHz)
3. Verify LRCLK frequency = sample_rate (16000 Hz)
4. Check that data transitions are aligned to BCLK edges

If the clocks look right but data is wrong, the issue is in the software configuration or the device tree.

## ALSA Mixer Configuration

Depending on the device tree and codec driver, you may need to set mixer controls:

```bash
# List mixer controls
amixer -c 0 contents

# Set playback volume (if there's a volume control)
amixer -c 0 set 'Master' 80%

# Unmute
amixer -c 0 set 'Master' unmute
```

On a simple I2S setup with dummy codec, there may be no mixer controls at all. Volume is then controlled by adjusting the PCM samples in software.

## Next Steps

Once you can record and play back:

1. Write a simple program that captures audio, applies basic processing (gain, noise gate), and plays it back.
2. Test with the voice pipeline: record a phrase, send to a speech API, play back the response.
3. Optimize buffer sizes for low latency.
4. Implement the full duplex audio path for the voice pipeline.
