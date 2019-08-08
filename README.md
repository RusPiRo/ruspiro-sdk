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
| Feature       | Description |
|---------------|-------------|
| ``with_boot`` | |


Once done the access to the functions and features of the whole RusPiRo crates beeing packed into the SDK crate are available in your rust files like so:
```
use ruspiro-sdk::*;

```

## Tutorial


## License
Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)