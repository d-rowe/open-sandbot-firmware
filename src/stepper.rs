use embassy_rp::gpio;
use embassy_rp::gpio::{AnyPin, Level};
use embassy_time::Timer;
use gpio::Output;

const STEP_COUNT: i64 = 4;

pub struct Stepper<'a> {
    out0: Output<'a, AnyPin>,
    out1: Output<'a, AnyPin>,
    out2: Output<'a, AnyPin>,
    out3: Output<'a, AnyPin>,
    speed: f64,
    current_step: i64,
}

impl Stepper<'_> {
    pub fn new<'a>(
        pin0: AnyPin,
        pin1: AnyPin,
        pin2: AnyPin,
        pin3: AnyPin,
    ) -> Stepper<'a> {
        Stepper {
            out0: create_output(pin0),
            out1: create_output(pin1),
            out2: create_output(pin2),
            out3: create_output(pin3),
            speed: 20.0,
            current_step: 0,
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed.min(100.0).max(1.0);
    }

    pub async fn step(&mut self, steps: i64) {
        let direction = steps / steps.abs();
        let micros_delay = (200000.0 / self.speed) - 1000.0;
        for _ in 0..steps.abs() {
            self.step_once(direction);
            Timer::after_micros(micros_delay as u64).await;
        }
    }

    fn step_once(&mut self, direction: i64) {
        self.current_step = (self.current_step + direction + STEP_COUNT) % STEP_COUNT;

        match self.current_step {
            0 => {
                self.out0.set_high();
                self.out1.set_low();
                self.out2.set_high();
                self.out3.set_low();
            },
            1 => {
                self.out0.set_low();
                self.out1.set_high();
                self.out2.set_high();
                self.out3.set_low();
            },
            2 => {
                self.out0.set_low();
                self.out1.set_high();
                self.out2.set_low();
                self.out3.set_high();
            },
            3 => {
                self.out0.set_high();
                self.out1.set_low();
                self.out2.set_low();
                self.out3.set_high();
            },
            _ => (),
        }
    }
}

fn create_output(pin: AnyPin) -> Output<'static, AnyPin> {
    Output::new(pin, Level::Low)
}
