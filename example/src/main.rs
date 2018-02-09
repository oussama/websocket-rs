#![feature(nll)]

extern crate websocket;
extern crate application;
extern crate serde_json;

use application::*;
use websocket::*;

fn main() {
    let size = (800, 600);
    let config = AppConfig::new("Test", size);
    let app = App::new(config);

    let (mut client,rx) = Client::new("wss://echo.websocket.org");

    client.send(SocketMessage::Text("hello".into()));

    app.run(move |_t:&mut App| {
        for msg in rx.try_iter(){
            let j = serde_json::to_string(&msg).unwrap();
            log(&j);
        }
        //println!("{}",sources.len());
    });
    
}
