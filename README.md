# Project template for rp2040-hal

based on rp2040-hal

It includes all of the `knurling-rs` tooling as showcased in https://github.com/knurling-rs/app-template (`defmt`, `defmt-rtt`, `panic-probe`, `flip-link`) to make development as easy as possible.


# Getting started
  
- The standard Rust tooling (cargo, rustup) which you can install from https://rustup.rs/
- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)
- flip-link - this allows you to detect stack-overflows on the first core, which is the only supported target for now.
- probe-run. Upstream support for RP2040 was added with version 0.3.1.

- second rp2040
  * https://github.com/majbthrd/DapperMime/releases/download/20210225/raspberry_pi_pico-DapperMime.uf2
  * wire up to debug port


```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
cargo install probe-run
```

# build


## debug
```sh
cargo run
```

## release
```sh
cargo run --release
```


If you do not specify a DEFMT_LOG level, it will be set to `debug`.
That means `println!("")`, `info!("")` and `debug!("")` statements will be printed.
If you wish to override this, you can change it in `.cargo/config.toml` 
```toml
[env]
DEFMT_LOG = "off"
```
You can also set this inline (on Linux/MacOS)  
```sh
DEFMT_LOG=trace cargo run
```

or set the _environment variable_ so that it applies to every `cargo run` call that follows:
#### Linux/MacOS/unix
```sh
export DEFMT_LOG=trace
```

Setting the DEFMT_LOG level for the current session  
for bash
```sh
export DEFMT_LOG=trace
```

