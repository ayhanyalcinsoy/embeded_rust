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

    // Motor kontrol pinleri
    let mut sag_ileri = pins.d2.into_output();  // Sağ motor ileri
    let mut sag_geri = pins.d4.into_output();   // Sağ motor geri
    let mut sol_ileri = pins.d7.into_output();  // Sol motor ileri
    let mut sol_geri = pins.d8.into_output();   // Sol motor geri

    // DEBUG: Test için LED
    let mut led = pins.d13.into_output();
    led.set_low();

    // PWM timerlarını doğru şekilde başlat
    // D10 için Timer1, D11 için Timer2 kullanılıyor
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    // PWM pinlerini başlat - ÖNEMLİ: Önce output'a çevir, sonra PWM'e
    let mut sol_pwm = pins.d10.into_output().into_pwm(&timer1);  // D10 - Sol motor PWM
    let mut sag_pwm = pins.d11.into_output().into_pwm(&timer2);  // D11 - Sağ motor PWM

    // PWM'leri etkinleştir
    sol_pwm.enable();
    sag_pwm.enable();
    
    // Başlangıçta %50 duty cycle (127/255)
    sol_pwm.set_duty(127);
    sag_pwm.set_duty(127);

    ufmt::uwriteln!(&mut serial, "Robot Baslatiliyor...\r").unwrap();
    ufmt::uwriteln!(&mut serial, "PWM Pinleri: D10(Sol), D11(Sag)\r").unwrap();
    ufmt::uwriteln!(&mut serial, "Kontrol Pinleri: D2, D4, D7, D8\r").unwrap();

    // Test için LED'i yak
    for _ in 0..3 {
        led.toggle();
        arduino_hal::delay_ms(200);
    }
    led.set_low();

    let mut determinant = Direction::Stop;

    loop {
        // Serial'den komut oku
        if let Ok(c) = nb::block!(serial.read()) {
            // Gelen karakteri echo
            ufmt::uwrite!(&mut serial, "Gelen: {}\r", c as char).unwrap();
            
            match c {
                b'F' => determinant = Direction::Forward,
                b'B' => determinant = Direction::Backward,
                b'L' => determinant = Direction::Left,
                b'R' => determinant = Direction::Right,
                b'S' => determinant = Direction::Stop,
                b'0'..=b'9' => {
                    let hiz_seviye = (c - b'0') as u8;
                    let yeni_hiz = hiz_seviye * 25 + 50; // 50-275 arası
                    let clamped_hiz = yeni_hiz.min(255);
                    
                    sol_pwm.set_duty(clamped_hiz);
                    sag_pwm.set_duty(clamped_hiz);
                    
                    ufmt::uwriteln!(&mut serial, "Hiz: {}/255\r", clamped_hiz).unwrap();
                    continue;
                }
                b'+' => {
                    let mevcut_hiz = sol_pwm.get_duty();
                    let yeni_hiz = if mevcut_hiz < 230 { mevcut_hiz + 25 } else { 255 };
                    sol_pwm.set_duty(yeni_hiz);
                    sag_pwm.set_duty(yeni_hiz);
                    ufmt::uwriteln!(&mut serial, "Hiz ARTIR: {}/255\r", yeni_hiz).unwrap();
                    continue;
                }
                b'-' => {
                    let mevcut_hiz = sol_pwm.get_duty();
                    let yeni_hiz = if mevcut_hiz > 75 { mevcut_hiz - 25 } else { 50 };
                    sol_pwm.set_duty(yeni_hiz);
                    sag_pwm.set_duty(yeni_hiz);
                    ufmt::uwriteln!(&mut serial, "Hiz AZALT: {}/255\r", yeni_hiz).unwrap();
                    continue;
                }
                /*b'T' => {
                    // TEST: PWM çıkışlarını test et
                    ufmt::uwriteln!(&mut serial, "PWM Test Basliyor...\r").unwrap();
                    for duty in (50..=250).step_by(50) {
                        sol_pwm.set_duty(duty);
                        sag_pwm.set_duty(duty);
                        ufmt::uwriteln!(&mut serial, "Duty Cycle: {}/255\r", duty).unwrap();
                        arduino_hal::delay_ms(1000);
                    }
                    sol_pwm.set_duty(127);
                    sag_pwm.set_duty(127);
                    continue;
                }*/
                _ => {
                    ufmt::uwriteln!(&mut serial, "Gecersiz komut: {}\r", c as char).unwrap();
                    continue;
                }
            }

            // Yön kontrolü
            match determinant {
                Direction::Forward => {
                    sag_ileri.set_high(); sag_geri.set_low();
                    sol_ileri.set_high(); sol_geri.set_low();
                    ufmt::uwriteln!(&mut serial, "ILERI - Hiz: {}/255\r", sol_pwm.get_duty()).unwrap();
                }
                Direction::Backward => {
                    sag_ileri.set_low(); sag_geri.set_high();
                    sol_ileri.set_low(); sol_geri.set_high();
                    ufmt::uwriteln!(&mut serial, "GERI - Hiz: {}/255\r", sol_pwm.get_duty()).unwrap();
                }
                Direction::Left => {
                    sag_ileri.set_high(); sag_geri.set_low();
                    sol_ileri.set_low(); sol_geri.set_high();
                    ufmt::uwriteln!(&mut serial, "SOL - Hiz: {}/255\r", sol_pwm.get_duty()).unwrap();
                }
                Direction::Right => {
                    sag_ileri.set_low(); sag_geri.set_high();
                    sol_ileri.set_high(); sol_geri.set_low();
                    ufmt::uwriteln!(&mut serial, "SAG - Hiz: {}/255\r", sol_pwm.get_duty()).unwrap();
                }
                Direction::Stop => {
                    sag_ileri.set_low(); sag_geri.set_low();
                    sol_ileri.set_low(); sol_geri.set_low();
                    ufmt::uwriteln!(&mut serial, "DUR\r").unwrap();
                }
            }
            
            // LED'i duruma göre yak
            led.toggle();
        }
        
        // Küçük bir gecikme
        arduino_hal::delay_ms(10);
    }
}
