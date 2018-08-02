warning: LF will be replaced by CRLF in Cargo.toml.
The file will have its original line endings in your working directory.
warning: LF will be replaced by CRLF in example/src/main.rs.
The file will have its original line endings in your working directory.
warning: LF will be replaced by CRLF in src/lib.rs.
The file will have its original line endings in your working directory.
[1mdiff --git a/Cargo.toml b/Cargo.toml[m
[1mindex 50eaf79..03d3bd9 100644[m
[1m--- a/Cargo.toml[m
[1m+++ b/Cargo.toml[m
[36m@@ -13,5 +13,6 @@[m [mserde_json = "1.0"[m
 stdweb = { git = "https://github.com/koute/stdweb" }[m
 [m
 [m
[31m-[target.'cfg(not(target_arch = "wasm32"))'.dependencies][m
[31m-ws = "*"[m
\ No newline at end of file[m
[32m+[m[32m[target.'cfg(not(target_arch = "wasm32"))'.dependencies.ws][m
[32m+[m[32mversion = "*"[m
[32m+[m[32mfeatures = ["ssl"][m
\ No newline at end of file[m
[1mdiff --git a/example/src/main.rs b/example/src/main.rs[m
[1mindex 7961c47..8e080a5 100644[m
[1m--- a/example/src/main.rs[m
[1m+++ b/example/src/main.rs[m
[36m@@ -12,9 +12,7 @@[m [mfn main() {[m
     let config = AppConfig::new("Test", size);[m
     let app = App::new(config);[m
 [m
[31m-    let (mut sender,rx) = WebSocketSender::new("ws://echo.websocket.org");[m
[31m-[m
[31m-    [m
[32m+[m[32m    let (mut sender,rx) = WebSocketSender::new("ws://localhost:5012");[m
 [m
     app.run(move |_t:&mut App| {[m
         for ev in rx.try_iter(){[m
[1mdiff --git a/src/lib.rs b/src/lib.rs[m
[1mindex 9efeb5b..b8ebfa1 100644[m
[1m--- a/src/lib.rs[m
[1m+++ b/src/lib.rs[m
[36m@@ -1,3 +1,5 @@[m
[32m+[m[32m#![recursion_limit="128"][m
[32m+[m
 extern crate serde;[m
 extern crate serde_json;[m
 #[macro_use][m
[1mdiff --git a/src/stdw.rs b/src/stdw.rs[m
[1mindex 30f4c19..064c601 100644[m
[1m--- a/src/stdw.rs[m
[1m+++ b/src/stdw.rs[m
[36m@@ -36,36 +36,30 @@[m [mimpl WebSocketSender {[m
 [m
         let sender = tx.clone();[m
         ws.add_event_listener(move |event: SocketMessageEvent| {[m
[31m-            let data = if let Some(text) = event.data().into_text() {[m
[31m-                SocketMessage::Text(text)[m
[32m+[m[32m            if let Some(text) = event.data().into_text() {[m
[32m+[m[32m                let data = SocketMessage::Text(text);[m
[32m+[m[32m                sender.send(SocketEvent::Message(data)).unwrap();[m
             }else if let Some(blob) = event.data().into_blob(){[m
                 let s2 = sender.clone();[m
                 js!{[m
[31m-                    console.log("blob");[m
[31m-                    function loadBlob(callback){[m
[32m+[m[32m                    function loadBlob(blob,callback){[m
                         var reader = new FileReader();[m
                         reader.addEventListener("loadend", function() {[m
                             callback(reader.result);[m
[31m-                        // reader.result contains the contents of blob as a typed array[m
                         });[m
[31m-                        reader.readAsArrayBuffer(@{blob});[m
[32m+[m[32m                        reader.readAsArrayBuffer(blob);[m
                     }[m
[31m-[m
[31m-                    loadBlob(@{move |buffer:ArrayBuffer|{[m
[32m+[m[32m                    loadBlob(@{blob},@{move |buffer:ArrayBuffer|{[m
                         let typed_array: TypedArray< u8 > = buffer.into();[m
                         let data = SocketMessage::Binary(typed_array.to_vec());[m
                         s2.send(SocketEvent::Message(data)).unwrap();[m
                     }});[m
                 }[m
[31m-                return;[m
[31m-                //SocketMessage::Binary(blob.to_vec())[m
             }else if let Some(buffer) = event.data().into_array_buffer() {[m
                 let typed_array: TypedArray< u8 > = buffer.into();[m
[31m-                SocketMessage::Binary(typed_array.to_vec())[m
[31m-            }else{[m
[31m-                panic!("unknown type");[m
[31m-            };[m
[31m-            sender.send(SocketEvent::Message(data)).unwrap();[m
[32m+[m[32m                let data = SocketMessage::Binary(typed_array.to_vec());[m
[32m+[m[32m                sender.send(SocketEvent::Message(data)).unwrap();[m
[32m+[m[32m            }[m
         });[m
 [m
         (WebSocketSender { ws }, rx)[m
[36m@@ -75,8 +69,10 @@[m [mimpl WebSocketSender {[m
         match message {[m
             SocketMessage::Text(msg) => {[m
                 self.ws.send_text(&msg);[m
[32m+[m[32m            },[m
[32m+[m[32m            SocketMessage::Binary(bytes)=>{[m
[32m+[m[32m                self.ws.send_bytes(&bytes);[m
             }[m
[31m-            _ => {}[m
         }[m
     }[m
 }[m
