use crate::common::*;
use serde::Serialize;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use wasm_bindgen::closure::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

use failure::Error;
use std::fmt;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct WebSocketSender {
    pub ws: WebSocket,
    onopen: Closure<Fn(JsValue)>,
    onclose: Closure<FnMut()>,
    onerror: Closure<FnMut(JsValue)>,
    onmessage: Closure<FnMut(MessageEvent)>,
}

impl fmt::Debug for WebSocketSender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WebSocketSender")
    }
}

impl WebSocketSender {
    pub fn new(url: &str) -> (WebSocketSender, Receiver<SocketEvent>) {
        let (tx, rx): (Sender<SocketEvent>, Receiver<SocketEvent>) = mpsc::channel();

        let ws = WebSocket::new(url).unwrap();

        let sender = tx.clone();
        let onopen = Closure::wrap(Box::new(move |_ev: JsValue| {
            sender.send(SocketEvent::Open).unwrap();
        }) as Box<Fn(JsValue)>);

        let sender = tx.clone();
        let onerror = Closure::wrap(Box::new(move |err: JsValue| {
            use serde_json::Value;
            let v: Value = err.into_serde().unwrap();
            log(&format!("wserr {:?}", v));
            sender.send(SocketEvent::Error).unwrap();
        }) as Box<FnMut(JsValue)>);

        let sender = tx.clone();
        let onclose = Closure::wrap(Box::new(move || {
            sender.send(SocketEvent::Close).unwrap();
        }) as Box<FnMut()>);

        let sender = tx.clone();
        let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string() {
                let data = SocketMessage::Text(text);
                sender.send(SocketEvent::Message(data)).unwrap();
            } else {
                log("MESSAGE NOT STRING");
            }
            /* if let Some(blob) = event.data().into_blob() {
                let s2 = sender.clone();
             js!{
                function loadBlob(blob,callback){
                    var reader = new FileReader();
                    reader.addEventListener("loadend", function() {
                        callback(reader.result);
                    });
                    reader.readAsArrayBuffer(blob);
                }
                loadBlob(@{blob},@{move |buffer:ArrayBuffer|{
                    let typed_array: TypedArray< u8 > = buffer.into();
                    let data = SocketMessage::Binary(typed_array.to_vec());
                    s2.send(SocketEvent::Message(data)).unwrap();
                }});
            }
            } else if let Some(buffer) = event.data().into_array_buffer() {
                let typed_array: TypedArray<u8> = buffer.into();
                let data = SocketMessage::Binary(typed_array.to_vec());
                sender.send(SocketEvent::Message(data)).unwrap();
            }*/
        }) as Box<FnMut(MessageEvent)>);
        let wss = WebSocketSender {
            onopen,
            onerror,
            onclose,
            onmessage,
            ws,
        };
        wss.ws
            .set_onmessage(Some(&wss.onmessage.as_ref().unchecked_ref()));
        wss.ws
            .set_onopen(Some(&wss.onopen.as_ref().unchecked_ref()));
        wss.ws
            .set_onerror(Some(&wss.onerror.as_ref().unchecked_ref()));
        wss.ws
            .set_onclose(Some(&wss.onclose.as_ref().unchecked_ref()));

        (wss, rx)
    }

    pub fn send(&mut self, message: SocketMessage) -> Result<(), Error> {
        match message {
            SocketMessage::Text(msg) => self
                .ws
                .send_with_str(&msg)
                .map_err(|err| failure::err_msg(format!("{:?}", err))),
            SocketMessage::Binary(mut bytes) => self
                .ws
                .send_with_u8_array(&mut bytes)
                .map_err(|err| failure::err_msg(format!("{:?}", err))),
        }
    }

    pub fn send_text<T: Serialize>(&mut self, msg: T) -> Result<(), Error> {
        let encoded = serde_json::to_string(&msg).expect("Failed to encode message");
        self.ws
            .send_with_str(&encoded)
            .map_err(|err| failure::err_msg(format!("{:?}", err)))
    }
}
