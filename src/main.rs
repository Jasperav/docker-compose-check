use std::{time, thread};
use std::process::Command;
use scylla::{Session, SessionBuilder};

#[tokio::main]
async fn main() {
    // Sleep a little bit so Scylla can start up
    let ten_millis = time::Duration::from_secs(60 * 3);

    thread::sleep(ten_millis);

    // Now check the nodetool status
    let result = Command::new("docker")
        .args(&["exec", "somescylla", "nodetool", "status"])
        .status()
        .unwrap();

    println!("Result: {:#?}", result);

    let output = Command::new("docker")
        .args(&[
            "exec",
            "somescylla",
            "cqlsh",
            "-f",
            "/project/setup.cql",
        ])
        .output()
        .unwrap();

    println!("Query result: {:#?}", output);

    let result = Command::new("docker")
        .args(&["exec", "somescylla", "nodetool", "status", "ks"])
        .status()
        .unwrap();

    println!("Post result: {:#?}", result);
}
