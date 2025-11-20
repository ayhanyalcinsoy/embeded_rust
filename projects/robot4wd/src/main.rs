#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm, Timer2Pwm};

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

    // Motor pinleri - L293 girişlerine göre
    let mut sag_ileri = pins.d2.into_output();  // L293-1A - Sağ motor ileri
    let mut sag_geri = pins.d4.into_output();   // L293-2A - Sağ motor geri
    let mut sol_ileri = pins.d7.into_output();  // L293-3A - Sol motor ileri
    let mut sol_geri = pins.d8.into_output();   // L293-4A - Sol motor geri

    // PWM timerları - D10 ve D11 için
    // Timer1: D9, D10 (biz D10'u kullanacağız)
    // Timer2: D3, D11 (biz D11'i kullanacağız, D3 servo için serbest)
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    // PWM pinleri - D10 ve D11
    let sol_pwm_pin = pins.d10.into_output().into_pwm(&timer1);  // Sol motor PWM (D10)
    let sag_pwm_pin = pins.d11.into_output().into_pwm(&timer2);  // Sağ motor PWM (D11)

    let mut sol_pwm = sol_pwm_pin;
    let mut sag_pwm = sag_pwm_pin;

    sol_pwm.enable();
    sag_pwm.enable();

    // Varsayılan hız (%78 - 200/255)
    let hiz: u8 = 200;
    sol_pwm.set_duty(hiz);
    sag_pwm.set_duty(hiz);

    ufmt::uwriteln!(&mut serial, "Robot Hazir! PWM: D10(Sol), D11(Sag)\r").unwrap();
    ufmt::uwriteln!(&mut serial, "Servo pinleri serbest: D3, D5, D6, D9\r").unwrap();

    let mut determinant = Direction::Stop;

    loop {
        if let Ok(c) = nb::block!(serial.read()) {
            match c {
                b'F' => determinant = Direction::Forward,
                b'B' => determinant = Direction::Backward,
                b'L' => determinant = Direction::Left,
                b'R' => determinant = Direction::Right,
                b'S' => determinant = Direction::Stop,
                b'0'..=b'9' => {
                    // Hız kontrolü: 0-9 arası sayılar
                    let hiz_seviye = (c - b'0') as u8;
                    let yeni_hiz = (hiz_seviye * 25).max(50); // Min %20, max %90
                    sol_pwm.set_duty(yeni_hiz);
                    sag_pwm.set_duty(yeni_hiz);
                    ufmt::uwriteln!(&mut serial, "Hiz: {}/255\r", yeni_hiz).unwrap();
                    continue;
                }
                b'+' => {
                    // Hız artır
                    let mevcut_hiz = sol_pwm.get_duty();
                    let yeni_hiz = mevcut_hiz.saturating_add(25).min(255);
                    sol_pwm.set_duty(yeni_hiz);
                    sag_pwm.set_duty(yeni_hiz);
                    ufmt::uwriteln!(&mut serial, "Hiz ARTIR: {}/255\r", yeni_hiz).unwrap();
                    continue;
                }
                b'-' => {
                    // Hız azalt
                    let mevcut_hiz = sol_pwm.get_duty();
                    let yeni_hiz = mevcut_hiz.saturating_sub(25).max(50);
                    sol_pwm.set_duty(yeni_hiz);
                    sag_pwm.set_duty(yeni_hiz);
                    ufmt::uwriteln!(&mut serial, "Hiz AZALT: {}/255\r", yeni_hiz).unwrap();
                    continue;
                }
                _ => {}
            }

            match determinant {
                Direction::Forward => {
                    sag_ileri.set_high(); sag_geri.set_low();
                    sol_ileri.set_high(); sol_geri.set_low();
                    ufmt::uwriteln!(&mut serial, "Yon: ILERI - Hiz: {}\r", sol_pwm.get_duty()).unwrap();
                }
                Direction::Backward => {
                    sag_ileri.set_low(); sag_geri.set_high();
                    sol_ileri.set_low(); sol_geri.set_high();
                    ufmt::uwriteln!(&mut serial, "Yon: GERI - Hiz: {}\r", sol_pwm.get_duty()).unwrap();
                }
                Direction::Left => {
                    sag_ileri.set_high(); sag_geri.set_low();
                    sol_ileri.set_low(); sol_geri.set_high();
                    ufmt::uwriteln!(&mut serial, "Yon: SOL - Hiz: {}\r", sol_pwm.get_duty()).unwrap();
                }
                Direction::Right => {
                    sag_ileri.set_low(); sag_geri.set_high();
                    sol_ileri.set_high(); sol_geri.set_low();
                    ufmt::uwriteln!(&mut serial, "Yon: SAG - Hiz: {}\r", sol_pwm.get_duty()).unwrap();
                }
                Direction::Stop => {
                    sag_ileri.set_low(); sag_geri.set_low();
                    sol_ileri.set_low(); sol_geri.set_low();
                    ufmt::uwriteln!(&mut serial, "Yon: DUR\r").unwrap();
                }
            }
        }
    }
}
