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

    let (mut sender,rx) = WebSocketSender::new("ws://echo.websocket.org");

    

    app.run(move |_t:&mut App| {
        for ev in rx.try_iter(){
            let j = serde_json::to_string(&ev).unwrap();
            log(&j);
            match ev {
                SocketEvent::Open => {
                    sender.send(SocketMessage::Text("hello".into()));
                },
                _ =>{

                }
            }
        }
        //println!("{}",sources.len());
    });
    
}
