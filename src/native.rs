use ws::*;
use ws;


use common::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc,Mutex};

pub struct WebSocketSender {
    sender:Arc<Mutex<Option<ws::Sender>>>,
}


impl WebSocketSender {

    pub fn new(url:&str)-> (WebSocketSender,Receiver<SocketEvent>) {

        let (tx, rx): (Sender<SocketEvent>, Receiver<SocketEvent>) = mpsc::channel();

        let sender = Arc::new(Mutex::new(None));
        let s = sender.clone();
        let owned_url:String = url.into();
        thread::spawn(move || {
            connect(owned_url,move|out|{
                s.lock().unwrap().get_or_insert(out);
                println!("handler");
                WebSocketHandler{tx:tx.clone()}
            }).unwrap();
        });
        
        (WebSocketSender{sender},rx)
    }

    pub fn send(&mut self,message:SocketMessage) {
        let data = match message {
            SocketMessage::Text(val) => Message::Text(val),
            SocketMessage::Binary(val) => Message::Binary(val), 
        };
        if let Some(ref sender) = *self.sender.lock().unwrap() {
            sender.send(data).unwrap();
        }else{
            println!("Not ready to send yet");   
        };
    }
}

pub struct WebSocketHandler {
    tx:Sender<SocketEvent>,
}

impl Handler for WebSocketHandler {

    fn on_open(&mut self, shake: Handshake) -> Result<()> { 
        println!("open");
        self.tx.send(SocketEvent::Open).unwrap();
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.tx.send(SocketEvent::Close).unwrap();
    }

    fn on_error(&mut self, err: Error) {
        self.tx.send(SocketEvent::Error).unwrap();
    }


    fn on_message(&mut self, msg: Message) -> Result<()> {
        let data = match msg {
            Message::Text(val) => SocketMessage::Text(val),
            Message::Binary(val) => SocketMessage::Binary(val), 
        };
        self.tx.send(SocketEvent::Message(data)).unwrap();
        Ok(())
    }

}