# Getting Started

Three ways into the Rune project. Pick the one that fits.

---

## Path 1: I want one

Back the Kickstarter when it launches. You'll receive a fully assembled Rune device with firmware pre-installed.

**Link**: [Coming soon -- Kickstarter campaign not yet live]

**Prerequisites**: None. A WiFi network and a phone (iOS or Android) for notification sync.

---

## Path 2: Build my own

Manufacture and assemble a Rune from source files.

**Start here**:
- [hardware/BOM.md](../hardware/BOM.md) -- Bill of materials with part numbers and sourcing links.
- [docs/development/prototype-setup.md](development/prototype-setup.md) -- Step-by-step prototype assembly and bring-up guide.

**Prerequisites**:
- Soldering experience (QFN packages, 0402 passives, BGA if using T113-S4 directly)
- Access to PCB fabrication (JLCPCB, PCBWay, or equivalent)
- USB-UART adapter for serial console
- A Linux workstation for building firmware
- Basic familiarity with embedded Linux (device trees, Buildroot, cross-compilation)

**Estimated time**: 2-4 weeks for PCB fabrication and assembly. 1-2 days for firmware build and flashing. Longer if this is your first embedded Linux project.

**Cost**: Approximately $40-60 USD in components, plus PCB fabrication costs.

---

## Path 3: Contribute to firmware

Work on Rune's software without building the hardware. You can develop and test against emulated or partially simulated targets.

**Start here**:
- [firmware/build.md](../firmware/build.md) -- Build instructions for the T113 Linux image and ESP32 firmware.
- [CONTRIBUTING.md](../CONTRIBUTING.md) -- Contribution guidelines, code style, and PR process.

**Prerequisites**:
- A Linux workstation (native or VM). macOS works for ESP32 firmware only.
- `git`, `make`, `gcc` cross-toolchain (arm-linux-gnueabihf), and standard build tools.
- Rust toolchain (if working on the userspace application).
- ESP-IDF v5.x (if working on the ESP32 firmware).
- Docker (optional, for reproducible Buildroot builds).

**Estimated time**: 1-2 hours to set up the toolchain and build from source. Ongoing from there.

**What to work on**: Check the issue tracker for issues tagged `good first issue` or `help wanted`. The areas that need the most help are documented in the issue tracker.
