use common::*;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

use stdweb::web::WebSocket;
use stdweb::web::IEventTarget;
use stdweb::web::event::*;
use stdweb::*;

use std::rc::Rc;
use std::cell::RefCell;


pub struct InnerClient {
    tx:Sender<SocketEvent>,
    ws:WebSocket,
    buffer:Option<Vec<SocketMessage>>,
    connected:bool,
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
                    self.ws.send_text(&msg);
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
    inner:Rc<RefCell<InnerClient>>
}

impl Client {


    pub fn new(url: &str) -> (Client,Receiver<SocketEvent>) {


        let (tx, rx): (Sender<SocketEvent>, Receiver<SocketEvent>) = mpsc::channel();



        let ws = WebSocket::new(url).unwrap();

        let inner = Rc::new(RefCell::new(
            InnerClient{
                tx,
                ws,
                buffer:None,
                connected:false,
            }
        ));

        let sender = inner.borrow_mut().tx.clone();
        let c = inner.clone();
        

        inner.borrow_mut().ws.add_event_listener(move |_: SocketOpenEvent| {
            c.borrow_mut().connected = true;
            c.borrow_mut().flush();
            sender.send(SocketEvent::Open).unwrap();
        });

        let sender = inner.borrow_mut().tx.clone();
        inner.borrow_mut().ws.add_event_listener(move |_: SocketErrorEvent| {
            sender.send(SocketEvent::Error).unwrap();
        });

        let sender = inner.borrow_mut().tx.clone();
        inner.borrow_mut().ws.add_event_listener(move |event: SocketCloseEvent| {
            sender.send(SocketEvent::Close).unwrap();
        });

        let sender = inner.borrow_mut().tx.clone();
        inner.borrow_mut().ws.add_event_listener(move |event: SocketMessageEvent| {
            sender.send(SocketEvent::Message(SocketMessage::Text(event.data().into_text().unwrap())));
        });

        (Client{inner},rx)

    }



    pub fn send(&mut self,message:SocketMessage) {
        self.inner.borrow_mut().send(message);
    }


}
