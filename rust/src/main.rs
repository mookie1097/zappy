#![allow(non_snake_case)]
#![allow(unused_imports)]

//use buttplug::server::device_manager;
mod config;
mod device;
mod fileio;
mod piGpio;
mod websocket;

use device::{Device, Function, *};
use gpio;
use std::collections::HashMap;

fn main() {
    println!("startwebsocket");

    websocket::startwebsocket();
    // std::process::exit(0);
    println!("Hello, buttplug!");

    let datamap = fileio::load();
    // println!("{:?}", datamap);

    println!("Config read in");
    //piGpio::testing();
    //println!("gpio test complete");

    //meow
    let mut devices = Vec::<Device>::new();
    for device in datamap.devices {
        let mut funcVec = Vec::<Function>::new();
        for func in device.functions {
                //name instead of type because its reserved
                let mut channels = Vec::<Channel>::new();
                for pin in func.pins {
                    channels.push(Channel { pin: pin })
                }
                let dev = Function {
                    name: func.name.to_string(),
                    channels: channels,
                };
                funcVec.push(dev);
            
        }

        let dev = Device {
            name: device.name,
            functions: funcVec,
        };
        devices.push(dev);
    }
    // println!("{:#?}", devices);

    // for device in devices.iter() {
    //     //vec![&tens as &Function, &vibrator as &Function];

    //     for func in device.functions {
    //         func.ping();

    //         for channel in func.getChannels().iter() {
    //             channel.print();
    //         }
    //         println!("");
    //     }
    // }
}

// fn setup() {
//     //pub datamap: DeviceDataMap;
// }
