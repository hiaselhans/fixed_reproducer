#![no_std]

pub fn foo(val: u128) -> u128 {
    let hi = val >> 64;
    let prod = hi * hi;
    let (sum, carry) = prod.overflowing_add(prod);
    prod + (sum >> 64) + (u128::from(carry) << 64)
}
