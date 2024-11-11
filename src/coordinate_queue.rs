use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

use crate::coordinate::PolarCoordinate;

static COORDINATE_CHANNEL: Channel<ThreadModeRawMutex, PolarCoordinate, 512> = Channel::new();

pub async fn queue(coordinate: PolarCoordinate) {
    let _ = COORDINATE_CHANNEL.send(coordinate).await;
}

pub async fn dequeue() -> PolarCoordinate {
    COORDINATE_CHANNEL.receive().await
}
