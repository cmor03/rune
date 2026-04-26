# E-Ink Display Bringup

Getting the Waveshare 3.7" e-paper display working with the T113-S4.

## SPI Wiring

All connections are 3.3V logic. Do not connect to 5V.

| E-Ink Pin | T113 (MangoPi) Pin | Function |
|-----------|--------------------|----------|
| VCC       | 3.3V               | Power |
| GND       | GND                | Ground |
| DIN (SDI) | SPI0_MOSI          | Serial data in |
| CLK       | SPI0_CLK           | Serial clock |
| CS        | SPI0_CS0           | Chip select (active low) |
| DC        | GPIO (e.g., PB2)   | Data/command select: LOW = command, HIGH = data |
| RST       | GPIO (e.g., PB3)   | Reset (active low pulse to reset) |
| BUSY      | GPIO (e.g., PB4)   | Busy signal: LOW = busy, HIGH = ready |

Keep wires short. SPI is sensitive to wire length and parasitic capacitance, especially at higher clock speeds. Under 10cm is ideal for breadboard.

## Enable spidev in the Device Tree

The kernel needs to expose `/dev/spidev0.0` for userspace access. This typically requires a device tree overlay or modification.

### Check If spidev Already Exists

```bash
ls /dev/spidev*
```

If `/dev/spidev0.0` is there, skip to the next section.

### Add spidev to the Device Tree

If spidev isn't exposed, you need to modify the device tree source (DTS):

```dts
&spi0 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&spi0_pins>;

    spidev@0 {
        compatible = "linux,spidev";
        reg = <0>;
        spi-max-frequency = <10000000>;  /* 10 MHz — start conservative */
    };
};
```

Compile and replace the DTB:

```bash
# In your Linux VM
dtc -I dts -O dtb -o sun8i-t113s-mangopi-mq-r.dtb sun8i-t113s-mangopi-mq-r.dts
# Copy to SD card's boot partition
```

After rebooting with the new DTB, `/dev/spidev0.0` should appear.

## Approach: Userspace SPI Driver

For prototyping, driving the display from userspace via spidev is the right call. It's easier to debug, faster to iterate, and you don't need to write a kernel module.

The kernel framebuffer approach (fbtft or DRM) is cleaner for production but adds complexity during bring-up. Save it for later.

## GPIO Setup

The DC, RST, and BUSY pins are regular GPIOs. Access them via sysfs or libgpiod.

```bash
# Using sysfs (works everywhere, deprecated but simple)
# Export GPIOs (replace numbers with actual GPIO numbers for PB2, PB3, PB4)
echo 34 > /sys/class/gpio/export   # DC (example: PB2 = 32 + 2 = 34)
echo 35 > /sys/class/gpio/export   # RST
echo 36 > /sys/class/gpio/export   # BUSY

echo out > /sys/class/gpio/gpio34/direction  # DC: output
echo out > /sys/class/gpio/gpio35/direction  # RST: output
echo in  > /sys/class/gpio/gpio36/direction  # BUSY: input
```

GPIO numbers on Allwinner chips: `PA0` = 0, `PB0` = 32, `PC0` = 64, etc. So `PB2` = 34.

## Display Init Sequence

The Waveshare 3.7" e-paper (SSD1677 controller, 480x280 pixels, 4-gray) has a specific initialization sequence.

### Reset

```
1. Set RST HIGH
2. Wait 10ms
3. Set RST LOW
4. Wait 10ms
5. Set RST HIGH
6. Wait 10ms
7. Wait for BUSY to go HIGH (ready)
```

### Init Commands

Send commands with DC=LOW, data bytes with DC=HIGH. CS goes LOW before each transaction, HIGH after.

The exact command sequence depends on the display controller variant. Use the Waveshare reference code as the canonical source:

- https://github.com/waveshare/e-Paper (look for the 3.7" example)

Key commands in the init sequence:

```
0x12  - Software reset
0x46  - Set RAM auto-increment
0x47  - Set RAM auto-increment
0x01  - Driver output control (set panel size: 279x479)
0x03  - Gate driving voltage
0x04  - Source driving voltage
0x3C  - Border waveform
0x22  - Display update control (load LUT from OTP, display)
0x20  - Activate display update sequence
```

Consult the SSD1677 datasheet for the full command set. The Waveshare example code is more practical than the datasheet.

## Partial vs Full Refresh

### Full Refresh (2-3 seconds)

- Flashes the screen black and white several times before drawing
- Clears all ghosting
- Use every N updates or when the display has accumulated ghosting

### Partial Refresh (200-300ms)

- Updates only changed regions (or the whole screen without the flash cycle)
- Ghosting accumulates over time
- Good for text updates, status changes

