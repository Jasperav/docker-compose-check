use std::{time, thread};
use std::process::Command;

fn main() {
    // Sleep a little bit so Scylla can start up
    let ten_millis = time::Duration::from_secs(60 * 5);

    thread::sleep(ten_millis);

    // Now check the nodetool status
    let result = Command::new("docker")
        .args(&["exec", "somescylla", "nodetool", "status"])
        .status()
        .unwrap();

    println!("Result: {:#?}", result);
}
