#![no_std]

pub fn foo(val: u128) -> u128 {
    let (_sum, carry) = val.overflowing_add(val);
    u128::from(carry) << 64
}
