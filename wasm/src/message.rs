use rustgym_msg::*;
use seed::prelude::*;
use web_sys::MediaStream;

pub enum Message {
    UrlChanged(subs::UrlChanged),
    SearchTextChanged(String),
    QueryText(String),
    KeyDown(web_sys::KeyboardEvent),
    WebSocketMsgOut(MsgOut),
    WebSocketMsgBin(MsgBin),
    WebSocketClosed(CloseEvent),
    WebSocketError(WebSocketError),
    MediaStreamReady(MediaStream),
    AllClients(Vec<ClientInfo>),
    WebSocketOpened,
    WebSocketFailed,
}