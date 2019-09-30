extern crate fistrs;

use fistrs::client::FistClient;

fn main() {
    let mut client = FistClient::new("localhost", "5575");
    client.connect();

    match client.version() {
        Ok(v) => println!("FIST version: {}", String::from_utf8(v).unwrap()),
        Err(e) => println!("Error encountered: {}", e)
    };
}