Strategy: use partial refresh for UI updates. Run a full refresh every 10-20 partial refreshes to clear ghosting.

## Framebuffer Layout

The 3.7" Waveshare display is 480x280 pixels with 4 grayscale levels (2 bits per pixel).

```
Memory layout: 480 * 280 / 4 = 33,600 bytes
Each byte holds 4 pixels: [px0_hi px0_lo px1_hi px1_lo px2_hi px2_lo px3_hi px3_lo]

Gray levels:
  0b11 = white
  0b10 = light gray
  0b01 = dark gray
  0b00 = black
```

You maintain a framebuffer in memory, render text/graphics into it, then send the whole buffer to the display.

## Font Rendering

For the prototype, use bitmap fonts. Simple, no dependencies.

```c
/* 8x16 bitmap font — each character is 16 bytes (8 pixels wide, 16 pixels tall) */
/* Full ASCII table is ~2KB. Plenty small. */

static const uint8_t font_8x16[128][16] = {
    /* ... glyph data ... */
    /* 'A' = 0x41 */
    [0x41] = { 0x00, 0x00, 0x10, 0x28, 0x28, 0x44, 0x44, 0x7C,
               0x82, 0x82, 0x82, 0x82, 0x00, 0x00, 0x00, 0x00 },
    /* ... */
};
```

For nicer rendering later, use a minimal FreeType wrapper. But bitmap fonts are fine for bring-up.

## Code Skeleton

This is illustrative C code showing the structure. It won't compile as-is — you need to fill in the actual SPI setup and command values from the Waveshare reference.

```c
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/spi/spidev.h>

#define EPD_WIDTH   480
#define EPD_HEIGHT  280
#define BUF_SIZE    (EPD_WIDTH * EPD_HEIGHT / 4)  /* 4 gray = 2bpp */

static int spi_fd;
static int gpio_dc_fd, gpio_rst_fd, gpio_busy_fd;
static uint8_t framebuffer[BUF_SIZE];

/* ---- Low-level SPI and GPIO ---- */

static void spi_init(void) {
    spi_fd = open("/dev/spidev0.0", O_RDWR);
    if (spi_fd < 0) { perror("open spidev"); exit(1); }

    uint8_t mode = SPI_MODE_0;
    uint32_t speed = 4000000;  /* 4 MHz */
    uint8_t bits = 8;

    ioctl(spi_fd, SPI_IOC_WR_MODE, &mode);
    ioctl(spi_fd, SPI_IOC_WR_BITS_PER_WORD, &bits);
    ioctl(spi_fd, SPI_IOC_WR_MAX_SPEED_HZ, &speed);
}

static void gpio_write(int fd, int value) {
    write(fd, value ? "1" : "0", 1);
}

static int gpio_read(int fd) {
    char buf[2];
    lseek(fd, 0, SEEK_SET);
    read(fd, buf, 1);
    return buf[0] == '1';
}

static void spi_send(const uint8_t *data, size_t len) {
    struct spi_ioc_transfer xfer = {
        .tx_buf = (unsigned long)data,
        .len = len,
    };
    ioctl(spi_fd, SPI_IOC_MESSAGE(1), &xfer);
}

/* ---- E-Paper Commands ---- */

static void epd_send_command(uint8_t cmd) {
    gpio_write(gpio_dc_fd, 0);   /* DC low = command */
    spi_send(&cmd, 1);
}

static void epd_send_data(const uint8_t *data, size_t len) {
    gpio_write(gpio_dc_fd, 1);   /* DC high = data */
    spi_send(data, len);
}

static void epd_wait_busy(void) {
    while (!gpio_read(gpio_busy_fd)) {
        usleep(10000);  /* 10ms polling */
    }
}

static void epd_reset(void) {
    gpio_write(gpio_rst_fd, 1);
    usleep(10000);
    gpio_write(gpio_rst_fd, 0);
    usleep(10000);
    gpio_write(gpio_rst_fd, 1);
    usleep(10000);
    epd_wait_busy();
}

static void epd_init(void) {
    epd_reset();

    epd_send_command(0x12);   /* Software reset */
    epd_wait_busy();

    /* Panel configuration commands — values from Waveshare reference */
    epd_send_command(0x01);   /* Driver output control */
    uint8_t doc[] = { 0x17, 0x01, 0x00 };  /* 280-1 = 0x117 */
    epd_send_data(doc, sizeof(doc));

    epd_send_command(0x3C);   /* Border waveform */
    uint8_t bw = 0x05;
    epd_send_data(&bw, 1);

    /* ... additional init commands per Waveshare example ... */

    epd_wait_busy();
}

/* ---- Framebuffer Operations ---- */

static void fb_clear(uint8_t color) {
    /* color: 0x00=black, 0x55=dark gray, 0xAA=light gray, 0xFF=white */
    memset(framebuffer, color, BUF_SIZE);
}

static void fb_set_pixel(int x, int y, uint8_t gray) {
    /* gray: 0=black, 1=dark gray, 2=light gray, 3=white */
    if (x < 0 || x >= EPD_WIDTH || y < 0 || y >= EPD_HEIGHT) return;

    int byte_idx = (y * EPD_WIDTH + x) / 4;
    int bit_offset = (3 - (x % 4)) * 2;

    framebuffer[byte_idx] &= ~(0x03 << bit_offset);
    framebuffer[byte_idx] |= (gray & 0x03) << bit_offset;
}

static void fb_draw_char(int x, int y, char ch, uint8_t gray) {
    /* Requires a bitmap font table — see font rendering section above */
    /* For each pixel in the glyph that's set, call fb_set_pixel() */
}

static void fb_draw_string(int x, int y, const char *str, uint8_t gray) {
    while (*str) {
        fb_draw_char(x, y, *str, gray);
        x += 8;  /* 8-pixel wide font */
        str++;
    }
}

/* ---- Display Update ---- */

static void epd_display(void) {
    /* Set RAM address counters to start */
    epd_send_command(0x4E);   /* Set RAM X address counter */
    uint8_t x_start = 0x00;
    epd_send_data(&x_start, 1);

    epd_send_command(0x4F);   /* Set RAM Y address counter */
    uint8_t y_start[] = { 0x00, 0x00 };
    epd_send_data(y_start, 2);

    /* Write framebuffer to RAM */
    epd_send_command(0x24);   /* Write RAM (black/white) */
    epd_send_data(framebuffer, BUF_SIZE);

    /* For 4-gray, also write to second RAM */
    epd_send_command(0x26);   /* Write RAM (red/gray) */
    epd_send_data(framebuffer, BUF_SIZE);

    /* Trigger display update */
    epd_send_command(0x22);   /* Display update control */
    uint8_t duc = 0xF7;       /* Full update */
    epd_send_data(&duc, 1);

    epd_send_command(0x20);   /* Activate display update sequence */
    epd_wait_busy();
}

/* ---- Main ---- */

int main(void) {
    spi_init();
    /* gpio_init() — open and configure GPIO fds for DC, RST, BUSY */

    epd_init();

    fb_clear(0xFF);  /* White background */
    fb_draw_string(10, 10, "Hello from Rune", 0x00);  /* Black text */
    epd_display();

    printf("Display updated. Check the e-ink panel.\n");

    close(spi_fd);
    return 0;
}
```

