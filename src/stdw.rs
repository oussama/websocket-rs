use common::*;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

use stdweb::web::WebSocket;
use stdweb::web::IEventTarget;
use stdweb::web::event::*;
use stdweb::web::*;
use stdweb::web::TypedArray;

pub struct WebSocketSender {
    ws: WebSocket,
}

impl WebSocketSender {
    pub fn new(url: &str) -> (WebSocketSender, Receiver<SocketEvent>) {
        let (tx, rx): (Sender<SocketEvent>, Receiver<SocketEvent>) = mpsc::channel();

        let ws = WebSocket::new(url).unwrap();

        let sender = tx.clone();
        ws.add_event_listener(move |_: SocketOpenEvent| {
            sender.send(SocketEvent::Open).unwrap();
        });

        let sender = tx.clone();
        ws.add_event_listener(move |_: SocketErrorEvent| {
            sender.send(SocketEvent::Error).unwrap();
        });

        let sender = tx.clone();
        ws.add_event_listener(move |event: SocketCloseEvent| {
            sender.send(SocketEvent::Close).unwrap();
        });

        let sender = tx.clone();
        ws.add_event_listener(move |event: SocketMessageEvent| {
            let data = if let Some(text) = event.data().into_text() {
                SocketMessage::Text(text)
            }else if let Some(blob) = event.data().into_blob(){
                let s2 = sender.clone();
                js!{
                    console.log("blob");
                    function loadBlob(callback){
                        var reader = new FileReader();
                        reader.addEventListener("loadend", function() {
                            callback(reader.result);
                        // reader.result contains the contents of blob as a typed array
                        });
                        reader.readAsArrayBuffer(@{blob});
                    }

                    loadBlob(@{move |buffer:ArrayBuffer|{
                        let typed_array: TypedArray< u8 > = buffer.into();
                        let data = SocketMessage::Binary(typed_array.to_vec());
                        s2.send(SocketEvent::Message(data)).unwrap();
                    }});
                }
                return;
                //SocketMessage::Binary(blob.to_vec())
            }else if let Some(buffer) = event.data().into_array_buffer() {
                let typed_array: TypedArray< u8 > = buffer.into();
                SocketMessage::Binary(typed_array.to_vec())
            }else{
                panic!("unknown type");
            };
            sender.send(SocketEvent::Message(data)).unwrap();
        });

        (WebSocketSender { ws }, rx)
    }

    pub fn send(&mut self, message: SocketMessage) {
        match message {
            SocketMessage::Text(msg) => {
                self.ws.send_text(&msg);
            }
            _ => {}
        }
    }
}
