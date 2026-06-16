use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub key: String,
	pub paths: Vec<String>,
	pub mode: String,
	pub allowed_extensions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct KeyboardDevice {
	pub instance_id: String,
	pub description: String,
	pub dev_node: u32,
}