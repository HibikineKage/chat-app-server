#[macro_use]
extern crate env_logger;
extern crate ws;

use ws::{listen, Builder, Sender, Settings};

fn main() {
    env_logger::init();
    if let Err(error) = Builder::new()
        .with_settings(Settings {
            max_connections: 10_000,
            encrypt_server: true,
            ..Settings::default()
        })
        .build(|out: Sender| {
            move |msg| {
                println!("server got message '{}'. ", msg);
                out.send(msg)
            }
        })
        .unwrap()
        .listen("localhost:12345")
    {
        println!("Failed to create WebSocket due to {:?}", error)
    }
}
