# esp8266-hal

An experimental hardware abstraction layer for the [esp8266](https://en.wikipedia.org/wiki/ESP8266) written in Rust.

Contributions are welcome :)

Join in on the discussion: https://matrix.to/#/#esp-rs:matrix.org!

An example project using the crate can be found [here](https://github.com/icewind1991/xtensa-rust-quickstart/tree/esp8266).

## Setting up the compiler

- setup the [xtensa rust](https://github.com/MabezDev/rust-xtensa) compiler.

```
$ git clone https://github.com/MabezDev/rust-xtensa
$ cd rust-xtensa
$ ./configure --llvm-root=$HOME/llvm-project/llvm/build
$ ./x.py build
```

- install the xtensa-lx106-elf toolchain from the [espressif web site](https://docs.espressif.com/projects/esp8266-rtos-sdk/en/latest/get-started/linux-setup.html).

```
$ mkdir ~/esp
$ tar -xzf ~/Downloads/xtensa-lx106-elf-linux64-1.22.0-100-ge567ec7-5.2.0.tar.gz -C ~/esp
$ PATH="$PATH:$HOME/esp/xtensa-lx106-elf/bin"
```

- install xargo

```
$ cargo install xargo
```

- point xargo to your compiler

```
$ export XARGO_RUST_SRC=/path/to/rust-xtensa/src
$ export RUSTC=/path/to/rust-xtensa/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
```

- install esptool

```
$ pip install esptool
```

Once you have the xtensa rust compiler setup you'll need to point `xargo`

## Flashing the examples

Once you have your rust compiler and toolchain bits setup you can flash the examples using

```
$ ./flash_example <example>
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
