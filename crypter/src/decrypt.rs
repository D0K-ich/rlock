use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;

pub fn decrypt_file_in_place(file_path: &Path, key: &[u8; 32]) -> Result<(), String> {
	let mut file = OpenOptions::new().read(true).write(true).open(file_path).map_err(|e| format!("Cannot open {}: {}", file_path.display(), e))?;

	let mut encrypted_data = Vec::new();
	file.read_to_end(&mut encrypted_data).map_err(|e| format!("Cannot read {}: {}", file_path.display(), e))?;

	if encrypted_data.len() < 12 { return Err(format!("{}: File too short", file_path.display())); }

	let nonce_bytes = &encrypted_data[..12];
	let ciphertext = &encrypted_data[12..];

	let cipher = Aes256Gcm::new(key.into());
	let nonce = Nonce::from_slice(nonce_bytes);

	let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| format!("Decryption failed (wrong key?): {}", e))?;

	file.seek(SeekFrom::Start(0)).map_err(|e| format!("Seek failed: {}", e))?;
	file.set_len(0).map_err(|e| format!("Truncate failed: {}", e))?;
	file.write_all(&plaintext).map_err(|e| format!("Write failed: {}", e))?;

	Ok(())
}