/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-sdk/0.1.0")]
#![no_std]

//! # RusPiRo System development kit library
//! 
//! This crate bundles the different RusPiRo crates for convinient uses as dependencie. The crate comes with several
//! feature gates to 'customize' the parts that are required within the actual project using this crate.
//! 

// re-export common crates that should be always available
#[macro_use]
extern crate ruspiro_register;
pub use ruspiro_register::{define_register, define_registers, RegisterField, RegisterFieldValue};
pub use ruspiro_gpio::GPIO;
pub use ruspiro_mailbox::{MAILBOX, ArmClockId};
pub use ruspiro_timer as timer;

/*
#[macro_use]
extern crate ruspiro_interrupt;
pub use ruspiro_interrupt::*;
*/

#[cfg(feature = "with_boot")]
#[macro_use]
extern crate ruspiro_boot;
#[cfg(feature = "with_boot")]
pub use ruspiro_boot::{come_alive_with, run_with};

// all other crates are exported based on the features given...
#[cfg(feature = "with_allocator")]
extern crate ruspiro_allocator;

#[cfg(any(feature = "with_console", feature = "with_uart"))]
#[macro_use]
extern crate ruspiro_console;
#[cfg(any(feature = "with_console", feature = "with_uart"))]
pub use ruspiro_console::{CONSOLE, print, println, info, warn, error};

#[cfg(feature = "with_uart")]
pub use ruspiro_uart::Uart0;

#[cfg(feature = "with_i2c")]
pub use ruspiro_i2c::{I2C, I2cDevice};