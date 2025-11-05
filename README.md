[![en](https://img.shields.io/badge/lang-en-blue.svg)](README.md)
[![de](https://img.shields.io/badge/lang-de-blue.svg)](README.de.md)

# ハッカー (Hakkā) - An Introduction to SMT Soldering and Microcontroller Programming

![board front](doc/board-front.png)
![board back](doc/board-back.png)

# Setup

## Toolchain
1. [Rustup.rs](https://rustup.rs/)
2. [The Rust on ESP Book
](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html#risc-v-devices)
3. [hakkaa-firmware](https://github.com/sirhcel/hakkaa-firmware)
4. Espflash
    ```
    $ cargo install --locked espflash
    ```
    `--locked` installs exactly the versions of the dependencies stored in the project

# Testing the setup and hardware

* Execute hardware test; Information and instruction are available via the log output
    ```
    $ cargo run --example eol-test
    ```

# Custom firmware for the target

* Generate documentation for Hakkaa board support and used modules and view it in the browser
    ```
    $ cargo doc --open
    ```

* based on a copy of the blinky example `examples/blinky.rs`
* create a copy in `src/bin`, for example `src/bin/pov.rs`
* Then run it in the same way as the hardware test
   ```
   $ cargo run --bin pov
   ```

# License

Licensed under either

* Apache License, Version 2.0 (LICENSE-APACHE or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your discretion.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

