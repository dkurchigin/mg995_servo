use arduino_hal::hal::port::PB1;
use arduino_hal::port::mode::PwmOutput;
use arduino_hal::simple_pwm::Timer1Pwm;
use arduino_hal::port::Pin;

const LEFT_DUTY: u8 = 12;
const STOP_DUTY: u8 = 22;
const RIGHT_DUTY: u8 = 32;

struct MG995 {
    pin: Pin<PwmOutput<Timer1Pwm>, PB1>,
    left_duty: u8,
    stop_duty: u8,
    right_duty: u8
}

impl MG995 {
    fn new(pin: Pin<PwmOutput<Timer1Pwm>, PB1>, left_duty: u8, stop_duty:u8, right_duty: u8) -> MG995 {
        MG995 { pin, left_duty, stop_duty, right_duty }
    }

    fn enable(&mut self) {
        self.pin.enable()
    }

    fn disable(&mut self) {
        self.pin.disable()
    }

    fn stop(&mut self) {
        self.pin.set_duty(self.stop_duty)
    }

    fn left(&mut self) {
        self.pin.set_duty(self.left_duty)
    }

    fn right(&mut self) {
        self.pin.set_duty(self.right_duty)
    }
}
