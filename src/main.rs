use crate::coordinate::PolarCoordinate;
use crate::motion_controller::MotionController;

mod coordinate;
mod motion_controller;

fn main() {
    let coordinate_0 = PolarCoordinate { theta: 0.0, rho: 1.0 };
    let coordinate_1 = PolarCoordinate { theta: 10.0, rho: 1.0 };
    let coordinate_2 = PolarCoordinate { theta: 10.0, rho: 0.5 };
    let coordinate_3 = PolarCoordinate { theta: 1.0, rho: 0.9 };
    let mut motion_controller = MotionController::new();
    motion_controller.queue_position(coordinate_0);
    motion_controller.queue_position(coordinate_1);
    motion_controller.queue_position(coordinate_2);
    motion_controller.queue_position(coordinate_3);
    let frame_0 = motion_controller.next_frame();
    let frame_1 = motion_controller.next_frame();
    let frame_2 = motion_controller.next_frame();
    let frame_3 = motion_controller.next_frame();
    println!("done");
}

