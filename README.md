# RusPiRo bare metal system development kit crate

This is the packaged crate combining different RusPiRo crates into one library. The library can be configured with feature gates
and allowes a more convinient usage pattern for the dependencies in your ``Cargo.toml`` file. See the details and usage patterns
below...

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

The always usable crates are:

- [``ruspiro-register``](https://crates.io/crates/ruspiro-register)
- [``ruspiro-gpio``](https://crates.io/crates/ruspiro-gpio)
- [``ruspiro-mailbox``](https://crates.io/crates/ruspiro-mailbox)
- [``ruspiro-timer``](https://crates.io/crates/ruspiro-timer)
- [``ruspiro-interrupt``](https://crates.io/crates/ruspiro-interrupt)

Additional features/crates are:

| Feature            | Default | Description |
|--------------------|---------|-------------|
| ``with_boot``      | yes     | Bundle the [``ruspiro-boot``](https://crates.io/crates/ruspiro-boot) crate into the sdk package providing Raspberry Pi boot code.|
| ``with_allocator`` | yes     | Bundle the [``ruspiro-allocator``](https://crates.io/crates/ruspiro-allocator) crate into the sdk package|
| ``with_console``   | no      | Bundle the [``ruspiro-console``](https://crates.io/crates/ruspiro-console) crate into the sdk package. This requires an allocator to be present. |
| ``with_uart``      | no      | Bundle the [``ruspiro-uart``](https://crates.io/crates/ruspiro-uart) crate into the sdk package. This will always bundle ``ruspiro-console`` and does also require an allocator to be present.|
| ``with_i2c``       | no      | Bundle the [``ruspiro-i2c``](https://crates.io/crates/ruspiro-i2c) crate into the sdk package. This requires an allocator to be present. |


## Usage Scenarios

The following sections give some guidance how the ``ruspiro-sdk`` might be utilized. Each scenario provides an example
that could be found in the sub-folders mentioned in the respective chapter. Those examples should build just fine and could
be used as starting point for your own projects.

### Scenario 1: Minimal with built-in boot
This scenario is the proposed entry point in using the RusPiRo SDK the first time. In this scenario the ``ruspiro-sdk``
will be used with the set of default features only.
Find the whole crate structure here: [Scenario-1](scenario-1)

### Scenario 1: **Advanced!** Minimal without built-in boot

This advanced scenario is intended for those who either already have their own boot assembly or do want to write their own one.
Additionally some other core requirements to successfully build a binary when not using the ``ruspiro-boot`` need to be fulfilled:
1. provide a panic handler and a eh_personality function.
2. provide unwind stubs:
    - __aeabi_unwind_cpp_pr0
    - __aeabi_unwind_cpp_pr1
    - _Unwind_Resume
3. when using the ``ruspiro-allocator`` in this scenario the linker need to provide the two symbols:
    - ``__heap_start``
    - ``__heap_end``
    
    indicating the physical memory address space of the heap.

### Scenario 2: Using Uart/Console with built-in boot
This might be the most typical scenario to start with as it provides the functions to successfully initialize the Uart
to be used as console output channel which makes "debugging" on the real hardware a bit easier.
Find the whole crate structure here: [Scenario-2](scenario-2)


## License
Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)