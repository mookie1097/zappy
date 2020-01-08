#![allow(non_snake_case)]
#![allow(unused_imports)]

//use buttplug::server::device_manager;
mod fileio;
mod piGpio;
use gpio;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

fn defaultMinValue() -> u32 {
    0
}
fn defaultMaxValue() -> u32 {
    100
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    version: String,
    devices: Vec<DeviceConfig>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DeviceConfig {
    name: String,
    functions: Vec<FunctionConfig>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct FunctionConfig {
    #[serde(rename = "type")] //cant name this var type for some reason
    name: String,
    #[serde(default = "defaultMinValue")]
    min: u32,
    #[serde(default = "defaultMaxValue")]
    max: u32,
    pins: Vec<PinConfig>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum PinConfig {
    Pair { positive: u32, negative: u32 },
    Pin(u32),
}

// use gpio::dummy::DummyGpioOut;
// use gpio::GpioOut;
mod device;
use device::{Device, Function, Tens, *};

fn main() {
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
