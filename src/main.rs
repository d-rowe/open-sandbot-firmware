#![no_std]
#![no_main]

use arm::Arm;
use coordinate::PolarCoordinate;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

mod arm;
mod coordinate;
mod stepper;
mod stepper_pair;

static POSITION_CHANNEL: Channel<ThreadModeRawMutex, &PolarCoordinate, 10> = Channel::new();

#[embassy_executor::task]
async fn arm_task() {
    let mut arm = Arm::new();
    loop {
        let coordinate: &PolarCoordinate = POSITION_CHANNEL.receive().await;
        arm.move_to(coordinate).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(arm_task()).unwrap();

    Timer::after_secs(5).await;
    POSITION_CHANNEL
        .send(&PolarCoordinate {
            theta: 0.0,
            rho: 1.0,
        })
        .await;
    POSITION_CHANNEL
        .send(&PolarCoordinate {
            theta: 0.0,
            rho: 0.0,
        })
        .await;
}
