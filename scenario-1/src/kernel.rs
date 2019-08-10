/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: MIT
 **********************************************************************************************************************/
#![no_std]
#![no_main]
#![feature(lang_items)]
//! # Scenarion 1 - minimal with built-in boot
//! 
//! When the one-time initialization is called due to coming alive we lit the LED connected to GPIO pin 17 to let the
//! world know that we are alive :)
//! 

use ruspiro_sdk::*;

come_alive_with!(alive);
run_with!(think);

fn alive(core: u32) {
    if core == 0 {
        GPIO.take_for(|gpio| {
            gpio.get_pin(17).unwrap()
                .to_output().high();
        });
    }
}

fn think(_core: u32) -> ! {

    loop { };
}
