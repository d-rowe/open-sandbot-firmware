use defmt::info;
use embassy_rp::{peripherals::UART0, uart::{Async, UartTx}};

use crate::transmission_channel;

#[embassy_executor::task]
pub async fn writer_task(mut tx: UartTx<'static, UART0, Async>) {
    loop {
        info!("waiting for message");
        let msg = transmission_channel::receive().await;
        info!("received {}", msg);
        let _ = tx.write(msg.as_bytes()).await;
    }
}
