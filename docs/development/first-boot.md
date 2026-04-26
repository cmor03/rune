# First Boot: Getting Linux on the T113

This guide gets you from bare MangoPi MQ-R to a running Linux system with network and SSH access.

## Step 1: Download a Linux Image

Two options:

### Option A: Tina Linux (Allwinner's official SDK)

MangoPi provides pre-built Tina Linux images:

- MangoPi MQ-R downloads page: https://mangopi.org/mqr
- MangoPi wiki: https://wiki.mangopi.org/

Tina Linux is based on OpenWrt. It works but is minimal and somewhat opaque. Good for initial bring-up.

### Option B: Community Buildroot/Mainline

The linux-sunxi community maintains mainline Linux support for Allwinner chips:

- linux-sunxi wiki (T113): https://linux-sunxi.org/T113-S4
- Community images and Buildroot configs can be found at: https://github.com/mangopi-sbc

For first boot, use whatever pre-built image is available. You can build your own Buildroot image later once you're familiar with the hardware.

### Download and Verify

```bash
# Example (replace with actual URL)
wget https://mangopi.org/downloads/mqr/tina-linux-sdcard.img.gz
gunzip tina-linux-sdcard.img.gz

# Verify the file looks right (should be several hundred MB to a few GB)
ls -lh tina-linux-sdcard.img
```

## Step 2: Flash the SD Card

On your Mac:

1. Insert the microSD card via USB-C card reader.
2. Open **balenaEtcher**.
3. Select the `.img` file.
4. Select the microSD card. **Double-check you selected the right drive.**
5. Click Flash.

Or use the command line:

```bash
# Find the disk (look for your SD card size)
diskutil list

# Unmount it (replace disk4 with your actual disk number)
diskutil unmountDisk /dev/disk4

# Write the image (use rdisk for raw device — much faster)
sudo dd if=tina-linux-sdcard.img of=/dev/rdisk4 bs=4m status=progress

# Eject
diskutil eject /dev/disk4
```

## Step 3: Connect the Serial Console

You need a serial connection to see boot output and get a shell. The MangoPi MQ-R exposes UART0 on header pins.

### Wiring

Connect the USB-TTL serial adapter to the MangoPi UART0 pins:

```
USB-TTL Adapter    MangoPi MQ-R
──────────────     ─────────────
TX          ──────── RX (UART0)
RX          ──────── TX (UART0)
GND         ──────── GND

DO NOT connect VCC/3.3V/5V from the serial adapter to the board.
The board has its own power supply. Connecting VCC can cause backfeed
and damage the serial adapter or the board.
```

Pin locations vary by board revision. Check the MangoPi MQ-R pinout diagram:
- Silkscreen labels on the board itself
- https://wiki.mangopi.org/ (pinout section)

Typically UART0 TX and RX are on the main GPIO header, clearly labeled.

### Pin Diagram (Common Layout)

```
MangoPi MQ-R GPIO Header (partial)
┌──────────────────────┐
│  ...                 │
│  GND ● ● 5V         │
│  TX0 ● ● RX0        │  ← UART0 (serial console)
│  ...                 │
└──────────────────────┘
```

Refer to the actual board silkscreen — this is illustrative.

## Step 4: Open the Serial Console

### macOS

```bash
# Find the serial device
ls /dev/tty.usbserial-*
# or
ls /dev/tty.wchusbserial-*

# Open with picocom (recommended)
picocom -b 115200 /dev/tty.usbserial-1410

# Or with screen
screen /dev/tty.usbserial-1410 115200
```

Replace the device path with whatever `ls` returned.

### Linux (inside VM, if serial adapter is passed through)

```bash
picocom -b 115200 /dev/ttyUSB0
```

### Settings

- Baud rate: 115200
- Data bits: 8
- Stop bits: 1
- Parity: None
- Flow control: None

These are the defaults for picocom at the specified baud rate.

**To exit picocom**: press `Ctrl+A`, then `Ctrl+X`.

**To exit screen**: press `Ctrl+A`, then `K`, then `Y`.

## Step 5: Power On and Watch Boot

1. Insert the flashed microSD card into the MangoPi.
2. Connect power (USB or your breadboard power circuit).
3. Watch the serial console.

You should see output like this:

```
[   0.000] boot0 is starting
[   0.002] DRAM: 128M
[   0.005] boot0: load bl31 ok
...
U-Boot SPL 2018.05-... (date)
...
U-Boot 2018.05-... (date)
...
Starting kernel ...
[    0.000000] Booting Linux on physical CPU 0x0
[    0.000000] Linux version 5.4.x ...
...
[    3.245] init started: BusyBox ...

root@MQ-R:~#
```

