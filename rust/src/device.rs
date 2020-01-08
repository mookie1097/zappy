use crate::PinConfig;

////// ---- DEVICE ---- //////
#[derive(Debug)]
pub struct Device {
	pub name: String,
	pub functions: Functions,
}

impl Device {
	fn ping(&self) {
		println!("Pinging Device {}", self.name);
	}
}

/////// ---- FUNCTION ---- ///////
#[derive(Debug)]
pub struct Functions {
	pub tens: Vec<Tens>,
	pub vibrator: Vec<Vibrator>,
}

pub trait Function {
	// this is stuff that MUST be implimented
	fn ping(&self);
	fn getChannels(&self) -> Vec<Channel>;
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

// #[derive(Copy, Clone, Debug)]
// pub enum Pin {
// 	//from pinconfig in main
// 	Pair { positive: u32, negative: u32 },
// 	Pin(u32),
// }

/////// ---- TENS ---- ///////
#[derive(Debug)]
pub struct Tens {
	pub name: String,
	pub channels: Vec<Channel>,
}

impl Function for Tens {
	fn ping(&self) {
		println!("Pinging Tens {}", self.name);
	}
	fn getChannels(&self) -> Vec<Channel> {
		return self.channels.clone();
	}
}
/////// ---- Vibe ---- ///////
#[derive(Debug)]
pub struct Vibrator {
	pub name: String,
	pub channels: Vec<Channel>,
}

impl Function for Vibrator {
	fn ping(&self) {
		println!("Pinging Vibrator {}", self.name);
	}
	fn getChannels(&self) -> Vec<Channel> {
		return self.channels.clone();
	}
}
