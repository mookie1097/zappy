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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum PinConfig {
    Pair { positive: u32, negative: u32 },
    Pin(u32),
}


use gpio::{GpioOut};
use gpio::dummy::DummyGpioOut;


fn main() {

    //let mut dg = DummyGpioOut::new(|_| ());
    //let mut dg = GpioOut::
    //let _ = dg.set_value(true);
    //let mut gpio23 = gpio::sysfs::SysFsGpioInput::open(23).unwrap();    
    


    println!("Hello, buttplug!");

    let datamap = fileio::load();
    println!("{:?}", datamap);

    println!("Goodbye, buttplug!");
    piGpio::testing();
}

// fn setup() {
//     //pub datamap: DeviceDataMap;
// }
