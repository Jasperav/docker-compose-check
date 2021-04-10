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

    let uri = "127.0.0.1:9042";
    let session = SessionBuilder::new().known_node(uri).build().await.unwrap();
    let result = session.query("CREATE KEYSPACE ks WITH replication = {'class':'SimpleStrategy', 'replication_factor' : 2};", &[]).await.unwrap();

    println!("Query result: {:#?}", result);
}
