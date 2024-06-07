#![no_std]
#![no_main]

extern crate alloc;

use crate::stepper::Stepper;
use crate::arm::Arm;
use crate::motion_controller::{MotionController, MotionControllerConfig};
use crate::coordinate::PolarCoordinate;
use embedded_alloc::Heap;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

mod common;
mod stepper;
mod coordinate;
mod motion_controller;
mod arm;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn init_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    init_heap();
    let p = embassy_rp::init(Default::default());
    let ain_2 = Output::new(p.PIN_18, Level::Low);
    let ain_1 = Output::new(p.PIN_19, Level::Low);
    let bin_1 = Output::new(p.PIN_20, Level::Low);
    let bin_2 = Output::new(p.PIN_21, Level::Low);
    let ain_4 = Output::new(p.PIN_10, Level::Low);
    let ain_3 = Output::new(p.PIN_11, Level::Low);
    let bin_3 = Output::new(p.PIN_12, Level::Low);
    let bin_4 = Output::new(p.PIN_13, Level::Low);
    let mut arm = Arm::new(
        Stepper::new(ain_2, ain_1, bin_1, bin_2),
        Stepper::new(ain_4, ain_3, bin_3, bin_4),
    );

    let mut motion_controller = MotionController::new(MotionControllerConfig {
        home_position: PolarCoordinate { theta: 0.0, rho: 0.0 },
        max_speed: 20.0,
        min_speed: 1.0,
        max_acceleration: 500.0,
        step_distance: 0.02,
    });

    motion_controller.queue_position(PolarCoordinate {
        theta: 0.0,
        rho: 1.0,
    });
    motion_controller.queue_position(PolarCoordinate {
        theta: 5.0,
        rho: 0.3,
    });
    motion_controller.queue_position(PolarCoordinate {
        theta: 0.0,
        rho: 0.0,
    });

    loop {
        let frame = motion_controller.next_frame();
        arm.move_to_frame(&frame).await;
    }
}