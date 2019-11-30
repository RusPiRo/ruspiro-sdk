/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-sdk/0.3.0")]
#![no_std]

//! # RusPiRo System development kit library
//! 
//! This crate bundles the different RusPiRo crates for convinient use as dependency. The crate comes with several
//! feature gates to 'customize' the parts that are required within the actual project using this crate.
//! 
//! The always usable crates are:
//! - [``ruspiro-register``](https://crates.io/crates/ruspiro-register)
//! - [``ruspiro-lock``](https://crates.io/crates/ruspiro-lock)
//! - [``ruspiro-singleton``](https:://create.io/crates/ruspiro-singleton)
//! - [``ruspiro-gpio``](https://crates.io/crates/ruspiro-gpio)
//! - [``ruspiro-mailbox``](https://crates.io/crates/ruspiro-mailbox)
//! - [``ruspiro-timer``](https://crates.io/crates/ruspiro-timer)
//! - [``ruspiro-interrupt``](https://crates.io/crates/ruspiro-interrupt)
//! - [``ruspiro-cache``](https://crates.io/crates/ruspiro-cache)
//! 
//! Additional features/crates are:
//! 
//! | Feature            | Default | Description |
//! |--------------------|---------|-------------|
//! | ``with_boot``      | yes     | Bundle the [``ruspiro-boot``](https://crates.io/crates/ruspiro-boot) crate into the sdk package providing Raspberry Pi boot code.|
//! | ``with_allocator`` | yes     | Bundle the [``ruspiro-allocator``](https://crates.io/crates/ruspiro-allocator) crate into the sdk package|
//! | ``with_console``   | no      | Bundle the [``ruspiro-console``](https://crates.io/crates/ruspiro-console) crate into the sdk package. This requires an allocator to be present. |
//! | ``with_uart``      | no      | Bundle the [``ruspiro-uart``](https://crates.io/crates/ruspiro-uart) crate into the sdk package. This will always bundle ``ruspiro-console`` and does also require an allocator to be present.|
//! | ``with_i2c``       | no      | Bundle the [``ruspiro-i2c``](https://crates.io/crates/ruspiro-i2c) crate into the sdk package. This requires an allocator to be present. |
//! 

// re-export common crates that should be always available
#[allow(unused_imports)]
#[macro_use]
extern crate ruspiro_register;
pub use ruspiro_register::{define_register, define_registers, RegisterField, RegisterFieldValue};
pub use ruspiro_gpio::GPIO;
pub use ruspiro_mailbox::{MAILBOX, ArmClockId};
pub use ruspiro_timer as timer;
pub use ruspiro_singleton::Singleton;
pub use ruspiro_lock as lock;

#[allow(unused_imports)]
#[macro_use]
pub extern crate ruspiro_interrupt;
pub use ruspiro_interrupt::*;

#[allow(unused_imports)]
pub use ruspiro_cache as cache;

#[cfg(feature = "with_boot")]
#[allow(unused_imports)]
#[macro_use]
extern crate ruspiro_boot;
#[cfg(feature = "with_boot")]
pub use ruspiro_boot::{come_alive_with, run_with};

// all other crates are exported based on the features given...
#[cfg(feature = "with_allocator")]
extern crate ruspiro_allocator;

#[cfg(any(feature = "with_console", feature = "with_uart"))]
#[allow(unused_imports)]
#[macro_use]
extern crate ruspiro_console;
#[cfg(any(feature = "with_console", feature = "with_uart"))]
pub use ruspiro_console::{CONSOLE, print, println, info, warn, error};

#[cfg(feature = "with_uart")]
pub use ruspiro_uart as uart;//{Uart0, Uart1};

#[cfg(feature = "with_i2c")]
pub use ruspiro_i2c::{I2C, I2cDevice};