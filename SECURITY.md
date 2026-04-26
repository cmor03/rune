# Security Policy

Rune is a small device, but it has serious trust boundaries: a microphone, a
camera, local files, BLE, WiFi, and optional cloud AI providers. Please report
security issues privately so users are not put at unnecessary risk.

## Reporting a Vulnerability

Do not open a public GitHub issue for security vulnerabilities.

Send reports to: **[security/contact email TBD]**

Include as much detail as you can:

- Affected component or directory
- Hardware revision, if relevant
- Firmware revision or branch, if relevant
- Steps to reproduce
- Expected impact
- Any proof-of-concept code or logs, sanitized for secrets

We will acknowledge receipt within 7 days when the project has an active
maintainer contact configured. Until then, this policy documents the intended
process and the contact field must be filled before accepting public hardware
or firmware releases.

## Responsible Disclosure Timeline

The target disclosure process is:

1. Acknowledge the report within 7 days.
2. Confirm scope and severity within 14 days.
3. Prepare a fix or mitigation plan within 30 days for high-severity issues.
4. Publish an advisory after a fix is available, or coordinate a longer embargo
   when users need time to update hardware or firmware.

Hardware vulnerabilities can take longer to remediate than software bugs. If a
board-level issue requires a PCB revision, the advisory should include practical
mitigations for existing units where possible.

## In Scope

- Firmware in this repository
- Buildroot configuration and root filesystem contents
- ESP32-S3 co-processor firmware and UART protocol
- BLE notification handling
- Device-side handling of microphone, camera, books, music, and local files
- Hardware design files, including power, USB, debug, storage, and radio
  connections
- Project-owned cloud companion services, if such services are added later

## Out of Scope

- Vulnerabilities in third-party AI providers
- Vulnerabilities in Spotify or other external music services
- Vulnerabilities in phone operating systems or notification APIs
- Attacks requiring physical destruction of the device
- Social engineering of maintainers or users
- Denial-of-service reports that only consume public project resources without
  demonstrating a device or user impact

## Security Design Principles

- No telemetry, analytics, or crash reporting that phones home.
- Cloud voice queries go directly to the user-selected provider.
- Secrets belong in local configuration, not in firmware images or source code.
- Hardware debug interfaces should be documented, not hidden.
- Failures in BLE, WiFi, display, audio, or camera should degrade gracefully.

## Supported Versions

Rune is pre-release. Security fixes target the default branch until tagged
firmware releases exist. Once releases begin, this section will list supported
firmware and hardware revisions.
