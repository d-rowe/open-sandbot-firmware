#![no_std]
#![no_main]

use arm::Arm;
use coordinate::PolarCoordinate;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::UART0;
use embassy_rp::uart;
use embassy_rp::{
    gpio::AnyPin,
    Peripheral,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use stepper_pair::StepperPairPins;
use uart_reader::reader_task;
use uart_writer::writer_task;
use {defmt_rtt as _, panic_probe as _};

mod arm;
mod coordinate;
mod stepper;
mod stepper_pair;
mod control_buffer;
mod coordinate_queue;
mod uart_reader;
mod uart_writer;

bind_interrupts!(struct Irqs {
    UART0_IRQ => uart::InterruptHandler<UART0>;
});

const MAX_POSITIONS: usize = 16384; // determines max size of queued pattern in positions
static POSITION_CHANNEL: Channel<ThreadModeRawMutex, PolarCoordinate, MAX_POSITIONS> =
    Channel::new();

static BAUDRATE: u32 = 115200;

#[embassy_executor::task]
async fn arm_task(stepper_pair_pins: StepperPairPins) {
    let mut arm = Arm::new(stepper_pair_pins);
    loop {
        let coordinate: PolarCoordinate = POSITION_CHANNEL.receive().await;
        arm.move_to(&coordinate).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut uart_config = uart::Config::default();
    uart_config.baudrate = BAUDRATE;
    let uart = uart::Uart::new(
        p.UART0,
        p.PIN_0,
        p.PIN_1,
        Irqs,
        p.DMA_CH0,
        p.DMA_CH1,
        uart_config,
    );

    let stepper_pair_pins = StepperPairPins {
        stepper0_step_pin: AnyPin::from(p.PIN_14).into_ref(),
        stepper0_dir_pin: AnyPin::from(p.PIN_15).into_ref(),
        stepper1_step_pin: AnyPin::from(p.PIN_12).into_ref(),
        stepper1_dir_pin: AnyPin::from(p.PIN_13).into_ref(),
        stepper_enable_pin: AnyPin::from(p.PIN_18).into_ref(),
    };

    let (tx, rx) = uart.split();

    spawner.spawn(reader_task(rx)).unwrap();
    spawner.spawn(writer_task(tx)).unwrap();
    spawner.spawn(arm_task(stepper_pair_pins)).unwrap();

    POSITION_CHANNEL
        .send(PolarCoordinate {
            theta: 0.0,
            rho: 1.0,
        })
        .await;
    POSITION_CHANNEL
        .send(PolarCoordinate {
            theta: 0.0,
            rho: 0.0,
        })
        .await;
}
