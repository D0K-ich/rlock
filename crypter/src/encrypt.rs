use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, OsRng};
use rand::RngCore;
use rypes::errors::errors::RError;

pub fn encrypt_file_in_place(file_path: &Path, key: &[u8; 32]) -> Result<(), RError> {
	let mut file = OpenOptions::new().read(true).write(true).open(file_path).map_err(|e| RError::new(format!("encrypt_file_in_place {}: {}", file_path.display(), e)))?;

	let mut plaintext = Vec::new();
	file.read_to_end(&mut plaintext).map_err(|e| RError::new(format!("encrypt_file_in_place read_to_end {}: {}", file_path.display(), e)))?;

	if plaintext.is_empty() { return Ok(()); }

	let cipher = Aes256Gcm::new(key.into());

	let mut nonce_bytes = [0u8; 12];
	OsRng.fill_bytes(&mut nonce_bytes);
	let nonce = Nonce::from_slice(&nonce_bytes);

	let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).map_err(|e| RError::new(format!("encrypt_file_in_place {}: {}", file_path.display(), e)))?;

	file.seek(SeekFrom::Start(0)).map_err(|e| RError::new(format!("encrypt_file_in_place seek {}: {}", file_path.display(), e)))?;
	file.set_len(0).map_err(|e| RError::new(format!("encrypt_file_in_place set_len {}: {}", file_path.display(), e)))?;
	file.write_all(&nonce_bytes).map_err(|e| RError::new(format!("encrypt_file_in_place write_all 1 {}: {}", file_path.display(), e)))?;
	file.write_all(&ciphertext).map_err(|e| RError::new(format!("encrypt_file_in_place write_all 2 {}: {}", file_path.display(), e)))?;
	
	Ok(())
}