use embassy_rp::{peripherals::UART0, uart::{Async, UartRx}};

use crate::{command::Command, coordinate::PolarCoordinate, coordinate_queue, transmission_channel};

static MOVE: &str = "MOVE";

#[embassy_executor::task]
pub async fn reader_task(mut rx: UartRx<'static, UART0, Async>) {
    loop {
        let mut command = Command::new();
        loop {
            let mut char_buf = [0u8];
            let _rr = rx.read(&mut char_buf).await;
            let _br = command.add_char_buf(&char_buf);
            if command.is_complete() {
                break;
            }
        }

        let input = command.to_str().unwrap();
        let mut args = input.split(' ');
        let method = args.next().unwrap();

        if method == MOVE {
            transmission_channel::send("MOVE ACK;").await;
            let theta_str = args.next().unwrap();
            let rho_str = args.next().unwrap();
            coordinate_queue::queue(PolarCoordinate {
                theta: theta_str.parse().unwrap(),
                rho: rho_str.parse().unwrap(),
            }).await;
        }
    }
}