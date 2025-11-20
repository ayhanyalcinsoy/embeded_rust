#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer1Pwm};

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Stop,
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Serial
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    // Motor pinleri
    let mut pin_ileri = pins.d2.into_output();
    let mut pin_geri = pins.d4.into_output();
    let mut pin_sol = pins.d7.into_output();
    let mut pin_sag = pins.d8.into_output();

    // PWM timerlarını başlat (servo daha sonra kullanılacak)
    let tc0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let tc1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let _pwm0 = pins.d5.into_output().into_pwm(&tc0);
    let _pwm1 = pins.d6.into_output().into_pwm(&tc0);
    let _pwm2 = pins.d9.into_output().into_pwm(&tc1);
    let _pwm3 = pins.d10.into_output().into_pwm(&tc1);

    ufmt::uwriteln!(&mut serial, "Robot Hazir!\r").unwrap();

    let mut determinant = Direction::Stop;

    loop {
        if let Ok(c) = nb::block!(serial.read()) {
            match c {
                b'F' => determinant = Direction::Forward,
                b'B' => determinant = Direction::Backward,
                b'L' => determinant = Direction::Left,
                b'R' => determinant = Direction::Right,
                b'S' => determinant = Direction::Stop,
                _ => {}
            }

            match determinant {
                Direction::Forward => {
                    pin_ileri.set_high();
                    pin_sol.set_high();
                    pin_geri.set_low();
                    pin_sag.set_low();
                }
                Direction::Backward => {
                    pin_geri.set_high();
                    pin_sag.set_high();
                    pin_ileri.set_low();
                    pin_sol.set_low();
                }
                Direction::Left => {
                    pin_ileri.set_high();
                    pin_sag.set_high();
                    pin_geri.set_low();
                    pin_sol.set_low();
                }
                Direction::Right => {
                    pin_geri.set_high();
                    pin_sol.set_high();
                    pin_ileri.set_low();
                    pin_sag.set_low();
                }
                Direction::Stop => {
                    pin_ileri.set_low();
                    pin_geri.set_low();
                    pin_sol.set_low();
                    pin_sag.set_low();
                }
            }
        }
    }
}
