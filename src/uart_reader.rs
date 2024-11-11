use defmt::info;
use embassy_rp::{peripherals::UART0, uart::{Async, UartRx}};

use crate::{control_buffer::ControlBuffer, coordinate::PolarCoordinate, coordinate_queue, transmission_channel};

static MOVE: &str = "MOVE";

#[embassy_executor::task]
pub async fn reader_task(mut rx: UartRx<'static, UART0, Async>) {
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

        if command == MOVE {
            transmission_channel::send("MOVE ACK\r\n").await;
            let theta_str = args.next().unwrap();
            let rho_str = args.next().unwrap();
            // TODO: parse rho
            coordinate_queue::queue(PolarCoordinate {
                theta: theta_str.parse::<f64>().unwrap(),
                rho: 0.0,
            }).await;
            info!("Move to {}, {}", theta_str, rho_str);
        }
    }
}