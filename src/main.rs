#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};

use rp_pico::hal::{
    i2c,
    gpio,
    gpio::pin::{FunctionI2C, Pin},
    gpio::pin::bank0::{
        Gpio4, Gpio5, Gpio25, Gpio12, Gpio13, Pins
    },
    pac,
    sio::Sio,
    clocks::{init_clocks_and_plls, Clock},
    Timer, timer,
    watchdog::Watchdog,
};

use defmt::*;
use defmt_rtt as _;

//use panic_halt as _;
use panic_semihosting as _;

type Num = fixed::FixedI64<typenum::U32>;

#[entry]
fn main() -> ! {
    info!("loop!");
    let number = Num::from_num(0.001f64);

    info!("value: {}", number.int_log2());


    loop {

    }
}
