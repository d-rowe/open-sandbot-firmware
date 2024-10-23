use crate::stepper::Stepper;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_rp::Peripherals;
use embassy_time::Timer;
use libm::fabs;

pub struct StepperPair<'a> {
    stepper_0: Stepper<'a>,
    stepper_1: Stepper<'a>,
    enable_output: Output<'a, AnyPin>,
}

impl StepperPair<'_> {
    pub fn new(p: Peripherals) -> Self {
        StepperPair {
            stepper_0: Stepper::new(AnyPin::from(p.PIN_14), AnyPin::from(p.PIN_15)),
            stepper_1: Stepper::new(AnyPin::from(p.PIN_12), AnyPin::from(p.PIN_13)),
            enable_output: Output::new(AnyPin::from(p.PIN_18), Level::Low),
        }
    }

    pub async fn move_to(&mut self, stepper_0_steps: i64, stepper_1_steps: i64) {
        self.engage();
        let is_s0_forward = stepper_0_steps >= 0;
        let is_s1_forward = stepper_1_steps < 0;
        let s0_steps_abs = abs(stepper_0_steps);
        let s1_steps_abs = abs(stepper_1_steps);
        let s0_ratio = (s0_steps_abs as f64 / (s1_steps_abs as f64).max(1.0)).min(1.0);
        let s1_ratio = (s1_steps_abs as f64 / (s0_steps_abs as f64).max(1.0)).min(1.0);

        let mut s0_partial_steps = 0.0;
        let mut s1_partial_steps = 0.0;
        let mut s0_steps = 0;
        let mut s1_steps = 0;

        while s0_steps_abs - s0_steps > 0 && s1_steps_abs - s1_steps > 0 {
            s0_partial_steps += s0_ratio;
            s1_partial_steps += s1_ratio;

            if s0_partial_steps >= 1.0 {
                self.stepper_0.step(is_s0_forward);
                s0_steps += 1;
                s0_partial_steps -= 1.0;
            }

            if s1_partial_steps >= 1.0 {
                self.stepper_1.step(is_s1_forward);
                s1_steps += 1;
                s1_partial_steps -= 1.0;
            }

            Timer::after_millis(5).await;
        }

        self.disengage();
    }

    fn engage(&mut self) {
        self.enable_output.set_low();
    }

    fn disengage(&mut self) {
        self.enable_output.set_high();
    }
}

fn abs(val: i64) -> i64 {
    fabs(val as f64) as i64
}