//use env_logger;
/// A thread-based client + server example. It also demonstrates using a struct as a WebSocket
/// handler to implement more handler methods than a closure handler allows.
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use ws::{connect, listen, CloseCode, Handler, Message, Result, Sender};

use buttplug::

use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn startwebsocket() {
    // Setup logging
    //env_logger::init();

    // Server WebSocket handler
    struct Server {
        out: Sender,
    }

    impl Handler for Server {
        fn on_message(&mut self, msg: Message) -> Result<()> {
            println!("Server got message '{}'. ", msg);
            //let msg = ;
            // = serde_json::from_str(msg.to_string().to_str());
            //let stri: str = msg.to_string().as_str();
            let jsonstuff = match serde_yaml::from_str(msg.as_text()) {
                Ok(obj) => {
                    println!("Config imported successfully");
                    //println!("{:?}", obj);
                    return obj;
                }
                Err(err) => {
                    println!("Config NOT imported successfully!");
                    println!("{}", err);
                }
            };
            //json!({"number":1,"state":1,"name":"prog1"}); //msg.to_string());
            println!("{}", serde_json::to_string_pretty(&jsonstuff).unwrap());
            println!("{} {}", jsonstuff["number"], jsonstuff["name"]);
            self.out.send(msg)
        }

        fn on_close(&mut self, code: CloseCode, reason: &str) {
            println!("WebSocket closing for ({:?}) {}", code, reason);
            println!("Shutting down server after first connection closes.");
            self.out.shutdown().unwrap();
        }
    }

    // Server thread
    let server = thread::spawn(move || listen("0.0.0.0:8765", |out| Server { out }).unwrap());

    // Give the server a little time to get going
    sleep(Duration::from_millis(10));

    let _ = server.join();

    println!("All done.")
}
