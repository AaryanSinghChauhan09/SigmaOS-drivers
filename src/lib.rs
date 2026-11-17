// SPDX-License-Identifier: Apache-2.0
//! SigmaOS drivers — kernel driver core.
//!
//! Device model, driver trait (compatible-string matching, probe/remove),
//! and a DriverManager that enumerates devices and binds drivers — the
//! mechanism half of Linux's driver model. Concrete drivers live elsewhere.

#![forbid(unsafe_code)]

pub mod drivers;

pub use drivers::device::{Device, DeviceState};
pub use drivers::driver::{Driver, DriverError};
pub use drivers::manager::DriverManager;
