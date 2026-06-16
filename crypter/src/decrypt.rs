use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use rypes::errors::errors::RError;

pub fn decrypt_file_in_place(file_path: &Path, key: &[u8; 32]) -> Result<(), RError> {
	let mut file = OpenOptions::new().read(true).write(true).open(file_path).map_err(|e| RError::new(format!("decrypt_file_in_place {}: {}", file_path.display(), e)))?;

	let mut encrypted_data = Vec::new();
	file.read_to_end(&mut encrypted_data).map_err(|e| RError::new(format!("decrypt_file_in_place {}: {}", file_path.display(), e)))?;

	if encrypted_data.len() < 12 { RError::new(format!("encrypted_data.len() < 12 {}", file_path.display())); }

	let nonce_bytes = &encrypted_data[..12];
	let ciphertext = &encrypted_data[12..];

	let cipher = Aes256Gcm::new(key.into());
	let nonce = Nonce::from_slice(nonce_bytes);

	let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| RError::new(format!("decrypt_file_in_place {}: {}", file_path.display(), e)))?;

	file.seek(SeekFrom::Start(0)).map_err(|e| RError::new(format!("decrypt_file_in_place {}: {}", file_path.display(), e)))?;
	file.set_len(0).map_err(|e| RError::new(format!("decrypt_file_in_place {}: {}", file_path.display(), e)))?;
	file.write_all(&plaintext).map_err(|e| RError::new(format!("decrypt_file_in_place {}: {}", file_path.display(), e)))?;

	Ok(())
}