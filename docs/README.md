# Rune Documentation

Rune is a 78x78x13mm pocket AI companion with a 3.7" monochrome e-ink display. Five functions: voice queries, ePub reading, music playback, phone notifications, and photo capture. Nothing else on purpose.

Firmware is licensed AGPL-3.0. Hardware is licensed CERN-OHL-S 2.0.

---

## Philosophy & Design

- [Philosophy](philosophy.md) -- Why Rune exists, what it refuses to become, and the design principles behind every decision.

## System Architecture

- [Architecture](architecture.md) -- Hardware layout, bus topology, software stack, cloud services, and data flow diagrams.
- [Glossary](glossary.md) -- Definitions for every acronym, chip, protocol, and tool referenced in this project.

## Development Guides

- [Getting Started](getting-started.md) -- Three paths into the project: back it, build it, or contribute to it.
- [FAQ](faq.md) -- Answers to the questions people actually ask.

## Hardware

- [Schematics](../hardware/schematics/) -- KiCad source files for the main board.
- [PCB](../hardware/pcb/) -- PCB layout and manufacturing files.
- [Enclosure](../hardware/enclosure/) -- Mechanical design for the case.

## Firmware

- [Buildroot](../firmware/buildroot/) -- Linux root filesystem configuration.
- [Kernel](../firmware/kernel/) -- Kernel config, device tree sources, and patches.
- [ESP32](../firmware/esp32/) -- Co-processor firmware for WiFi, BLE, and peripheral management.
- [Userspace](../firmware/userspace/) -- The main application that runs on the T113-S4.
