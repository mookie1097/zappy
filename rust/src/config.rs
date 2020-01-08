use serde::{Deserialize, Serialize};

fn defaultMinValue() -> u32 {
	0
}
fn defaultMaxValue() -> u32 {
	100
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
	pub version: String,
	pub devices: Vec<DeviceConfig>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DeviceConfig {
	pub name: String,
	pub functions: Vec<FunctionConfig>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FunctionConfig {
	#[serde(rename = "type")] //cant name this var type for some reason
	pub name: String,
	#[serde(default = "defaultMinValue")]
	pub min: u32,
	#[serde(default = "defaultMaxValue")]
	pub max: u32,
	pub pins: Vec<PinConfig>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum PinConfig {
	Pair { positive: u32, negative: u32 },
	Pin(u32),
}
