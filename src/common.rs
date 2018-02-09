
#[derive(Debug,Serialize, Deserialize)]
pub enum SocketEvent {
    Open,
    Close,
    Error,
    Message(SocketMessage),
}

#[derive(Debug,Serialize, Deserialize)]
pub enum SocketMessage {
    Text(String),
    Binary(Vec<u8>),
}
