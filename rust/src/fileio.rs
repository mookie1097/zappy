use crate::config::{Config, DeviceConfig};
use serde_yaml;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const DEVICES_FILENAME: &str = "devices.yaml";

pub fn load() -> Config {
    // Create a path to the desired file
    let path = Path::new(DEVICES_FILENAME);
    let display = path.display();
    //Config;
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
    match serde_yaml::from_str(&s) {
        Ok(obj) => {
            println!("Config imported successfully");
            //println!("{:?}", obj);
            return obj;
        }
        Err(err) => {
            println!("Config NOT imported successfully!");
            println!("{}", err);
            return Config {
                version: "meow".to_string(),
                devices: Vec::<DeviceConfig>::new(),
            };
        }
    } //this is a return since theres no semicolon

    // `file` goes out of scope, and the file gets closed
}

pub fn store(map: &Config) {
    let serialized = match serde_yaml::to_string(&map) {
        Err(why) => panic!("MEOW MEOW{}", why.description()),
        Ok(nya) => nya,
    };
    //println!("{}", serialized);
    let path = Path::new(DEVICES_FILENAME);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
