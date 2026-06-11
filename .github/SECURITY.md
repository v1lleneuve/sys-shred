# Security Policy

## Supported Versions

The following versions of sys-shred are currently supported with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 1.2.x   | :white_check_mark: |
| 1.1.x   | :warning: Legacy   |
| < 1.1.x | :x: End of Life    |

## Security Model (v1.2.0+)

`sys-shred` employs a multi-layered anti-forensic approach:
- **Entropy Source**: Uses `rand::rngs::StdRng` for cryptographically secure random data generation.
- **Hardware Sync**: Enforces `sync_all()` (equivalent to `fsync` or `FlushFileBuffers`) to bypass OS and disk controller write caches.
- **SSD TRIM/Discard**: Optional hardware-level deallocation to mitigate "wear-leveling" data persistence on Flash-based media.
- **Read-back Verification**: Optional hardware-level confirmation that data was physically committed to the medium.
- **Metadata Scrubbing**: Path randomization and file truncation are performed to clear filesystem-level artifacts.

## Reporting a Vulnerability

If you discover a security vulnerability within this project, please do not report it through public issue trackers. Instead, send a detailed report to the maintainer at:

v1lleneuve@proton.me

Please include the following information in your report:

- A descriptive title of the vulnerability.
- Steps to reproduce the issue.
- Potential impact and severity.
- Any suggested mitigations or fixes.

I will acknowledge receipt of your report within 48 hours and provide a timeline for resolution and public disclosure.

## Disclosure Policy

This project follows a responsible disclosure model. I ask that you provide a reasonable amount of time to address the vulnerability before making it public. In return, I will work to ensure the issue is resolved promptly and that you receive proper credit for the discovery.
