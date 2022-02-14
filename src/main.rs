#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_reason: &PanicInfo) -> ! {
    loop {}
}


pub fn foo(val: u128) -> u128 {
    let hi = val >> 64;
    let prod = hi * hi;
    let (sum, carry) = prod.overflowing_add(prod);
    prod + (sum >> 64) + (u128::from(carry) << 64)
}


#[entry]
fn main() -> ! {
    for i in 0..200 {
        let x = foo(i);
        if x > 190 {
            loop {}
        }
    }
    loop {}
}
