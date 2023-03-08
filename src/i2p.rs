use log::{debug, info};
use std::process::Command;

pub async fn start() {
    info!("starting i2p");
    let output = Command::new("i2p-zero-linux.v1.20/router/bin/i2p-zero")
        .spawn()
        .expect("i2p-zero failed to start");
    debug!("{:?}", output.stdout);
}

pub async fn create_tunnel() {
    info!("creating tunnel");
    let output = Command::new("i2p-zero-linux.v1.20/router/bin/tunnel-control.sh")
        .args(["server.create", "127.0.0.1", "8000"])
        .spawn()
        .expect("i2p-zero failed to create a tunnel");
    debug!("{:?}", output.stdout);
}
