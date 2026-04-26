# ESP32-S3 Co-processor Firmware

Firmware for the ESP32-S3 co-processor. Built with **esp-idf**.

## Responsibilities

- **BLE notification mirroring** -- connects to phone, receives notifications (ANCS for iOS, GATT for Android), relays them to the T113 over UART
- **WiFi fallback** -- provides WiFi connectivity when the T113's built-in networking is insufficient or unavailable

## Communication

Communicates with the T113 main processor over **UART** using a binary protocol. See [firmware/architecture.md](../architecture.md) for the full protocol specification (frame format, command table, UART parameters).

## Building

Requires the esp-idf toolchain (v5.x). See [firmware/build.md](../build.md) for setup instructions.

```bash
cd firmware/esp32/
idf.py set-target esp32s3
idf.py build
```

## Flashing

```bash
idf.py -p /dev/ttyUSB0 flash
```

See [firmware/flash.md](../flash.md) for details and alternative flashing methods.

## Current status

Not yet populated. UART protocol and BLE notification handling are being defined.
