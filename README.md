# RusPiRo bare metal system development kit crate



[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-sdk.svg?branch=master)](https://travis-ci.org/RusPiRo/ruspiro-sdk)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-sdk.svg)](https://crates.io/crates/ruspiro-sdk)
[![Documentation](https://docs.rs/ruspiro-sdk/badge.svg)](https://docs.rs/ruspiro-sdk)
[![License](https://img.shields.io/crates/l/ruspiro-sdk.svg)](https://github.com/RusPiRo/ruspiro-sdk#license)


## Usage
To use the crate just add the following dependency to your ``Cargo.toml`` file. The components of the sdk crate could be configured using feature gates. The most important one is to set the ``ruspiro_pi3`` feature to enable compiling the peripheral address space and Raspberry Pi 3 specific components successfully.
```
[dependencies]
ruspiro-sdk = { version = "0.1.0", features = ["ruspiro_pi3"]
```

Additional features are:

| Feature            | Default | Description |
|--------------------|---------|-------------|
| ``with_boot``      | yes     | Bundle the ``ruspiro-boot`` crate into the sdk package providing Raspberry Pi boot code.|
| ``with_allocator`` | yes     | Bundle the ``ruspiro-allocator`` crate into the sdk package|
| ``with_console``   | no      | Bundle the ``ruspiro-console`` crate into the sdk package. This requires an allocator to be present. |
| ``with_uart``      | no      | Bundle the ``ruspiro-uart`` crate into the sdk package. This will always bundle ``ruspiro-console`` and does also require an allocator to be present.|
| ``with_i2c``       | no      | Bundle the ``ruspiro-i2c`` crate into the sdk package. This requires an allocator to be present. |


## Usage Scenarios

The following sections give some guidance how the ``ruspiro-sdk`` might be utilized.

### Scenario 1: Minimal with built-in boot
This scenario is the proposed entry point in using the RusPiRo SDK the first time. In this scenario the ``ruspiro-sdk`` comes with a set of default features, ready to be used.
Find the whole crate structure here: [Scenario-1](scenario-1)

**Cargo.toml**
```
[package]
name = "rpi3-kernel"
version = "0.0.1"
edition = "2018"

[[bin]]
name = "kernel7"
path = "src/kernel.rs"

[dependencies]
ruspiro-sdk = { version = "0.1.0", features = ["ruspiro_pi3"] }
```

**src/kernel.rs**
```
#![no_std]
#![no_main]

use ruspiro_sdk::*;

come_alive_with!(alive);
run_with!(think);

fn alive(core: u32) {
    if core == 0 {
        GPIO.take_for(|gpio| {
            gpio.get_pin(17).unwrap().to_output().high();
        });
    }
}

fn think(_core: u32) -> ! {

    loop { };
}
```

### Scenario 1: **Advanced!** Minimal without built-in boot

This advanced scenario 0 is intended for those who either already have their own boot assembly or do want to write their own one.
So the source code provided assumes the boot assembly to kick-off the Raspberry Pi is provided be the implementer in the file ``src/asm/boot.s``.
**Cargo.toml**
```
[package]
name = "rpi3-kernel"
edition = "2018"

[[bin]]
name = "kernel7"
path = "src/kernel.rs"

[dependencies]
ruspiro-sdk = { version = "0.1.0", default-features = false, features = ["ruspiro_pi3"] }
```

**src/kernel.rs**
```
#![no_std]
#![no_main]
#![feature(global_asm)]

// make use of the ruspiro-sdk functions
use ruspiro_sdk::*;

// include the assembly file. Based on the assembly used this might not properly compile here. In this case the assembly should be pre-build using a build script.
// as this is even more advanced this is not demonstrated here. Check out the [Rust documentation](https://doc.rust-lang.org/cargo/reference/build-scripts.html) for details.
global_asm!(include_str!("./asm/boot.s"));

// we assume that the implementer of the boot assembly is calling a function as the entry point of the Rust environment. This is assumed to be ``__rust_entry__`` and is intended to never return.

#[no_mangle]
fn __rust_entry__() -> ! {
    
    // we are in Rust and can use the sdk features like the GPIO's
    GPIO.take_for(|gpio| {
        // set GPIO Pin as output and high to lit a connected LED
        gpio.get_pin(17).unwrap().to_output().high();
    });

    loop { }; // halt here to never return from this function
} 
```

#### HINT:
If you tend to go for this scenario and you would like to use further features like shown in the scenarios below (e.g. ``ruspiro-console``) that do use the custom allocator you have to ensure that the linker script provides the following symbols:
``__heap_start`` and ``__heap_end`` indicating the physical memory address space of the heap.

### Scenario 2: Using Uart/Console with built-in boot



## License
Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)