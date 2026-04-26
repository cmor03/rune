# Contributing to Rune

Thanks for your interest in contributing to Rune. This document explains how to do it well.

## Before You Start

Read [docs/philosophy.md](docs/philosophy.md). PRs that violate the design philosophy will be declined. If you're unsure whether something fits, open an issue first.

Open an issue before starting significant work so we can discuss the approach. This saves everyone time.

## First-Time Contributors

Look for issues labeled **"good first issue"**. Documentation improvements are always welcome and are a great way to get familiar with the project.

## Pull Request Guidelines

- **One logical change per PR.** Don't bundle unrelated fixes.
- Open an issue before starting significant work to discuss the approach.

## Style Guide

- **Rust:** `rustfmt` + `clippy` clean. No warnings.
- **C:** Linux kernel style.
- **Documentation:** Clear English. No marketing fluff.

## Testing

- Include tests for new functionality.
- Don't break existing tests.
- Test on ARM target when touching hardware interfaces.

## Commit Messages

Use imperative mood, 50-character subject line, body that explains *why*.

Prefix your subject line:

- `firmware:` — firmware changes
- `hardware:` — hardware design changes
- `docs:` — documentation
- `ci:` — CI/CD pipeline

Example:

```
firmware: add BLE reconnection timeout

The device would hang indefinitely when the phone moved out of range.
A 30-second timeout lets it fall back to offline mode gracefully.

Signed-off-by: Your Name <your@email.com>
```

## DCO Sign-Off

All commits must be signed off per the [Developer Certificate of Origin](https://developercertificate.org/). Add `-s` to your commit command:

```
git commit -s -m "firmware: your change description"
```

This adds a `Signed-off-by` line certifying you wrote (or have the right to submit) the code under the project's license.

### Developer Certificate of Origin 1.1

```
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.


Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

## Review Process

- A maintainer will review your PR within a week.
- Changes may be requested. Don't take it personally.
- **Firmware changes** require two approvals.
- **Documentation changes** require one approval.

## Code of Conduct

This project follows a code of conduct. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
