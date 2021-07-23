# Antei

Antei is an experimental Operating System for the `x86_64` architecture written in stable Rust.

## Dependencies

You need to install these programs:
- `rustup`
- `cargo`
- `qemu-system-x86_64`
- `mtools`
- `x86_64-pc-linux-gnu-gcc`
- `x86_64-w64-mingw32-gcc`
- `lld-link`

You need to install the Rust toolchain for Windows:
```sh
rustup target add x86_64-pc-windows-gnu
```

These binaries must exist at the same directory which `Makefile` is in.
- `OVMF_VARS.fd`
- `OVMF_CODE.fd`

## Running
```sh
make run
```

Adding `RELEASE=1` will compile binaries with optimizations.
```sh
make RELEASE=1 run
```

## Testing
```sh
make test
```

Adding `RELEASE=1` will test the OS with optimizations.
```sh
make RELEASE=1 test
```


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
