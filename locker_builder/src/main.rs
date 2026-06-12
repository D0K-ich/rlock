mod models;
mod ui;

use eframe::egui;
use crate::models::BuilderApp;

fn main() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_inner_size([1200.0, 700.0])
			.with_min_inner_size([900.0, 600.0])
			.with_title("Encryptor/Decryptor Builder v3.0"),
		..Default::default()
	};

	eframe::run_native(
		"Encryptor Builder",
		options,
		Box::new(|_cc| Box::new(BuilderApp::new())),
	)
}