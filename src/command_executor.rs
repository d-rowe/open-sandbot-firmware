use core::str::Split;

use crate::{
    command_buffer::CommandBuffer, coordinate::PolarCoordinate, coordinate_queue,
    transmission_channel,
};

const MOVE: &str = "MOVE";

pub async fn execute(command_buf: CommandBuffer) {
    let mut args = command_buf.to_str().unwrap().split(' ');
    let method = args.next().unwrap();

    match method {
        MOVE => execute_move(args).await,
        _ => {}
    };
}

async fn execute_move(mut args: Split<'_, char>) {
    transmission_channel::send("MOVE ACK\r\n").await;
    let theta_str = args.next().unwrap();
    let rho_str = args.next().unwrap();
    coordinate_queue::queue(PolarCoordinate {
        theta: theta_str.parse::<f64>().unwrap(),
        rho: rho_str.parse::<f64>().unwrap(),
    })
    .await;
}
