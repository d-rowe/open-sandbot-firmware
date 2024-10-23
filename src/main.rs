#![no_std]
#![no_main]

use arm::Arm;
use coordinate::PolarCoordinate;
use embassy_executor::Spawner;
use embassy_rp::{gpio::AnyPin, Peripheral};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use stepper_pair::StepperPairPins;
use {defmt_rtt as _, panic_probe as _};

mod arm;
mod coordinate;
mod stepper;
mod stepper_pair;

static POSITION_CHANNEL: Channel<ThreadModeRawMutex, &PolarCoordinate, 10> = Channel::new();

#[embassy_executor::task]
async fn arm_worker(stepper_pair_pins: StepperPairPins) {
    let mut arm = Arm::new(stepper_pair_pins);
    loop {
        let coordinate: &PolarCoordinate = POSITION_CHANNEL.receive().await;
        arm.move_to(coordinate).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    spawner
        .spawn(arm_worker(StepperPairPins {
            stepper0_step_pin: AnyPin::from(p.PIN_14).into_ref(),
            stepper0_dir_pin: AnyPin::from(p.PIN_15).into_ref(),
            stepper1_step_pin: AnyPin::from(p.PIN_12).into_ref(),
            stepper1_dir_pin: AnyPin::from(p.PIN_13).into_ref(),
            stepper_enable_pin: AnyPin::from(p.PIN_18).into_ref(),
        }))
        .unwrap();

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
