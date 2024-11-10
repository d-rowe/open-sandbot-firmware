use embassy_rp::{peripherals::UART0, uart::{Async, UartTx}};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn writer_task(mut tx: UartTx<'static, UART0, Async>) {
    loop {
        let _ = tx.write("Hi from the UART writer\r\n".as_bytes()).await;
        Timer::after_secs(30).await;
    }
}
