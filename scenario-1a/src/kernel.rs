/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: MIT
 **********************************************************************************************************************/
#![no_std]
#![no_main]

//! # Scenarion 1A - minimal without built-in boot
//! 
//! This example will not compile just out of the box. The reason is that there are several additional requirements
//! not fulfilled that are provided by the ``ruspiro-boot`` crate. Those requirements are:
//! 1. provide a panic handler and a eh_personality function.
//! 2. provide unwind stubs:
//!     - __aeabi_unwind_cpp_pr0
//!     - __aeabi_unwind_cpp_pr1
//!     - _Unwind_Resume
//!
//! Those functions could be provided in this or any other crate that will be linked into the final binary.

#![feature(global_asm)]
#![feature(lang_items)]

use ruspiro_sdk::*;

global_asm!(include_str!("./asm/boot.s"));

#[no_mangle]
fn __rust_entry__() -> ! {
    
    GPIO.take_for(|gpio| {
        gpio.get_pin(17).unwrap().to_output().high();
    });
    
    loop { };
}
