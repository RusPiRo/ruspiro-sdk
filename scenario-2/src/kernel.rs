/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: MIT
 **********************************************************************************************************************/
#![no_std]
#![no_main]

//! # Scenarion 2 - use Uart/Console with built-in boot
//! 

use ruspiro_sdk::*;

come_alive_with!(alive);
run_with!(think);

fn alive(core: u32) {
    if core == 0 {
        // start by taking the core_clock with a mailbox call
        MAILBOX.take_for(|mb| mb.get_clockrate(ArmClockId::Core))
            .and_then(|core_rate| {
                // using this to initialize the UART
                let mut uart = Uart0::new();
                uart.initialize(core_rate, 115_200).map(|_| uart)
            }).and_then(|uart| {
                // set the uart as current console output channel
                CONSOLE.take_for(|console| console.replace(uart));
                // everything ok here
                Ok(())                
            }).and_then(|_| {
                // when everything went fine lit the LED 17
                GPIO.take_for(|gpio| gpio.get_pin(17))
            }).and_then(|pin| {
                pin.to_output().high();
                Ok(())
            }).expect("unable to initialize kernel"); // panic if something went wrong during init
    }
}

fn think(_core: u32) -> ! {

    loop { };
}
