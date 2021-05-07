//! This showcase how to use the parsing macro to easily
//! extract data from a log file.
#[macro_use] extern crate afmt;

#[fmt("<" level ">" facility ": " msg)]
#[derive(Debug)]
struct Log {
    level: u32,
    facility: String,
    msg: String,
}

fn main() {
    let logs = vec![
        "<5>httpd: GET '/'",
        "<3>systemd: oh no !",
        "<7>sshd: connection inbound",
        "invalid log entry",
        "<bad number>name: wow",
        "<",
    ];

    for log in logs {
        match log.parse::<Log>() {
            Ok(log) => println!("{:?}", log),
            Err(e) => println!("{:?}", e),
        }
    }
}