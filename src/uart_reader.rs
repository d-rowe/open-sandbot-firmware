use embassy_rp::{peripherals::UART0, uart::{Async, UartRx}};

use crate::{command_buffer::CommandBuffer, command_executor};

#[embassy_executor::task]
pub async fn reader_task(mut rx: UartRx<'static, UART0, Async>) {
    loop {
        let mut command_buffer = CommandBuffer::new();
        loop {
            let mut char_buf = [0u8];
            let _rr = rx.read(&mut char_buf).await;
            let _br = command_buffer.add_char_buf(&char_buf);
            if command_buffer.is_complete() {
                break;
            }
        }

        command_executor::execute(command_buffer).await;
    }
}