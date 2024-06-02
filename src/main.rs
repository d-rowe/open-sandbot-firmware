use std::collections::VecDeque;
use crate::coordinate::PolarCoordinate;
use crate::motion_controller::{MotionController, MotionControllerConfig};

mod coordinate;
mod motion_controller;

fn main() {
    let mut positions: VecDeque<PolarCoordinate> = VecDeque::new();
    positions.push_back(PolarCoordinate { theta: 1.0, rho: 1.0 });

    positions.push_back(PolarCoordinate { theta: 2.0, rho: 1.0 });
    positions.push_back(PolarCoordinate { theta: 100.0, rho: 1.0 });
    positions.push_back(PolarCoordinate { theta: 1.0, rho: 0.9 });
    let _motion_controller = MotionController::new(MotionControllerConfig {
        home_position: PolarCoordinate { theta: 0.0, rho: 0.0},
        max_acceleration: 1.0,
        max_speed: 100.0,
        min_speed: 1.0,
        step_distance: 0.05,
    });
    println!("done");
}

