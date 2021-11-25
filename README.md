# esp8266-hal

An experimental hardware abstraction layer for the [esp8266](https://en.wikipedia.org/wiki/ESP8266) written in Rust.

Contributions are welcome :)

Join in on the discussion: https://matrix.to/#/#esp-rs:matrix.org!

An example project using the crate can be found [here](https://github.com/icewind1991/xtensa-rust-quickstart/tree/esp8266).

## Setting up the compiler

In order to build Rust for the Xtensa architecture, you must use the [esp-rs/rust](https://github.com/esp-rs/rust) compiler fork.

This can be installed via the installation scripts and pre-built artifacts found in the [esp-rs/rust-build](https://github.com/esp-rs/rust-build) repository. Alternatively, you can build and install the compiler from source.

For more information relating to the Rust compiler fork please refer to the [Installing Rust](https://esp-rs.github.io/book/getting-started/installing-rust.html) section of [The Rust on ESP Book](https://esp-rs.github.io/book/).

### Using the Installation Scripts

#### Linux/macOS

```shell
$ curl -LO https://raw.githubusercontent.com/esp-rs/rust-build/main/install-rust-toolchain.sh
$ chmod +x install-rust-toolchain.sh
$ ./install-rust-toolchain.sh
```

#### Windows

```powershell
PS> Invoke-WebRequest https://raw.githubusercontent.com/esp-rs/rust-build/main/Install-RustToolchain.ps1 -OutFile Install-RustToolchain.ps1
PS> .\Install-RustToolchain.ps1
```

## Flashing the examples

Once you have the Rust compiler fork installed you can flash the examples using [cargo-espflash](https://github.com/esp-rs/espflash/tree/master/cargo-espflash):

```shell
$ cargo install cargo-espflash
$ cargo espflash --release --example blinky /dev/ttyUSB0
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
