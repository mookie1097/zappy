//use buttplug::server::device_manager;
mod fileio;

use std::collections::HashMap;


pub struct DeviceDataMap {
    pub map: HashMap<GuildId, ServerData>,
}

fn setup(client: &mut Client) {
    {
        data.insert::<ServerDataMap>(fileio::load());
    }
}

fn main() {
    println!("Hello, buttplug!");
    setup();
    println!("Goodbye, buttplug!");

}
