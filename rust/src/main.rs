#![allow(non_snake_case)]
#![allow(unused_imports)]

//use buttplug::server::device_manager;
mod config;
mod device;
mod fileio;
mod piGpio;

use bluster::Peripheral;

use device::{Device, Function, Tens, *};
use gpio;
use std::collections::HashMap;

fn main() {
    // std::process::exit(0);
    println!("Hello, buttplug!");

    let datamap = fileio::load();
    println!("{:?}", datamap);

    println!("Config read in");
    piGpio::testing();
    println!("gpio test complete");

    //meow
    let mut devices = Vec::<Device>::new();
    for device in datamap.devices {
        let mut tensVec = Vec::<Tens>::new();
        let mut vibeVec = Vec::<Vibrator>::new();
        for func in device.functions {
            if func.name == "tens" {
                //name instead of type because its reserved
                let mut channels = Vec::<Channel>::new();
                for pin in func.pins {
                    channels.push(Channel { pin: pin })
                }
                let tens = Tens {
                    name: "Tens".to_string(),
                    channels: channels,
                };
                tensVec.push(tens);
            } else if func.name == "vibrator" {
                //name instead of type because its reserved
                let mut channels = Vec::<Channel>::new();
                for pin in func.pins {
                    channels.push(Channel { pin: pin })
                }
                let vibe = Vibrator {
                    name: "Tens".to_string(),
                    channels: channels,
                };
                vibeVec.push(vibe);
            }
        }

        let dev = Device {
            name: device.name,
            functions: Functions {
                tens: tensVec,
                vibrator: vibeVec,
            },
        };
        devices.push(dev);
    }
    println!("{:#?}", devices);

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
