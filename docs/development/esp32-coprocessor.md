# ESP32-S3 Co-Processor Setup

The ESP32-S3 handles BLE, WiFi fallback, and future peripheral expansion.

## Why a Co-Processor

The T113-S4 has WiFi (RTL8723DS) but no BLE hardware. The ESP32-S3 fills this gap:

1. **BLE for phone notifications**: pair with a phone, receive notification mirroring (Apple ANCS / Android GATT), forward to T113 over UART.
2. **WiFi fallback**: if the T113's onboard RTL8723DS WiFi is unreliable (it can be), the ESP32 can bridge WiFi over UART.
3. **Future expansion**: cellular modem (SIM7600 over UART), additional sensors, Zigbee/Thread with an 802.15.4 radio.

The T113 runs the main application, UI, and voice pipeline. The ESP32 is a peripheral that responds to commands from the T113.

## Hardware Setup

### UART Wiring

| MangoPi Pin (T113) | ESP32-S3 Pin | Function |
|---------------------|--------------|----------|
| UART2_TX            | GPIO44 (RX)  | T113 transmit → ESP32 receive |
| UART2_RX            | GPIO43 (TX)  | ESP32 transmit → T113 receive |
| GND                 | GND          | Common ground |

TX/RX crossover: T113 TX to ESP32 RX, T113 RX to ESP32 TX. Run an explicit GND wire between boards even if they share a common power rail.

Do not connect VCC/3.3V between the boards. Each board uses its own 3.3V regulator.

### UART Settings

- Baud rate: 115200 (default, increase to 921600 once everything works)
- Data bits: 8
- Stop bits: 1
- Parity: None
- Flow control: None

## Flashing the ESP32-S3

### Install ESP-IDF

On your Mac or in the Linux VM:

```bash
# Clone ESP-IDF
mkdir -p ~/esp
cd ~/esp
git clone -b v5.2 --recursive https://github.com/espressif/esp-idf.git
cd esp-idf
./install.sh esp32s3

# Activate the environment (run this in every new terminal)
source ~/esp/esp-idf/export.sh
```

### Create the Project

```bash
cd ~/Desktop/rune/repo/rune/esp32
# Or create a new project
idf.py create-project rune_coprocessor
cd rune_coprocessor
```

### Build and Flash

```bash
# Set target
idf.py set-target esp32s3

# Build
idf.py build

# Flash (ESP32-S3-DevKitC-1 has USB-C, plug it in)
idf.py -p /dev/tty.usbmodem* flash

# Monitor serial output
idf.py -p /dev/tty.usbmodem* monitor
```

The ESP32-S3-DevKitC-1 enters download mode automatically when flashing via the USB-C port. If it doesn't, hold the BOOT button while pressing RESET.

## UART Protocol

The T113 and ESP32 communicate over a simple binary protocol. This is deliberately minimal — no protobuf, no JSON, no complexity.

### Frame Format

```
┌──────┬──────┬─────┬────────┬────────┬───────────┬──────┐
│ 0xAA │ 0x55 │ CMD │ LEN_LO │ LEN_HI │ PAYLOAD.. │ CRC8 │
└──────┴──────┴─────┴────────┴────────┴───────────┴──────┘

Header:   0xAA 0x55 (sync bytes)
CMD:      1 byte command
LEN:      2 bytes, little-endian, payload length (0 if no payload)
PAYLOAD:  0 to 65535 bytes
CRC8:     CRC-8/MAXIM over CMD + LEN + PAYLOAD
```

The sync bytes (0xAA, 0x55) allow the receiver to resynchronize after garbage or partial frames.

### Commands: T113 → ESP32

| CMD  | Name             | Payload | Description |
|------|------------------|---------|-------------|
| 0x01 | WIFI_SCAN        | none    | Request WiFi scan |
| 0x02 | WIFI_CONNECT     | SSID\0PASSWORD\0 | Connect to WiFi network |
| 0x03 | WIFI_STATUS      | none    | Query WiFi connection status |
| 0x04 | WIFI_DISCONNECT  | none    | Disconnect from WiFi |
| 0x10 | BLE_START_NOTIFY | none    | Start BLE notification listener |
| 0x11 | BLE_STOP_NOTIFY  | none    | Stop BLE notification listener |
| 0x12 | BLE_PAIR         | none    | Enter BLE pairing mode |
| 0xFF | HEARTBEAT        | none    | Keepalive ping |

### Responses: ESP32 → T113

Responses mirror the command byte with the high bit set:

| CMD  | Name             | Payload | Description |
|------|------------------|---------|-------------|
| 0x81 | WIFI_SCAN_RESULT | JSON array of {ssid, rssi} | Scan results |
| 0x82 | WIFI_CONNECT_ACK | 1 byte: 0=fail, 1=success | Connection result |
| 0x83 | WIFI_STATUS_RESP | 1 byte: 0=disconnected, 1=connected, then IP string | Status |
| 0x84 | WIFI_DISCONNECT_ACK | none | Confirmed |
| 0x90 | BLE_START_ACK    | 1 byte: 0=fail, 1=success | Started |
| 0x91 | BLE_STOP_ACK     | none | Stopped |
| 0x92 | BLE_PAIR_ACK     | 1 byte: 0=fail, 1=success | Paired |
| 0xFF | HEARTBEAT_ACK    | none | Pong |

