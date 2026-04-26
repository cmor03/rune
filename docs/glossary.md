# Glossary

Technical terms used throughout the Rune documentation.

---

**ALSA** -- Advanced Linux Sound Architecture. The kernel-level audio subsystem on Linux. Rune uses ALSA to capture audio from the I2S microphone and play audio through the I2S amplifier.

**AXP2101** -- A power management IC (PMIC) from X-Powers. Manages battery charging, voltage regulation, and power sequencing for the T113-S4 and ESP32-S3. Communicates with the T113 over I2C.

**BLE** -- Bluetooth Low Energy. Version 5.0, handled by the ESP32-S3. Used for phone notification sync (ANCS on iOS, GATT on Android) and optional file transfers.

**Buildroot** -- A build system for generating embedded Linux root filesystems. Rune uses Buildroot to produce a minimal Linux image for the T113-S4 -- no systemd, no desktop, just BusyBox and the userspace application.

**Device Tree** -- A data structure that describes hardware to the Linux kernel. Instead of hard-coding board-specific details in the kernel, a device tree file tells the kernel what peripherals exist, how they're connected, and how to configure them.

**DTS/DTB** -- Device Tree Source / Device Tree Blob. DTS is the human-readable source file. DTB is the compiled binary that the bootloader passes to the kernel. Rune's DTS defines pin muxing, SPI, I2S, UART, USB, and GPIO configurations for the T113-S4 board.

**E-ink / E-paper** -- An electrophoretic display technology. Tiny charged particles (black and white) move in response to an electric field to form an image. The image persists without power. Rune uses a 3.7" monochrome e-ink display at 480x280 pixels, driven over SPI.

**ESP32-S3** -- A WiFi and Bluetooth SoC from Espressif. In Rune, it serves as a co-processor handling all wireless communication. It runs its own firmware on FreeRTOS (via ESP-IDF) and communicates with the T113-S4 over UART.

**FEL mode** -- A USB boot mode built into Allwinner SoCs. When the T113-S4 enters FEL mode, it exposes a USB endpoint that accepts commands from a host computer. Used for initial firmware flashing, recovery, and low-level debugging. Accessed via the `xfel` tool.

**I2C** -- Inter-Integrated Circuit. A two-wire serial bus (SDA + SCL) used for low-speed peripheral communication. In Rune, I2C connects the T113-S4 to the AXP2101 PMIC.

**I2S** -- Inter-IC Sound. A serial bus protocol designed for digital audio data. Rune uses a single I2S bus on the T113-S4, shared between the MEMS microphone (input) and the MAX98357A amplifier (output).

**INMP441** -- A MEMS microphone with an I2S digital output. One of two mic options for Rune (the other being the SPH0645). Captures audio for voice queries.

**KiCad** -- An open-source electronics design automation suite. Rune's schematics and PCB layout are designed in KiCad. Source files are in the `hardware/` directory.

**librespot** -- An open-source Spotify Connect client written in Rust. Rune uses librespot to stream music from Spotify without the official Spotify app. Requires a Spotify Premium account.

**MAX98357A** -- A Class D audio amplifier with an I2S input, from Analog Devices (formerly Maxim). Takes digital audio over I2S and drives Rune's mono speaker directly. No DAC needed -- the I2S-to-analog conversion is built in.

**picocom** -- A minimal terminal emulation program for serial ports. Used during Rune development to connect to the T113-S4's UART console for debugging and log output.

**PMIC** -- Power Management Integrated Circuit. A chip that manages power distribution, battery charging, and voltage regulation. Rune uses the AXP2101.

**SPI** -- Serial Peripheral Interface. A synchronous serial bus with clock, MOSI, MISO, and chip select lines. Rune uses SPI to drive the e-ink display controller. Accessed from userspace via the `spidev` kernel driver.

**spidev** -- The Linux kernel's userspace SPI driver. Exposes SPI peripherals as `/dev/spidevX.X` device files. The Rune userspace application uses spidev to send commands and framebuffer data to the e-ink display without needing a kernel-space display driver.

**SPH0645** -- A MEMS microphone with an I2S digital output, from Knowles. One of two mic options for Rune (the other being the INMP441).

**T113-S4** -- An application processor from Allwinner. Dual-core ARM Cortex-A7 @ 1.2GHz with 128MB DDR3 integrated into the package. Runs Linux. This is Rune's main processor, handling display rendering, audio processing, ePub layout, and application logic.

**UART** -- Universal Asynchronous Receiver-Transmitter. A serial communication protocol using TX and RX lines. Rune uses UART for communication between the T113-S4 and ESP32-S3 (default 115200 baud, upgradeable to 921600).

**USB** -- Universal Serial Bus. Rune uses USB internally to connect the UVC camera module to the T113-S4's USB host port. The external USB-C connector provides charging and FEL mode access.

**UVC** -- USB Video Class. A standard USB device class for video capture devices. Rune's camera module is a UVC-compliant device, which means it works with the standard Linux V4L2 video subsystem without a custom driver.

**V4L2** -- Video for Linux 2. The Linux kernel's video capture and output API. Rune uses V4L2 to capture still frames from the UVC camera module.

**xfel** -- A command-line tool for communicating with Allwinner SoCs in FEL mode. Used to flash firmware, write to SPI NOR/NAND, load boot images, and perform low-level hardware testing during Rune development and manufacturing.
