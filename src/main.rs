use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("File not found.");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(format!("Could not read file {}", filename).as_str());

    let v: serde_json::Value = serde_json::from_str(contents.as_str()).unwrap();
    let ip_addr = v["IPv4Address"].clone();
    let username = v["User"].clone();

    let base_api_call = format!(
        "http://{}/api/{}",
        ip_addr.as_str().unwrap(),
        username.as_str().unwrap()
    );
    let hue_client = reqwest::Client::new();
    println!("Turning light on");
    hue_client
        .put(format!("{}/lights/1/state", base_api_call).as_str())
        .body(r#"{"on":true}"#)
        .send()
        .await
        .unwrap();

    thread::sleep(time::Duration::from_secs(5));

    println!("Turning light off");
    hue_client
        .put(format!("{}/lights/1/state", base_api_call).as_str())
        .body(r#"{"on":false}"#)
        .send()
        .await
        .unwrap();
}
