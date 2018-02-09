use ws::*;
use ws;


use common::*;

use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

pub struct InnerClient {
    pub tx:Sender<SocketEvent>,
    pub buffer:Option<Vec<SocketMessage>>,
    pub connected:bool,
    pub out:Option<ws::Sender>
}

impl InnerClient {
    
    pub fn flush(&mut self) {
        if self.connected(){
            let buffer = { self.buffer.take() };
            if let Some(buffer) = buffer {
                for message in buffer {
                    self.send(message);
                }
            }
        }
    }

    pub fn connected(&self) -> bool {
        self.connected
        //(self.ws.ready_state() as usize) == 1
    }

    pub fn send(&mut self,message:SocketMessage) {
        if self.connected(){
            match message {
                SocketMessage::Text(msg) => {
                    self.out.unwrap().send(Message::Text(msg));
                },
                _ => {}
            }
        }else{
            if self.buffer.is_none(){
                self.buffer = Some(Vec::new());
            }
            if let Some(ref mut buffer) = self.buffer {
                buffer.push(message);
            }
        }
    }
}

pub struct Client {
    inner:Rc<RefCell<InnerClient>>,
}

impl Client {
    pub fn new(url:&str)-> (Client,Receiver<SocketEvent>) {

        let (tx, rx): (Sender<SocketEvent>, Receiver<SocketEvent>) = mpsc::channel();

        let inner = Rc::new(RefCell::new(InnerClient{
            connected:false,
            tx,
            buffer:None,
            out:None,
        }));

        let c = inner.clone();

        connect(url,|out|{
            (*c.borrow_mut()).out = Some(out);
            WebSocketHandler{inner:c}
        });

        (Client{inner},rx)
    }

    pub fn send(&mut self,message:SocketMessage) {
        (*self.inner.borrow_mut()).send(message);
    }
}

pub struct WebSocketHandler {
    inner:Rc<RefCell<InnerClient>>,
}

impl Handler for WebSocketHandler {

    fn on_open(&mut self, shake: Handshake) -> Result<()> { 
        let inner = *self.inner.borrow_mut();
        inner.connected = true;
        inner.flush();
        inner.tx.send(SocketEvent::Open).unwrap();
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.inner.borrow_mut().tx.send(SocketEvent::Close).unwrap();
    }

    fn on_error(&mut self, err: Error) {
        let inner = *self.inner.borrow_mut();
        inner.tx.send(SocketEvent::Error).unwrap();
    }


    fn on_message(&mut self, msg: Message) -> Result<()> {
        let data = match msg {
            Message::Text(val) => SocketMessage::Text(val),
            Message::Binary(val) => SocketMessage::Binary(val), 
        };
        self.inner.borrow_mut().tx.send(SocketEvent::Message(data)).unwrap();
        Ok(())
    }

}