The boot sequence:
1. **SPL (Secondary Program Loader)**: first code to run, initializes DRAM
2. **U-Boot**: bootloader, loads kernel and device tree from SD card
3. **Linux kernel**: decompresses and starts, initializes drivers
4. **Init**: userspace starts (BusyBox init, systemd, or OpenWrt's procd)

## Step 6: Login

### Tina Linux

```
Username: root
Password: (empty — just press Enter)
```

Or:

```
Username: tina
Password: tina
```

Depends on the image version. Try root with no password first.

### Community/Buildroot Images

Usually root with no password.

If you get a login prompt and can't figure out the password, re-flash with a different image or build your own with a known root password.

## Step 7: Configure WiFi

The MangoPi MQ-R has an onboard RTL8723DS WiFi/BT chip.

```bash
# Check if the WiFi interface exists
ip link show wlan0

# If wlan0 doesn't show up, the firmware blob might be missing.
# Check for the firmware:
ls /lib/firmware/rtl8723ds/

# Scan for networks (if wpa_supplicant is available)
wpa_cli scan
wpa_cli scan_results

# Configure WiFi
wpa_passphrase "YourSSID" "YourPassword" > /etc/wpa_supplicant.conf

# Connect
wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant.conf

# Get an IP address
udhcpc -i wlan0

# Verify
ping -c 3 8.8.8.8

# Check your IP address
ip addr show wlan0
```

### Make WiFi Persistent Across Reboots

On Tina Linux / OpenWrt:

```bash
# Edit the wireless config
vi /etc/config/wireless
```

On Buildroot / systemd:

```bash
# Create wpa_supplicant service
cat > /etc/wpa_supplicant/wpa_supplicant-wlan0.conf << EOF
network={
    ssid="YourSSID"
    psk="YourPassword"
}
EOF

# Enable at boot (if systemd)
systemctl enable wpa_supplicant@wlan0
systemctl enable dhcpcd
```

## Step 8: Enable SSH

### If Using Dropbear (Common on Minimal Images)

```bash
# Start dropbear SSH server
dropbear

# Or if it's a service
/etc/init.d/dropbear start
/etc/init.d/dropbear enable  # start on boot
```

### If Using OpenSSH

```bash
# Start SSH server
/etc/init.d/sshd start
# or
systemctl start sshd
systemctl enable sshd
```

### Verify from Your Mac

```bash
# Replace with the IP from step 7
ssh root@192.168.1.XXX

# Or if mDNS/avahi is running on the device
ssh root@mangopi.local
```

## Step 9: Set Up Key-Based SSH

On your Mac:

```bash
# Generate a key if you don't have one
ls ~/.ssh/id_ed25519 || ssh-keygen -t ed25519

# Copy to the device
ssh-copy-id root@192.168.1.XXX

# Test passwordless login
ssh root@192.168.1.XXX
```

If `ssh-copy-id` isn't available on the device (minimal image), do it manually:

```bash
# On your Mac
cat ~/.ssh/id_ed25519.pub | ssh root@192.168.1.XXX "mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys && chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys"
```

### Set a Hostname

On the device:

```bash
echo "rune" > /etc/hostname
hostname rune
```

Now you can (after mDNS is set up or `/etc/hosts` is configured):

```bash
ssh root@rune.local
```

## Troubleshooting

### Board Hangs at SPL / Nothing After Power On

- **Bad SD card**: try a different card. Cheap cards are unreliable. Use Samsung EVO or SanDisk Ultra.
- **Bad image**: re-download and re-flash. Verify the image size looks right.
- **Wrong image format**: make sure you're flashing the `.img` file, not a `.img.gz` compressed file.
- **Card not seated**: push the microSD in until it clicks.

### No UART Output at All

- **TX/RX swapped**: swap the TX and RX wires between serial adapter and board.
- **Wrong baud rate**: confirm 115200. Try 9600 or 57600 if you see garbled output.
- **Serial adapter not recognized**: check `ls /dev/tty.*serial*` on macOS. Install CH340 driver if needed (see `mac-setup.md`).
- **Wrong pins**: verify you're on UART0, not UART1/2/3. Check the pinout diagram.
- **No power**: is the board actually powered? Check for an LED on the MangoPi.

### Garbled Serial Output

- **Baud rate mismatch**: the most common cause. Must be 115200.
- **Loose wires**: reseat the jumper wires. Breadboard connections are unreliable.

### Kernel Panic

- **Wrong device tree**: the DTB doesn't match the board. Use the image built for MangoPi MQ-R specifically.
- **Corrupted filesystem**: re-flash the SD card.
- **Missing rootfs**: U-Boot found the kernel but not the root filesystem. Check the partition layout of the image.

### WiFi Not Connecting

- **RTL8723DS firmware missing**: the WiFi chip needs a firmware blob. Check:
  ```bash
  dmesg | grep rtl8723
  ls /lib/firmware/rtl*
  ```
  If the firmware files are missing, you need to add them to your image or copy them manually.
- **No wlan0 interface**: driver not loaded. Check `dmesg | grep wlan` for errors.
- **Wrong SSID/password**: double-check with `wpa_passphrase`.
- **2.4GHz only**: the RTL8723DS is 2.4GHz only. Make sure your network has a 2.4GHz band.

### Board Doesn't Boot from SD at All (Goes to FEL Mode)

The T113 tries to boot from SD first. If it fails, it falls back to FEL mode (USB recovery). You'll see nothing on serial, but the board shows up as a USB device:

```bash
# On macOS
xfel version
# If you see "AWUSBFEX soc=... " then the board is in FEL mode
```

This means the SD card image isn't being recognized. Re-flash with a known good image.
