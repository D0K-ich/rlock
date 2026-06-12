use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct PathEntry {
	pub path: String,
	pub is_file: bool,
}

#[derive(Clone)]
pub struct LogEntry {
	pub timestamp: String,
	pub message: String,
	pub is_error: bool,
}

pub struct BuilderApp {
	pub key_hex: String,
	pub paths: Vec<PathEntry>,
	pub mode: String,
	pub status: String,
	pub new_path: String,
	pub logs: Vec<LogEntry>,
	pub encrypt_exe_name: String,
	pub decrypt_exe_name: String,
	pub encrypt_only_target: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub key: String,
	pub paths: Vec<String>,
	pub mode: String,
	pub encrypt_extensions_only: bool,
}
