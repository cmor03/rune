# Changelog

All notable changes to Rune will be documented in this file.

This project follows the spirit of [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and uses semantic versioning once tagged firmware releases begin. Rune is still
pre-release, so version numbers are not assigned yet.

## Unreleased

### Added

- Initial open-hardware repository structure.
- Project philosophy, architecture, hardware, firmware, and development
  documentation.
- Full AGPL-3.0 repository license and CERN-OHL-S 2.0 hardware license.
- Contribution, security, code of conduct, issue, and pull request guidance.

### Notes

- Firmware language boundaries are still being validated. Rust is the preferred
  direction for userspace, C remains appropriate for kernel work, and ESP32
  firmware follows esp-idf conventions.
- Production PCB details are placeholders until the first hardware revision is
  released.
