# Security Policy

## Supported Versions

The following versions of sys-shred are currently supported with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.3.x   | :white_check_mark: |
| 0.2.x   | :warning: Legacy   |
| 0.1.x   | :x: End of Life    |

## Security Model (v0.3.0+)

`sys-shred` employs a multi-layered anti-forensic approach:
- **Entropy Source**: Uses `rand::rngs::StdRng` for cryptographically secure random data generation.
- **Hardware Sync**: Enforces `sync_all()` (equivalent to `fsync` or `FlushFileBuffers`) to bypass OS and disk controller write caches.
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