### Compile and Run

```bash
# In the Linux VM, cross-compile
arm-linux-gnueabihf-gcc -o epd_test epd_test.c

# Copy to device
scp epd_test root@rune.local:/tmp/

# Run on device
ssh root@rune.local /tmp/epd_test
```

## Test Sequence

1. **Wire check**: before powering on, verify all SPI connections with a multimeter on continuity mode.
2. **Power on**: display should be blank (whatever was last written, or random noise on first use).
3. **Run init + clear**: send the init sequence and clear to white. The display should flash and go white. If it does, SPI is working.
4. **Draw text**: write "Hello from Rune" and display. If text appears, you're done with bring-up.
5. **Partial refresh**: modify the framebuffer and update with partial refresh. Should be noticeably faster (200-300ms vs 2-3 seconds).

## Debugging

### Display stays blank

1. Check BUSY line — if stuck LOW, the display never finishes initializing. Check RST (did you pulse it correctly?).
2. Hook up the logic analyzer to SPI. Verify CS goes low, clock is running, data is present.
3. Check MOSI data against the expected init sequence from Waveshare reference code.
4. Verify DC pin toggling correctly (low for commands, high for data).

### Display shows garbage

1. Wrong init sequence — the pixel format, scan direction, or panel size doesn't match.
2. Framebuffer byte order might be swapped. Try flipping the bit packing.
3. SPI clock too fast. Drop to 1 MHz and test.

### Display partially updates then stops

1. BUSY line not being polled, or polled incorrectly. The display asserts BUSY during refresh.
2. CS might be going high mid-transfer if using large spidev writes. Try smaller chunks.

## Next Steps

Once "Hello from Rune" appears on the display:

1. Implement a proper text rendering module with word wrap.
2. Add a UI layout system (status bar, main content area, input indicator).
3. Implement partial refresh for the content area, full refresh for full redraws.
4. Build the display module into the main Rune application.
