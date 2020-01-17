use crate::config::PinConfig;

////// ---- DEVICE ---- //////
#[derive(Debug)]
pub struct Device {
	pub name: String,
	pub functions: Vec<Function>,
}

impl Device {
	fn ping(&self) {
		println!("Pinging Device {}", self.name);
	}
}




/////// ---- Function ---- ///////
#[derive(Debug)]
pub struct Function {
	
	pub name: String,
	pub channels: Vec<Channel>,
}

impl Function{
	fn trigger(&self, channel: u32){
		println!("hi");
	}

	fn ping(&self) {
		println!("Pinging Vibrator {}", self.name);
	}
	fn getChannels(&self) -> Vec<Channel> {
		return self.channels.clone();
	}
}

/////// ---- channel and pin---- ///////

//this stores the pins along with other info if provided
#[derive(Copy, Clone, Debug)]
pub struct Channel {
	pub pin: PinConfig,
	//pub min: u32,
	//pub max: u32,
}

impl Channel {
	fn ping(&self) {
		println!("Pinging channel {:#?}", self.pin);
	}
	pub fn print(&self) {
		println!("pin: {:#?}", self.pin);

		//println!("pin: {:#?} min: {} max: {}", self.pin, self.min, self.max);
	}
}




