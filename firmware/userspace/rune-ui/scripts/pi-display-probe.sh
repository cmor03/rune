#!/bin/sh
set -eu

echo "== model =="
tr -d '\0' </proc/device-tree/model 2>/dev/null || true
echo

echo "== kernel =="
uname -a
echo

echo "== fbdev =="
ls -l /dev/fb* 2>/dev/null || echo "no /dev/fb* devices"
echo

echo "== fb sysfs =="
found_fb=0
for fb in /sys/class/graphics/fb*; do
    [ -e "$fb" ] || continue
    found_fb=1
    echo "$fb"
    printf "  name: "
    cat "$fb/name" 2>/dev/null || true
    printf "  virtual_size: "
    cat "$fb/virtual_size" 2>/dev/null || true
    printf "  stride: "
    cat "$fb/stride" 2>/dev/null || true
    printf "  bits_per_pixel: "
    cat "$fb/bits_per_pixel" 2>/dev/null || true
done
[ "$found_fb" -eq 1 ] || echo "no framebuffer sysfs entries"
echo

echo "== drm =="
ls -l /dev/dri/ 2>/dev/null || echo "no /dev/dri devices"
echo

echo "== spi =="
ls -l /dev/spidev* 2>/dev/null || echo "no /dev/spidev devices"
echo

echo "== configured overlays =="
config=/boot/firmware/config.txt
[ -f "$config" ] || config=/boot/config.txt
echo "config: $config"
grep -nE '^(dtparam|dtoverlay)' "$config" 2>/dev/null || echo "no dtparam/dtoverlay lines found"
echo

echo "== display-ish dmesg =="
dmesg | grep -Ei 'fb|drm|spi|st77|ili|waveshare|panel|lcd|fbtft|tinydrm|mipi|dsi' | tail -160 || true
echo

echo "== interpretation =="
if ls /dev/fb* >/dev/null 2>&1; then
    echo "fbdev display device found. Try rune-ui-demo --backend fbdev with the matching /dev/fbN."
    fb=/sys/class/graphics/fb0
    if [ -e "$fb" ]; then
        size=$(cat "$fb/virtual_size" 2>/dev/null || true)
        bpp=$(cat "$fb/bits_per_pixel" 2>/dev/null || true)
        stride=$(cat "$fb/stride" 2>/dev/null || true)
        echo
        echo "fb0 quick read:"
        echo "  size: ${size:-unknown}"
        echo "  bpp: ${bpp:-unknown}"
        echo "  stride: ${stride:-unknown}"
        echo
        case "$size:$bpp" in
            320,480:16)
                echo "recommended Rune command:"
                echo "  rune-ui --screen home --backend fbdev --fb /dev/fb0 --format rgb565 --fb-width 320 --fb-height 480 --stride ${stride:-640}"
                ;;
            320,960:16)
                echo "recommended Rune command:"
                echo "  rune-ui --screen home --backend fbdev --fb /dev/fb0 --format rgb565 --fb-width 320 --fb-height 480 --stride ${stride:-640}"
                echo "note: virtual height is doubled; use the visible 320x480 size to avoid stacked frames."
                ;;
            480,320:16)
                echo "recommended Rune command:"
                echo "  rune-ui --screen home --backend fbdev --fb /dev/fb0 --format rgb565 --rotate 90 --fb-width 480 --fb-height 320 --stride ${stride:-960}"
                ;;
            320,480:32)
                echo "recommended Rune command:"
                echo "  rune-ui --screen home --backend fbdev --fb /dev/fb0 --format xrgb8888 --fb-width 320 --fb-height 480 --stride ${stride:-1280}"
                ;;
            320,960:32)
                echo "recommended Rune command:"
                echo "  rune-ui --screen home --backend fbdev --fb /dev/fb0 --format xrgb8888 --fb-width 320 --fb-height 480 --stride ${stride:-1280}"
                echo "note: virtual height is doubled; use the visible 320x480 size to avoid stacked frames."
                ;;
            480,320:32)
                echo "recommended Rune command:"
                echo "  rune-ui --screen home --backend fbdev --fb /dev/fb0 --format xrgb8888 --rotate 90 --fb-width 480 --fb-height 320 --stride ${stride:-1920}"
                ;;
            *)
                echo "No exact recommendation for this geometry yet. Use visible width, visible height, bpp-derived format, and sysfs stride."
                ;;
        esac
    fi
elif ls /dev/dri/card* >/dev/null 2>&1; then
    echo "DRM devices found, but no fbdev display. If no connector/size is active, the LCD overlay is probably missing."
elif ls /dev/spidev* >/dev/null 2>&1; then
    echo "SPI is available, but no Linux display device exists. The LCD likely needs a dtoverlay or a direct SPI driver."
else
    echo "No display or SPI devices found. Enable SPI and load the LCD driver/overlay first."
fi
