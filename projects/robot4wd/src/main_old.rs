#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut en_a = pins.d9.into_output();
    let mut in1 = pins.d7.into_output();
    let mut in2 = pins.d6.into_output();
    let mut en_b = pins.d3.into_output();
    let mut in3 = pins.d5.into_output();
    let mut in4 = pins.d4.into_output();

    en_a.set_high();
    en_b.set_high();

    in1.set_high();
    in2.set_low();

    in3.set_high();
    in4.set_low();

    loop{}

   
}
