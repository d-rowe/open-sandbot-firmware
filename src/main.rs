#![no_std]
#![no_main]

extern crate alloc;

use defmt::println;
use crate::arm::Arm;
use crate::motion_controller::{MotionController, MotionControllerConfig};
use crate::coordinate::PolarCoordinate;
use embedded_alloc::Heap;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

mod motion_frame;
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
    let mut arm = Arm::new(p);
    let mut motion_controller = MotionController::new(MotionControllerConfig {
        home_position: PolarCoordinate { theta: 0.0, rho: 0.0 },
        max_speed: 20.0,
        min_speed: 1.0,
        max_acceleration: 10000.0,
        step_distance: 0.01,
    });

    motion_controller.queue_position(PolarCoordinate {
        theta: 0.0,
        rho: 1.0,
    });
    motion_controller.queue_position(PolarCoordinate {
        theta: 0.0,
        rho: 0.0,
    });

    loop {
        let frame = motion_controller.next_frame();
        arm.move_to_frame(&frame).await;
        Timer::after_millis(1).await;
    }
}