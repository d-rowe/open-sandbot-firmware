#![no_std]
#![no_main]

use arm::Arm;
use coordinate::PolarCoordinate;
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::UART0;
use embassy_rp::uart;
use embassy_rp::{
    gpio::AnyPin,
    uart::{Async, UartRx, UartTx},
    Peripheral,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Timer;
use stepper_pair::StepperPairPins;
use control_buffer::ControlBuffer;
use {defmt_rtt as _, panic_probe as _};

mod arm;
mod coordinate;
mod stepper;
mod stepper_pair;
mod control_buffer;
mod coordinate_queue;

bind_interrupts!(struct Irqs {
    UART0_IRQ => uart::InterruptHandler<UART0>;
});

const MAX_POSITIONS: usize = 16384; // determines max size of queued pattern in positions
static POSITION_CHANNEL: Channel<ThreadModeRawMutex, PolarCoordinate, MAX_POSITIONS> =
    Channel::new();

static BAUDRATE: u32 = 115200;

#[embassy_executor::task]
async fn arm_worker(stepper_pair_pins: StepperPairPins) {
    let mut arm = Arm::new(stepper_pair_pins);
    loop {
        let coordinate: PolarCoordinate = POSITION_CHANNEL.receive().await;
        arm.move_to(&coordinate).await;
    }
}

#[embassy_executor::task]
async fn uart_reader(mut rx: UartRx<'static, UART0, Async>) {
    loop {
        let mut control_buffer = ControlBuffer::new();
        loop {
            let mut char_buf = [0u8];
            let _rr = rx.read(&mut char_buf).await;
            let _br = control_buffer.add_char_buf(&char_buf);
            if control_buffer.is_complete() {
                break;
            }
        }

        let input = control_buffer.to_str().unwrap();
        let mut args = input.split(' ');
        let command = args.next().unwrap();

        info!("{}", command);
        if command == "MOVE" {
            let theta_str = args.next().unwrap();
            let rho_str = args.next().unwrap();
            info!("Move to {}, {}", theta_str, rho_str);
        }
    }
}

#[embassy_executor::task]
async fn uart_writer(mut tx: UartTx<'static, UART0, Async>) {
    loop {
        let _ = tx.write("Hi from the UART writer\r\n".as_bytes()).await;
        Timer::after_secs(30).await;
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

    let (tx, rx) = uart.split();

    spawner.spawn(uart_reader(rx)).unwrap();
    spawner.spawn(uart_writer(tx)).unwrap();

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
