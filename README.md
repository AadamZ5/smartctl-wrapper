# smartctl-wrapper-rs

This library aims to provide `smartctl` usability in Rust using `smartctl`'s `--json` option, which makes parsing data easy.

## Prerequisites:
- You must have `smartmontools` installed on your system. This program only wraps output from it.
- Whatever user / program that consumes this library must have access to all system resources that `smartctl` needs. Typically this is root.
