use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

static TRANSMISSION_CHANNEL: Channel<ThreadModeRawMutex, &str, 8> = Channel::new();

pub async fn send(msg: &'static str) {
    TRANSMISSION_CHANNEL.send(msg).await
}

pub async fn receive()-> &'static str {
    TRANSMISSION_CHANNEL.receive().await
}