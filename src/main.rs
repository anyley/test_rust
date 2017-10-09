extern crate ws;
extern crate env_logger;
extern crate serde;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

//use serde_json::;
//use ws::{listen, CloseCode, OpCode, Sender, Frame, Handler, Handshake, Message, Result, Error, ErrorKind};

#[derive(Debug, Deserialize, Serialize)]
struct Action {
    #[serde(rename = "type")]
    _type: String,
    meta: Option<serde_json::Value>,
    payload: serde_json::Value,
    error: Option<serde_json::Value>,
}

impl std::convert::From<Action> for ws::Message {
    fn from(action: Action) -> ws::Message {
        let v = json!(action);
        ws::Message::from(v.to_string())
    }
}

impl std::convert::From<ws::Message> for Action {
    fn from(msg: ws::Message) -> Action {
        serde_json::from_str(&msg.to_string()).unwrap()
    }
}

fn fib(n: i64) -> i64 {
  if n < 2 { return n; }
  return fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("Start...");
    println!("{}", fib(40));

    env_logger::init().unwrap();

    ws::listen("0.0.0.0:3012", |out| {
        move |msg: ws::Message| {
            println!("Server got message '{:#?}'. ", msg);
            let value = Action::from(msg);
            println!("json from str: {:#?}", value);

            out.send(value)
        }
    }).unwrap();
}

