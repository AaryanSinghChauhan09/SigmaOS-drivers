# SigmaOS Drivers

This crate exports a `DriverManager` and a broad collection of driver and
device-model modules. Their presence in source does not establish that a driver
is safe, complete, or validated against physical hardware. The repository
depends on the sibling `SigmaOS-kernel` checkout through a local path.

Before enabling a driver on a machine, document its supported device IDs,
hardware and emulator evidence, failure behavior, and privilege boundary.
Module-level or model tests alone are not hardware validation.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).
