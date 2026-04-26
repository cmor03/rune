# CLAUDE.md -- Instructions for AI Assistants

## Project Context

Rune is an open-source pocket AI companion device (78x78x13mm) with a 3.7" monochrome e-ink display. It runs Linux (Buildroot) on an Allwinner T113-S4 SoC with an ESP32-S3 co-processor for WiFi/BLE. It does exactly five things: voice queries to cloud AI, ePub reading, music playback, phone notification mirroring, and photo capture. The firmware is AGPL-3.0, the hardware is CERN-OHL-S 2.0. The design philosophy prioritizes focus, simplicity, and user agency over feature count.

## Coding Conventions

### Rust (userspace applications under `firmware/userspace/`)
- Format with `rustfmt` (default config)
- Must pass `clippy` with no warnings: `cargo clippy -- -D warnings`
- Use `thiserror` for library error types, `anyhow` for application error types
- Prefer `no_std` where possible; this is an embedded target with limited resources
- Target: `armv7-unknown-linux-musleabihf`

### C (kernel modules under `firmware/kernel/`)
- Follow Linux kernel coding style (`scripts/checkpatch.pl` clean)
- Tabs for indentation, 80-column lines
- Function names: lowercase with underscores, prefixed by subsystem (e.g., `rune_display_init`)

### C/C++ (ESP32 firmware under `firmware/esp32/`)
- Follow esp-idf conventions and project structure
- Use esp-idf component manager for dependencies
- Prefer C over C++ unless a dependency requires it
- Use ESP_LOG macros for logging (ESP_LOGI, ESP_LOGW, ESP_LOGE)

### General
- No `unsafe` in Rust without a `// SAFETY:` comment explaining why it's sound
- All public functions and types get doc comments
- No TODO comments in committed code without a linked issue number

## Error Handling Philosophy

- Propagate errors up. Use `Result` in Rust, return error codes in C.
- Never panic in production code. `unwrap()` and `expect()` are banned outside of tests.
- Log context at error boundaries -- when an error crosses a module boundary, log what was being attempted and why it failed before converting or propagating.
- Hardware errors (display, audio, BLE) should degrade gracefully. A failed BLE connection should not crash the voice query feature.
- Use structured logging with the subsystem name: `[display]`, `[audio]`, `[ble]`, `[voice]`, `[reader]`.

## Repository Layout

```
rune/
  firmware/
    buildroot/     # Buildroot external tree, defconfig, rootfs overlay
    kernel/        # Out-of-tree kernel modules, device tree overlays
    userspace/     # Rust workspace -- all user-facing applications
    esp32/         # ESP32-S3 firmware (esp-idf project)
  hardware/
    schematics/    # KiCad schematics
    pcb/           # PCB layout, Gerbers, BOM
    enclosure/     # Mechanical design files (STEP, STL)
  docs/
    philosophy.md  # Design philosophy -- READ THIS FIRST before proposing changes
    architecture.md
    development/   # Build guides, toolchain setup, flashing instructions
```

## What NOT to Do

These are hard rules, not suggestions:

- **Do not add cloud-dependent features without an offline fallback or an explicit opt-in.** Voice queries require the network by nature; nothing else should.
- **Do not add features that violate the philosophy doc.** No browser. No app store. No social feeds. No video playback. No algorithmic recommendations. Read `docs/philosophy.md` before proposing UX changes.
- **Do not add telemetry, analytics, or any form of data collection.** No crash reporting that phones home. No usage stats. Nothing.
- **Do not weaken AGPL compliance.** Do not add proprietary dependencies to the firmware. Do not add linking exceptions. If a dependency's license is incompatible with AGPL-3.0, do not use it.
- **Do not optimize for x86 at the expense of ARM.** This runs on a Cortex-A7. Profile on target hardware.
- **Do not add dependencies without justification.** Every crate, every library adds binary size and attack surface. Prefer the standard library. Justify new deps in the PR description.

## Testing Expectations

- **Unit tests** for all business logic. Rust modules should have `#[cfg(test)] mod tests` with meaningful coverage.
- **Integration tests** for hardware interfaces. These run on the actual ARM target, not just x86.
- **Mocking hardware** is acceptable for CI, but real hardware tests are required before merging changes to drivers or HAL code.
- **ESP32 firmware**: use esp-idf's Unity test framework for on-device tests.
- Tests must pass on both x86 (CI, with hardware mocked) and ARM (pre-merge, on real device or QEMU).

## Commit Message Format

```
subsystem: imperative summary under 50 chars

Explain WHY this change is being made, not WHAT changed (the diff
shows what changed). Include context that will help someone reading
this commit six months from now understand the motivation.

If this fixes a bug, describe the bug and how this commit fixes it.
If this is a new feature, explain why it's needed and how it fits
the design philosophy.

Refs: #123
```

Valid subsystem prefixes:
- `firmware:` -- general firmware changes spanning multiple subsystems
- `firmware/userspace:` -- Rust userspace applications
- `firmware/esp32:` -- ESP32 co-processor firmware
- `firmware/kernel:` -- kernel modules and device tree
- `firmware/buildroot:` -- Buildroot configuration and packages
- `hardware:` -- schematics, PCB, enclosure
- `docs:` -- documentation changes
- `ci:` -- build and CI pipeline changes

## PR Conventions

- One logical change per PR. Don't bundle unrelated fixes.
- Link related issues in the PR description.
- If the PR touches hardware interfaces, note whether it has been tested on real hardware or only in simulation.
- PR descriptions should explain the "why" -- reviewers can read the code for the "what".

## The Philosophy Doc

`docs/philosophy.md` is the design bible for this project. Before proposing any user-facing change, read it. If your change conflicts with the philosophy, either don't make it or propose a philosophy amendment as a separate discussion first. The philosophy doc is not an obstacle -- it's the reason this project exists as something different from a smartphone.
