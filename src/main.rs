#![no_std]
#![no_main]

use arm::Arm;
use coordinate::PolarCoordinate;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

mod arm;
mod coordinate;
mod stepper;
mod stepper_pair;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut arm = Arm::new();
    loop {
        arm.move_to(&PolarCoordinate {
            theta: 0.0,
            rho: 1.0,
        })
        .await;
        arm.move_to(&PolarCoordinate {
            theta: 1.57,
            rho: 0.5,
        })
        .await;
        arm.move_to(&PolarCoordinate {
            theta: 0.0,
            rho: 0.0,
        })
        .await;
        Timer::after_secs(5).await;
    }
}
