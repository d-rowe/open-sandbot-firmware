use crate::coordinate::PolarCoordinate;
use crate::stepper_pair::StepperPair;
use core::f64::consts::PI;
use libm::{acos, fabs, pow, round};

const MAIN_PULLEY_TEETH: f64 = 90.0;
const MOTOR_PULLEY_TEETH: f64 = 16.0;
const DEGREES_PER_STEP: f64 = 1.8;
const MICRO_STEPS: f64 = 16.0;

const STEPS_PER_DEG: f64 = MICRO_STEPS * MAIN_PULLEY_TEETH / MOTOR_PULLEY_TEETH / DEGREES_PER_STEP;

struct StepPosition {
    primary_steps: i64,
    secondary_steps: i64,
}

impl StepPosition {
    pub fn get_total_steps(&self) -> i64 {
        let primary_total_steps = abs(self.primary_steps);
        let secondary_total_steps = abs(self.secondary_steps);
        primary_total_steps + secondary_total_steps
    }

    pub fn delta(&self, other: &StepPosition) -> StepPosition {
        StepPosition {
            primary_steps: self.primary_steps - other.primary_steps,
            secondary_steps: self.secondary_steps - other.secondary_steps,
        }
    }
}

pub struct Arm<'a> {
    step_position: StepPosition,
    stepper_pair: StepperPair<'a>,
}

impl Arm<'_> {
    pub fn new() -> Self {
        let p = embassy_rp::init(Default::default());
        Arm {
            step_position: get_target_step_position(&PolarCoordinate {
                theta: 0.0,
                rho: 0.0,
            }),
            stepper_pair: StepperPair::new(p),
        }
    }

    pub async fn move_to(&mut self, position: &PolarCoordinate) {
        let target_step_position = get_target_step_position(position);
        let delta_step_position = target_step_position.delta(&self.step_position);

        if delta_step_position.get_total_steps() == 0 {
            // already at target position
            return;
        }

        self.stepper_pair
            .move_to(
                delta_step_position.primary_steps,
                delta_step_position.secondary_steps,
            )
            .await;

        self.step_position = target_step_position;
    }
}

fn get_target_step_position(position: &PolarCoordinate) -> StepPosition {
    let PolarCoordinate { theta, rho } = position;
    let theta_degrees = -degrees(*theta);
    let secondary_degrees = 180.0 - degrees(acos((0.5 - pow(*rho, 2.0)) * 2.0));
    let primary_offset = secondary_degrees / 2.0;
    let primary_degrees = theta_degrees - primary_offset;

    let primary_steps = round(primary_degrees * STEPS_PER_DEG) as i64;
    StepPosition {
        primary_steps,
        secondary_steps: round(secondary_degrees * STEPS_PER_DEG) as i64 + primary_steps,
    }
}

fn degrees(radians: f64) -> f64 {
    radians * (180.0 / PI)
}

fn abs(val: i64) -> i64 {
    fabs(val as f64) as i64
}
