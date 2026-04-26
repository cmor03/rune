# Camera Bringup: USB UVC

Getting the USB camera working with the T113-S4.

## Hardware Setup

1. Plug the USB UVC camera module into the MangoPi's USB host port.
2. If the MangoPi only has a micro-USB OTG port, use a USB OTG adapter (micro-USB male to USB-A female) to switch it to host mode.

That's it for wiring. USB UVC is plug-and-play at the hardware level.

## Verify Detection

### Check USB Device

```bash
lsusb
```

You should see an entry for the camera, something like:

```
Bus 001 Device 003: ID 0c45:6366 Microdia USB 2.0 Camera
```

The exact vendor/product ID depends on your camera module. If nothing appears, see troubleshooting below.

### Check Video Device Node

```bash
ls /dev/video*
```

You should see `/dev/video0` (and possibly `/dev/video1` for metadata).

### Check Kernel Messages

```bash
dmesg | tail -20
```

After plugging in, you should see:

```
usb 1-1: new high-speed USB device number 3 using ehci-platform
uvcvideo: Found UVC 1.00 device USB Camera (0c45:6366)
input: USB Camera as /devices/...
```

## Query Camera Capabilities

```bash
# Full device info
v4l2-ctl --device=/dev/video0 --all

# List supported formats
v4l2-ctl --device=/dev/video0 --list-formats-ext
```

Typical output:

```
ioctl: VIDIOC_ENUM_FMT
    Type: Video Capture

    [0]: 'MJPG' (Motion-JPEG, compressed)
        Size: Discrete 640x480
            Interval: Discrete 0.033s (30.000 fps)
        Size: Discrete 1280x720
            Interval: Discrete 0.033s (30.000 fps)
    [1]: 'YUYV' (YUYV 4:2:2)
        Size: Discrete 640x480
            Interval: Discrete 0.033s (30.000 fps)
```

MJPG is preferred — it's compressed, so it uses less USB bandwidth and is faster to transfer.

## Capture a Test Frame

### Using v4l2-ctl

```bash
# Set format to MJPEG 640x480
v4l2-ctl --device=/dev/video0 \
  --set-fmt-video=width=640,height=480,pixelformat=MJPG

# Capture one frame to a JPEG file
v4l2-ctl --device=/dev/video0 \
  --stream-mmap \
  --stream-count=1 \
  --stream-to=/tmp/test.jpg

# Check the file
ls -la /tmp/test.jpg
# Should be a few KB to a few hundred KB depending on scene complexity
```

### Copy to Your Mac and View

```bash
# On your Mac
scp root@rune.local:/tmp/test.jpg ~/Desktop/
open ~/Desktop/test.jpg
```

If the image looks correct, the camera is working.

### Using ffmpeg (If Available)

```bash
# Capture a single frame
ffmpeg -f v4l2 -input_format mjpeg -video_size 640x480 \
  -i /dev/video0 -frames:v 1 /tmp/test.jpg
```

## V4L2 API Integration

For programmatic capture, use the V4L2 (Video4Linux2) API directly. This is the standard Linux video capture interface.

### Basic Capture Flow

```
1. open("/dev/video0", O_RDWR)
2. VIDIOC_S_FMT    — set format (MJPEG, 640x480)
3. VIDIOC_REQBUFS  — request memory-mapped buffers
4. VIDIOC_QUERYBUF — get buffer addresses
5. mmap()          — map buffers to userspace
6. VIDIOC_QBUF     — queue buffers
7. VIDIOC_STREAMON  — start capture
8. poll() / select() — wait for frame
9. VIDIOC_DQBUF   — dequeue filled buffer (this is your JPEG frame)
10. Process the frame
11. VIDIOC_QBUF    — re-queue buffer
12. Repeat 8-11 for more frames
13. VIDIOC_STREAMOFF — stop capture
14. munmap(), close()
```

For Rune, we only need single frame capture — not continuous streaming. So the flow is:

```
open → set format → request buffers → mmap → queue → stream on →
wait → dequeue (grab frame) → stream off → close
```

### Rust Crate

If writing the firmware in Rust, the `v4l` crate wraps V4L2:

```rust
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

fn capture_frame() -> Vec<u8> {
    let dev = Device::new(0).expect("failed to open /dev/video0");

    let mut fmt = dev.format().expect("failed to get format");
    fmt.width = 640;
    fmt.height = 480;
    fmt.fourcc = FourCC::new(b"MJPG");
    dev.set_format(&fmt).expect("failed to set format");

    let mut stream = Stream::with_buffers(&dev, Type::VideoCapture, 4)
        .expect("failed to create stream");

    let (buf, _meta) = stream.next().expect("failed to capture frame");
    buf.to_vec()
}
```

## Camera Settings

### Exposure

Most UVC cameras default to auto-exposure. If images are too dark or bright:

```bash
# List available controls
v4l2-ctl --device=/dev/video0 --list-ctrls

# Set manual exposure
v4l2-ctl --device=/dev/video0 --set-ctrl=auto_exposure=1
v4l2-ctl --device=/dev/video0 --set-ctrl=exposure_time_absolute=200

# Or force auto-exposure
v4l2-ctl --device=/dev/video0 --set-ctrl=auto_exposure=3
```

### White Balance

```bash
v4l2-ctl --device=/dev/video0 --set-ctrl=white_balance_automatic=1
```

For Rune's use case (simple photo capture, not video), auto settings are fine.

## Common Issues

### No /dev/video0

1. **USB host not enabled**: the T113's USB port might be in OTG mode by default. Check the device tree for USB host enable:
   ```dts
   &usb_otg {
       dr_mode = "host";  /* or "otg" */
       status = "okay";
   };
   ```
2. **Need OTG cable**: if using the micro-USB port, you need an OTG adapter. Without it, the port stays in device mode.
3. **UVC driver not built**: check kernel config for `CONFIG_USB_VIDEO_CLASS=y` (or `=m`). If module, `modprobe uvcvideo`.
4. **USB power insufficient**: some camera modules draw too much current. The MangoPi's USB port might not supply enough. Try a powered USB hub.

### Permission Denied

```bash
# Quick fix — run as root
# Proper fix — add udev rule
cat > /etc/udev/rules.d/99-video.rules << 'EOF'
SUBSYSTEM=="video4linux", MODE="0666"
EOF
udevadm trigger
```

### Image Is Dark / Black

1. Camera needs time to adjust auto-exposure. Capture a few frames and discard the first 3-5.
2. Check exposure settings: `v4l2-ctl --device=/dev/video0 --get-ctrl=exposure_time_absolute`
3. Lens cap still on (yes, really — some modules have a protective film).

### Image Has Horizontal Lines / Tearing

1. USB bandwidth issue — try lower resolution.
2. Switch from YUYV (uncompressed) to MJPEG (compressed). YUYV at high resolutions can exceed USB 2.0 bandwidth.

### Camera Not UVC-Compliant

If `dmesg` shows the USB device but no UVC driver binding, the camera might not be UVC-compliant. Only UVC cameras work with the generic driver. Check the camera's spec sheet. Any camera advertised as "USB UVC" or "driverless" should work.

## Design Note

The camera in Rune is for simple, on-demand photo capture — not video streaming or real-time processing. The T113-S4 is not a video processing chip. The usage pattern is:

1. User triggers a photo (e.g., "take a picture of this").
2. Rune captures a single JPEG frame.
3. Frame is sent to the AI provider as part of the query.
4. Response is displayed on e-ink / spoken through speaker.

Single frame capture latency is typically under 100ms. This is fine for the use case.
