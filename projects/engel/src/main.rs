#![no_std]
#![no_main]

use arduino_hal::{
    entry,
    prelude::*,
    simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm, Timer2Pwm},
};
use panic_halt as _;

/// Mesafe sensörü okuması (HC-SR04 için). delay referansı ile çalışır.
fn read_distance<Trig, Echo>(trigger: &mut Trig, echo: &mut Echo, delay: &mut arduino_hal::Delay) -> u32
where
    Trig: embedded_hal::digital::OutputPin,
    Echo: embedded_hal::digital::InputPin,
{
    // Trigger pulse
    trigger.set_low().ok();
    delay.delay_us(20u16);
    trigger.set_high().ok();
    delay.delay_us(100u16);
    trigger.set_low().ok();

    let mut timeout = 5u32;

    // Echo yükselene kadar bekle (timeout koruması)
    while echo.is_low().unwrap_or(false) {
        timeout += 1;
        if timeout > 25_000 {
            return 0; // timeout
        }
        delay.delay_us(5u16);
    }

    // Echo HIGH süresini ölç
    let mut dur = 0u32;
    while echo.is_high().unwrap_or(false) {
        dur += 1;
        if dur > 60_000 {
            return 0;
        }
        delay.delay_us(5u16);
    }

    // Süreyi cm'ye çevir (yaklaşık)
    dur / 58
}

#[entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Delay
    let mut delay = arduino_hal::Delay::new();

    // Timers (bir kez oluştur)
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64); // D9 için (OC1A/OC1B)
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64); // D3 için

    // Mesafe sensörleri
    let mut left_trigger = pins.a5.into_output();
    let mut left_echo = pins.a4.into_floating_input();
    let mut front_trigger = pins.a3.into_output();
    let mut front_echo = pins.a2.into_floating_input();
    let mut right_trigger = pins.a1.into_output();
    let mut right_echo = pins.a0.into_floating_input();

    // Motor kontrol pinleri (IN pinleri)
    let mut left_in1 = pins.d4.into_output();
    let mut left_in2 = pins.d5.into_output();
    let mut right_in3 = pins.d7.into_output();
    let mut right_in4 = pins.d6.into_output();

    // PWM pinlerini bir kez oluştur (ENA/ENB)
    let left_ena_pin = pins.d9.into_output();
    let right_enb_pin = pins.d3.into_output();
    let mut left_ena = left_ena_pin.into_pwm(&timer1);
    let mut right_enb = right_enb_pin.into_pwm(&timer2);
    left_ena.enable();
    right_enb.enable();
     
    // Varsayılan hız (0..=255)
    let speed: u8 = 250u8;

    loop {
       
        // Mesafeleri oku
        let left_dist = read_distance(&mut left_trigger, &mut left_echo, &mut delay);
        let front_dist = read_distance(&mut front_trigger, &mut front_echo, &mut delay);
        let right_dist = read_distance(&mut right_trigger, &mut right_echo, &mut delay);
        ufmt::uwriteln!(&mut serial, "L:{} F:{} R:{}", left_dist, front_dist, right_dist).unwrap();

        if (front_dist < 20 && front_dist != 0)
            || (left_dist < 20 && left_dist != 0)
            || (right_dist < 20 && right_dist != 0) {
            // Ön engel: dur
            left_in1.set_high();
            left_in2.set_low();
            right_in3.set_high();
            right_in4.set_low();
            left_ena.set_duty(0);
            right_enb.set_duty(0);
            delay.delay_ms(150u16);

            // Geri Git
            left_in1.set_low();
            left_in2.set_high();
            right_in3.set_low();
            right_in4.set_high();
            left_ena.set_duty(speed);
            right_enb.set_duty(speed);
            delay.delay_ms(150u16);

            // dur
            left_in1.set_low();
            left_in2.set_low();
            right_in3.set_low();
            right_in4.set_low();
            left_ena.set_duty(0);
            right_enb.set_duty(0);
            delay.delay_ms(100u16);

            // Daha açık tarafa dön
            if left_dist > right_dist {
                // sola dön: sol geri, sağ ileri
                left_in1.set_low();
                left_in2.set_high();
                right_in3.set_high();
                right_in4.set_low();
            } else {
                // sağa dön: sol ileri, sağ geri
                left_in1.set_high();
                left_in2.set_low();
                right_in3.set_low();
                right_in4.set_high();
            }

            // dönüş için hız uygula
            left_ena.set_duty(speed);
            right_enb.set_duty(speed);
            delay.delay_ms(300u16);

            // dur
            left_in1.set_low();
            left_in2.set_low();
            right_in3.set_low();
            right_in4.set_low();
            left_ena.set_duty(0);
            right_enb.set_duty(0);
            delay.delay_ms(150u16);
        } else {
            // İleri git
            left_in1.set_high();
            left_in2.set_low();
            right_in3.set_high();
            right_in4.set_low();
            left_ena.set_duty(speed);
            right_enb.set_duty(speed);
        }

        delay.delay_ms(100u16);
    }
}