### Unsolicited Events: ESP32 → T113

| CMD  | Name              | Payload | Description |
|------|-------------------|---------|-------------|
| 0x20 | BLE_NOTIFICATION  | app_name\0title\0body\0 | Phone notification received |
| 0x21 | WIFI_DISCONNECTED | none | WiFi connection dropped |

Null-terminated strings in the payload. The T113 parses them sequentially.

### CRC-8 Implementation

```c
/* CRC-8/MAXIM (polynomial 0x31, init 0x00) */
uint8_t crc8(const uint8_t *data, size_t len) {
    uint8_t crc = 0x00;
    for (size_t i = 0; i < len; i++) {
        crc ^= data[i];
        for (int j = 0; j < 8; j++) {
            if (crc & 0x80)
                crc = (crc << 1) ^ 0x31;
            else
                crc = crc << 1;
        }
    }
    return crc;
}
```

## BLE Notification Daemon

The primary use case for the ESP32 is mirroring phone notifications to Rune.

### Apple (ANCS)

Apple Notification Center Service (ANCS) is a BLE GATT profile that pushes notifications from an iPhone to a connected BLE peripheral.

1. ESP32 advertises as an ANCS consumer.
2. User pairs iPhone with ESP32 via Bluetooth settings.
3. iPhone pushes notification metadata over ANCS.
4. ESP32 reads the notification attributes (app name, title, body).
5. ESP32 sends a `BLE_NOTIFICATION` frame over UART to T113.

ESP-IDF has ANCS client examples. Start from there.

### Android

Android uses standard BLE GATT notifications. The phone runs a companion app (or uses Android's built-in notification access) to push notifications to the ESP32 via a custom GATT characteristic.

For initial development, focus on ANCS (Apple) since it's a standardized protocol. Android support can come later.

## WiFi Fallback

If the T113's RTL8723DS WiFi is unreliable (it sometimes is), the ESP32 can serve as a WiFi bridge:

1. T113 sends `WIFI_CONNECT` to ESP32 over UART.
2. ESP32 connects to WiFi.
3. T113 sends TCP data over UART (a higher-level protocol extension).
4. ESP32 forwards to the internet and returns responses.

This is slower than native WiFi (UART is the bottleneck) but works as a fallback. Implement this only if the RTL8723DS proves problematic.

## Testing

### From T113 Side

Use picocom or minicom to send raw bytes to the ESP32:

```bash
# Open UART2 on the T113
picocom -b 115200 /dev/ttyS2
```

Then send a heartbeat frame manually (or write a quick test script):

```python
#!/usr/bin/env python3
import serial
import struct

ser = serial.Serial('/dev/ttyS2', 115200, timeout=2)

def crc8(data):
    crc = 0x00
    for byte in data:
        crc ^= byte
        for _ in range(8):
            if crc & 0x80:
                crc = ((crc << 1) ^ 0x31) & 0xFF
            else:
                crc = (crc << 1) & 0xFF
    return crc

# Build heartbeat frame
cmd = 0xFF
payload = b''
length = len(payload)
frame_body = struct.pack('<BH', cmd, length) + payload
crc = crc8(frame_body)
frame = b'\xAA\x55' + frame_body + bytes([crc])

print(f"Sending: {frame.hex()}")
ser.write(frame)

# Read response
resp = ser.read(64)
print(f"Received: {resp.hex()}")

ser.close()
```

### From ESP32 Side

Use `idf.py monitor` to see debug output from the ESP32. The ESP32 firmware should log received commands and sent responses.

### Loopback Test

Before testing the full protocol, verify basic UART connectivity:

1. On the ESP32, write firmware that echoes received bytes.
2. On the T113, send known bytes and verify they come back.
3. If they don't, TX/RX are swapped or baud rate is wrong.

## Common Issues

### ESP32 Not Responding

1. **UART TX/RX swapped**: the most common mistake. Swap the wires.
2. **Wrong baud rate**: both sides must be 115200.
3. **ESP32 in download mode**: GPIO0 held low during boot. Make sure nothing is pulling GPIO0 low.
4. **ESP32 not running firmware**: flash might have failed. Re-flash and check `idf.py monitor` output.
5. **No common ground**: run a GND wire between the boards.

### BLE Pairing Fails

1. **iPhone BLE issues**: toggle Bluetooth off/on on the iPhone. Forget the device and re-pair.
2. **ESP32 not advertising**: check the BLE init code. Use a BLE scanner app (e.g., LightBlue on iOS) to verify the ESP32 is visible.
3. **ANCS not enabled**: ANCS requires the ESP32 to advertise specific service UUIDs. Verify against Apple's ANCS spec.

### UART Data Corruption

1. **Baud rate drift**: at 115200, timing isn't critical. At higher speeds, make sure both sides use the same clock source.
2. **Long wires**: keep UART wires short (under 30cm). Long wires pick up noise.
3. **Missing ground**: the ground wire between boards is essential for signal reference.
