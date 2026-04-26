# Iteration Workflow

The development inner loop: how to go from code change to seeing results on the device.

## Fast Loop (Seconds)

This is the most common workflow. You edit code on your Mac, cross-compile in the Linux VM, and deploy to the device over SSH.

```
Edit on Mac → Cross-compile in VM → SCP to device → Restart service → See changes
```

### Commands

```bash
# In the Linux VM (orb shell rune-dev)

# Build the Rune binary for ARM
cargo build --target armv7-unknown-linux-gnueabihf --release

# Copy to device
scp target/armv7-unknown-linux-gnueabihf/release/rune root@rune.local:/usr/bin/rune

# Restart the service on the device
ssh root@rune.local systemctl restart rune
```

Total time: 5-15 seconds for an incremental build + deploy.

### One-Liner

```bash
cargo build --target armv7-unknown-linux-gnueabihf --release && \
scp target/armv7-unknown-linux-gnueabihf/release/rune root@rune.local:/usr/bin/rune && \
ssh root@rune.local systemctl restart rune
```

### Makefile Target

Add this to your Makefile so you can just run `make deploy`:

```makefile
DEVICE_HOST := root@rune.local
DEVICE_BIN := /usr/bin/rune
TARGET := armv7-unknown-linux-gnueabihf

.PHONY: build deploy watch

build:
	cargo build --target $(TARGET) --release

deploy: build
	scp target/$(TARGET)/release/rune $(DEVICE_HOST):$(DEVICE_BIN)
	ssh $(DEVICE_HOST) systemctl restart rune

# ESP32 firmware
esp32-flash:
	cd esp32/rune_coprocessor && idf.py build flash
```

### Auto-Rebuild on Save

Use `inotifywait` (Linux) or `fswatch` (macOS) to automatically rebuild when files change:

```bash
# In the Linux VM — rebuild on any .rs file change
sudo apt install -y inotify-tools

while inotifywait -r -e modify --include '\.rs$' /Users/coltonmorris/Desktop/rune/repo/rune/firmware/src/; do
    cargo build --target armv7-unknown-linux-gnueabihf --release && \
    scp target/armv7-unknown-linux-gnueabihf/release/rune root@rune.local:/usr/bin/rune && \
    ssh root@rune.local systemctl restart rune
    echo "--- deployed ---"
done
```

Or on macOS with fswatch:

```bash
brew install fswatch

fswatch -o firmware/src/ | while read; do
    orb shell rune-dev -- bash -c 'cd /path/to/repo && cargo build --target armv7-unknown-linux-gnueabihf --release' && \
    # ... scp and restart
done
```

## Medium Loop (Minutes)

When you need to change the device tree, kernel config, or system image contents.

```
Edit device tree or config → Rebuild in Buildroot → Flash new SD image → Reboot device
```

### Commands

```bash
# In the Linux VM, inside the Buildroot directory

# After modifying a device tree or package config
make

# This rebuilds only what changed. Usually 1-5 minutes for device tree changes.
# Full rebuilds (kernel config change) take 10-30 minutes.

# The output image is at:
# output/images/sdcard.img

# Copy image to macOS for flashing
cp output/images/sdcard.img /Users/coltonmorris/Desktop/rune/

# On macOS: flash with balenaEtcher or dd
# Then insert the SD card into the device and power cycle
```

### Tips

- Keep the old SD card as a backup. If the new image doesn't boot, swap back.
- Label your SD cards (masking tape + pen) with the image date/version.
- If you're only changing the device tree, you can sometimes copy just the DTB file to the SD card's boot partition without reflashing the entire image:

```bash
# Mount the SD card's boot partition on macOS
# Replace disk4s1 with the actual partition
diskutil mount /dev/disk4s1
cp sun8i-t113s-mangopi-mq-r.dtb /Volumes/BOOT/
diskutil unmount /dev/disk4s1
```

## Slow Loop (Rare)

Full Buildroot rebuild from scratch. This happens when you change the kernel config, add new packages, or update the toolchain.

```bash
# In the Linux VM
make clean  # or rm -rf output/ for truly clean
make

# Takes 30-60 minutes on first build, depending on CPU and network speed
```

Avoid this loop. Structure your changes so they fit in the fast or medium loop. If you need to add a kernel module, see if you can build it out-of-tree first and test before committing to a full rebuild.

## Recovery Loop

When the device won't boot at all — bad SD card, corrupted image, bricked bootloader.

### FEL Mode Recovery

Allwinner chips have a built-in FEL mode: a USB recovery protocol that works even when the SD card is empty or corrupted.

```bash
# On macOS

# 1. Remove the SD card from the device
# 2. Connect the device to your Mac via USB (OTG port)
# 3. Power on the device. With no bootable SD card, it enters FEL mode automatically.

# 4. Verify FEL mode
xfel version
# Should output: AWUSBFEX soc=... (T113-S4)

# 5. Write a new bootloader and kernel to SPI flash or DRAM
xfel ddr d1           # Init DRAM (d1 for T113-S4/D1)
xfel write 0x40000000 u-boot.bin    # Write U-Boot to DRAM
xfel exec  0x40000000               # Execute U-Boot from DRAM
```

Once U-Boot is running from DRAM, you can boot a kernel over USB or reflash the SD card from within U-Boot.

### If FEL Mode Doesn't Work

1. Check that the USB cable supports data (not charge-only).
2. Try a different USB port on your Mac.
3. Check `xfel` is installed correctly: `xfel version` with no device should show a "not found" error, not a crash.
4. Some MangoPi revisions need a specific button held during power-on to enter FEL. Check the MangoPi wiki.

## Tips and Workflow Optimizations

### SSH Key Auth

Set this up first. Typing passwords on every deploy kills your flow.

```bash
# On macOS, once
ssh-copy-id root@rune.local
```

### Hostname Resolution

If `rune.local` doesn't resolve (mDNS not running on the device), add a static entry:

```bash
# On macOS
echo "192.168.1.XXX rune.local" | sudo tee -a /etc/hosts
```

Replace the IP with the device's actual IP.

### Multiple SD Cards

Keep at least 3 SD cards:

1. **Working image**: known-good, stable. Never modify.
2. **Development image**: current work-in-progress.
3. **Experimental**: for risky changes (kernel mods, bootloader changes).

If development goes wrong, swap in the working image and you're back in business in 30 seconds.

### Remote Debug Logging

Instead of connecting a serial console every time, use SSH and journald/syslog:

```bash
# Follow logs in real time
ssh root@rune.local journalctl -f -u rune

# Or if using syslog
ssh root@rune.local tail -f /var/log/messages
```

### Quick Test Without systemctl

During rapid iteration, skip the service manager. Just kill and restart the binary directly:

```bash
ssh root@rune.local "killall rune; /usr/bin/rune &"

# Or for foreground with logs
ssh root@rune.local "killall rune; /usr/bin/rune"
# Ctrl+C to stop
```

### ESP32 Iteration

The ESP32 has its own build/flash loop, independent of the T113:

```bash
# On macOS (or in VM if you set up esp-idf there)
cd esp32/rune_coprocessor
idf.py build flash monitor
```

The ESP32-S3-DevKitC-1 flashes over USB-C in about 10 seconds. `monitor` shows serial output immediately after flash, so you see the boot and your debug prints.

### When Everything Is Broken

1. Breathe.
2. Go back to basics: can you get a serial console? If not, fix that first.
3. Boot a known-good SD card.
4. Test one subsystem at a time.
5. Check the obvious: power, ground, TX/RX crossover.
6. Use the logic analyzer. It shows you what's actually happening, not what you think is happening.
