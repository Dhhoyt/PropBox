use std::{io};

use tokio::{net::TcpListener, runtime::Runtime, time::{Instant, Duration, sleep_until}};

#[tokio::main]
async fn main() {
    game_loop().await;
}

async fn game_loop() {
    let max_fps = 20.0;
    let frame_time = Duration::from_secs_f32(1.0/max_fps);
    let mut next_frame = Instant::now();
    loop {
        sleep_until(next_frame).await;
        next_frame += frame_time;
        
        //update();
    }
}
