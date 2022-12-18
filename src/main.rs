use url::Url;
use tungstenite::{connect, Message};
use serde_json;

const BASE_URL: &str = "wss://stream.data.alpaca.markets/v1beta2/crypto";

fn main() {
    let (mut socket, _response) = connect(Url::parse(BASE_URL).unwrap()).expect("Can't connect");
    println!("{:#?}", _response);

    let authorization = r#"{"action": "auth", "key": "{YOUR_KEY_ID}", "secret": "{YOUR_SECRET}"}"#;
    let subscribe = r#"{"action":"subscribe","trades":["BTC/USD","ETH/USD"],"quotes":[],"bars":[]}"#;

    socket.write_message(Message::Text(authorization.into())).unwrap();
    socket.write_message(Message::Text(subscribe.into())).unwrap();

    loop {  
        let msg = socket.read_message().expect("Error reading message").to_string();
        if msg != "" {
            let msg_vector: Vec<serde_json::Value> = serde_json::from_str(&msg).unwrap(); 
            for m in msg_vector {
                if m["T"] == "t" {
                    println!("{:#?}", &m);
                }
            }   
        }   
    }
}
