# Reproducer

Reproducer for a regression bug introduced in rust 1.45 (thumbv6 target)

failing function:

```rust
pub fn foo(val: u128) -> u128 {
    let (_sum, carry) = val.overflowing_add(val);
    u128::from(carry) << 64
}
```

results in an infinite compilation.


## Steps to reproduce


``` bash
rustup target add thumbv6m-none-eabi
cargo build
```

## Validate compilation in rust 1.44

rust-toolchain
``` 
[toolchain]
channel = "1.44.0"
components = ["rust-src"]
```


``` bash
rustup target add thumbv6m-none-eabi
cargo build
```