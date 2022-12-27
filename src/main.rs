



use std::process::Command;
use std::{thread, time};

fn get_ip() -> String {
    let ip = Command::new("curl")
        .arg("ifconfig.me")
        .output()
        .expect("Unknown");
    String::from_utf8(ip.stdout).expect("Error").replace("\"", "")
}

fn get_local_ip() -> String {
    let ip = Command::new("ip")
        .arg("address")
        .arg("show")
        .output()
        .expect("Error running command");
    let string = String::from_utf8(ip.stdout).expect("Error converting");
    string
}

fn main() {
    
    let mut count = 0;
    let mut public_ip = get_ip();
    let mut local_ip = get_local_ip();
    println!("{}", local_ip);

    loop {
        println!("{:?}", public_ip);
        thread::sleep(time::Duration::from_secs(1));
        if count > 600 {
            public_ip = get_ip();
            count = 0;
        }
    }
}
