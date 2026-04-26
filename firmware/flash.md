# Flashing the Rune Device

Multiple methods depending on what you're doing. For day-to-day development, use SSH. For recovery or initial setup, use SD card or FEL mode.

## Method 1: Hot-Reload over SSH (Development)

The fastest path during development. Requires the device to be booted and on the network.

### Push the binary

```bash
scp target/armv7-unknown-linux-gnueabihf/release/rune root@rune.local:/usr/bin/rune
```

### Restart the service

```bash
ssh root@rune.local systemctl restart rune
```

### One-liner

```bash
scp target/armv7-unknown-linux-gnueabihf/release/rune root@rune.local:/usr/bin/rune && ssh root@rune.local systemctl restart rune
```

This is the preferred workflow. Build on your dev machine, push over the network, restart. Takes seconds.

## Method 2: SD Card Swap

For flashing a complete system image (kernel + rootfs + bootloader). Use this for initial device setup or when the rootfs itself has changed.

### 1. Build the full image

```bash
cd firmware/buildroot/
make
```

This produces an SD card image (e.g., `output/images/sdcard.img`).

### 2. Flash to SD card

Using balenaEtcher (GUI) or dd (command line):

```bash
# Find your SD card device -- BE CAREFUL, wrong device = data loss
lsblk

# Flash (replace /dev/sdX with your actual SD card device)
sudo dd if=output/images/sdcard.img of=/dev/sdX bs=4M status=progress conv=fsync
```

### 3. Insert and boot

Put the SD card in the Rune device and power on. First boot may take longer as the system initializes.

## Method 3: FEL Mode Recovery

FEL is Allwinner's USB boot mode. Use this when the device won't boot from SD card, or for initial bootloader programming.

### Enter FEL mode

1. Power off the device
2. Hold the **FEL button**
3. Connect USB cable to your computer
4. Release the FEL button after connecting

### Install xfel

```bash
# From source
git clone https://github.com/xboot/xfel.git
cd xfel
make
sudo make install

# Or on Arch Linux
yay -S xfel
```

### Verify FEL connection

```bash
xfel version
```

Should print the chip ID (e.g., `AWUSBFEX ID=0x00185900(T113-S4)`).

### Write SPL (Secondary Program Loader)

```bash
xfel spinor write 0x0 output/images/sunxi-spl.bin
```

### Write U-Boot

```bash
xfel spinor write 0x40000 output/images/u-boot.bin
```

### Write kernel and rootfs to SPI NOR (if applicable)

```bash
xfel spinor write 0x100000 output/images/uImage
xfel spinor write 0x500000 output/images/rootfs.squashfs
```

### Alternative: Write entire SD card image via FEL

```bash
xfel sd write 0 output/images/sdcard.img
```

### Reset the device

```bash
xfel reset
```

The device should now boot from the freshly written image.

## Bootloader Recovery

If the bootloader is corrupted and the device won't enter FEL mode normally, you're in worst-case territory.

1. Remove the SD card
2. Enter FEL mode (without SD card, the T113 should fall back to FEL automatically if SPI NOR is empty/corrupted)
3. Use `xfel` to reflash SPL and U-Boot as described above
4. Flash a fresh SD card image and boot from that

If nothing works, reflash the entire SD card from scratch using Method 2.

## Flashing the ESP32

The ESP32-S3 connects via USB (or UART depending on your board revision).

### Using esp-idf

```bash
cd firmware/esp32/
idf.py -p /dev/ttyUSB0 flash
```

### Using esptool.py directly

```bash
esptool.py --chip esp32s3 --port /dev/ttyUSB0 --baud 460800 \
  write_flash -z \
  0x0 build/bootloader/bootloader.bin \
  0x8000 build/partition_table/partition-table.bin \
  0x10000 build/rune_esp32.bin
```

### Monitor serial output

```bash
idf.py -p /dev/ttyUSB0 monitor
```

## Production Flashing

For batch flashing multiple devices, use a scripted FEL workflow:

```bash
#!/bin/bash
# production_flash.sh -- flash a Rune device via FEL
set -e

echo "Waiting for device in FEL mode..."
while ! xfel version 2>/dev/null; do
    sleep 1
done

echo "Device found. Flashing..."
xfel spinor write 0x0 sunxi-spl.bin
xfel spinor write 0x40000 u-boot.bin
xfel sd write 0 sdcard.img
xfel reset

echo "Done. Device should boot momentarily."
```

Connect each device in FEL mode, run the script, repeat.
