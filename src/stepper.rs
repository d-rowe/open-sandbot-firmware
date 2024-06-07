use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::Output;

const STEP_COUNT: i64 = 4;

pub struct Stepper<'a> {
    pin0: Output<'a>,
    pin1: Output<'a>,
    pin2: Output<'a>,
    pin3: Output<'a>,
    speed: f64,
    current_step: i64,
}

impl Stepper<'_> {
    pub fn new<'a>(
        pin0: Output<'a>,
        pin1: Output<'a>,
        pin2: Output<'a>,
        pin3: Output<'a>,
    ) -> Stepper<'a> {
        Stepper {
            pin0 ,
            pin1,
            pin2,
            pin3,
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
                self.pin0.set_high();
                self.pin1.set_low();
                self.pin2.set_high();
                self.pin3.set_low();
            },
            1 => {
                self.pin0.set_low();
                self.pin1.set_high();
                self.pin2.set_high();
                self.pin3.set_low();
            },
            2 => {
                self.pin0.set_low();
                self.pin1.set_high();
                self.pin2.set_low();
                self.pin3.set_high();
            },
            3 => {
                self.pin0.set_high();
                self.pin1.set_low();
                self.pin2.set_low();
                self.pin3.set_high();
            },
            _ => panic!("current step can only be 0-3"),
        }
    }
}
