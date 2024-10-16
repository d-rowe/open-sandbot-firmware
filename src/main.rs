#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};
use crate::stepper_pair::StepperPair;

mod coordinate;
mod stepper;
mod arm;
mod stepper_pair;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut stepper_pair = StepperPair::new(p);
    stepper_pair.move_to(500, 2000).await;
}
