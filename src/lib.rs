//! # MG995 Servo
//!
//! A library for MG995 servomotor
//! Minimal functionality, easy to use

#![no_std]
use arduino_hal::hal::port::PB1;
use arduino_hal::port::mode::PwmOutput;
use arduino_hal::simple_pwm::Timer1Pwm;
use arduino_hal::port::Pin;

pub const LEFT_DUTY: u8 = 12;
pub const STOP_DUTY: u8 = 22;
pub const RIGHT_DUTY: u8 = 32;

pub struct MG995 {
    pin: Pin<PwmOutput<Timer1Pwm>, PB1>,
    left_duty: u8,
    stop_duty: u8,
    right_duty: u8
}

/// # Example
///
/// ```
///     let dp = arduino_hal::Peripherals::take().unwrap();
///     let pins = arduino_hal::pins!(dp);
///     let timer1 = Timer1Pwm::new(dp.TC1, simple_pwm::Prescaler::Prescale1024);
///     let servo_pin = pins.d9.into_output().into_pwm(&timer1);
///
///     let mut mg995 = MG995::new(servo_pin, STOP_DUTY, LEFT_DUTY, RIGHT_DUTY);
///     mg995.enable();
///     loop {
///         mg995.stop();
///         arduino_hal::delay_ms(500);
///         mg995.right();
///         arduino_hal::delay_ms(500);
///         mg995.stop();
///         arduino_hal::delay_ms(500);
///         mg995.left();
///         arduino_hal::delay_ms(500);
///     }
/// ```
impl MG995 {
    pub fn new(pin: Pin<PwmOutput<Timer1Pwm>, PB1>, left_duty: u8, stop_duty:u8, right_duty: u8) -> MG995 {
        MG995 { pin, left_duty, stop_duty, right_duty }
    }

    pub fn enable(&mut self) {
        self.pin.enable()
    }

    pub fn disable(&mut self) {
        self.pin.disable()
    }

    pub fn stop(&mut self) {
        self.pin.set_duty(self.stop_duty)
    }

    pub fn left(&mut self) {
        self.pin.set_duty(self.left_duty)
    }

    pub fn right(&mut self) {
        self.pin.set_duty(self.right_duty)
    }
}
