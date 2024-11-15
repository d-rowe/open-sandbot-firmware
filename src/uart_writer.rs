use defmt::info;
use embassy_rp::{peripherals::UART0, uart::{Async, UartTx}};

use crate::transmission_channel;

#[embassy_executor::task]
pub async fn writer_task(mut tx: UartTx<'static, UART0, Async>) {
    loop {
        let msg = transmission_channel::receive().await;
        info!("sending message: {}", msg);
        let mut new_lined_buf = [0u8; 16];
        let mut new_lined_buf_idx: usize = 0;
        for byte in msg.as_bytes() {
            new_lined_buf[new_lined_buf_idx] = *byte;
            new_lined_buf_idx += 1;
        }

        // add \n
        new_lined_buf[new_lined_buf_idx] = 10;
        let _ = tx.write(&new_lined_buf).await;
    }
}
