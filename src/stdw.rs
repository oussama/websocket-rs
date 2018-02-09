use common::*;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

use stdweb::web::WebSocket;
use stdweb::web::IEventTarget;
use stdweb::web::event::*;


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
            sender.send(SocketEvent::Message(SocketMessage::Text(
                event.data().into_text().unwrap(),
            ))).unwrap();
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
