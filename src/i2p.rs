use crate::logger::{log, LogLevel};

#[derive(Debug)]
pub enum I2pStatus {
    Accept,
    Reject,
}

impl I2pStatus {
    pub fn value(&self) -> String {
        match *self {
            I2pStatus::Accept => String::from("Accepting tunnels"),
            I2pStatus::Reject => String::from("Rejecting tunnels: Starting up"),
        }
    }
}

// START i2p connection verification
/// TODO: create a tunnel for the server at initial startup
/// if one does not exist. See https://github.com/i2p-zero/i2p-zero
pub async fn check_connection() -> () {
    let client: reqwest::Client = reqwest::Client::new();
    let host: &str = "http://localhost:7657/tunnels";
    let tick: std::sync::mpsc::Receiver<()> = schedule_recv::periodic_ms(10000);
    // TODO: better handling and notification of i2p tunnel status
    //  this check should be running in the background
    loop {
        tick.recv().unwrap();
        match client.get(host).send().await {
            Ok(response) => {
                // do some parsing here to check the status
                let res = response.text().await;
                match res {
                    Ok(res) => {
                        // split the html from the local i2p tunnels page
                        let split1 = res.split("<h4><span class=\"tunnelBuildStatus\">");
                        let mut v1: Vec<String> = split1.map(|s| String::from(s)).collect();
                        let s1 = v1.remove(1);
                        let v2 = s1.split("</span></h4>");
                        let mut split2: Vec<String> = v2.map(|s| String::from(s)).collect();
                        let status: String = split2.remove(0);
                        if status == I2pStatus::Accept.value() {
                            log(LogLevel::INFO, "I2P is currently accepting tunnels").await;
                            break;
                        } else if status == I2pStatus::Reject.value() {
                            log(LogLevel::INFO, "I2P is currently rejecting tunnels").await;
                        } else {
                            log(LogLevel::INFO, "I2P is offline").await;
                        }
                    }
                    _ => log(LogLevel::ERROR, "I2P status check failure").await,
                }
            }
            Err(_e) => {
                log(LogLevel::ERROR, "I2P status check failure").await;
            }
        }
    }
}
// END I2P connection verification
