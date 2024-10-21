use aes_gcm::aead::{generic_array::GenericArray, Aead, KeyInit, OsRng};
use aes_gcm::Aes256Gcm;
use aes_gcm::Key;
use rand::RngCore;
use std::fs;

pub fn encrypt_file(
    file_path: &str,
    key: &[u8; 32],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let plaintext = fs::read(file_path)?;

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = cipher
        .encrypt(GenericArray::from_slice(&nonce), plaintext.as_ref())
        .expect("mhm mhm");

    let mut output = vec![];
    output.extend_from_slice(&nonce);
    output.extend_from_slice(&ciphertext);

    fs::write(output_path, output)?;

    Ok(())
}

pub fn decrypt_file(
    file_path: &str,
    key: &[u8; 32],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read(file_path)?;

    let (nonce, ciphertext) = data.split_at(12);

    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher
        .decrypt(GenericArray::from_slice(nonce), ciphertext)
        .expect("mhm");

    fs::write(output_path, plaintext)?;

    Ok(())
}

pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}
