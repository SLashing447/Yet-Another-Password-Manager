use aead::{AeadCore, KeyInit};
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::OsRng};
use anyhow::{Ok, Result, anyhow};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha256;

/// Derive a 32-byte AES key from multiple password components, salt, and iteration count
fn derive_key(components: &[&str], salt: &[u8], iterations: u32) -> [u8; 32] {
    let combined_password = components.join("");
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(combined_password.as_bytes(), salt, iterations, &mut key);
    key
}

/// Create a new encrypted Record from data, name, password components and iterations
pub fn encrypt(
    data: &[u8],
    password_components: Vec<&str>,
    iterations: u32,
) -> Result<[Vec<u8>; 3]> {
    let salt = generate_salt(16);

    let key_bytes = derive_key(&password_components, &salt, iterations);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

    // let ciphertext = cipher.encrypt(nonce, data).expect("encryption failure!");
    let ciphertext = cipher
        .encrypt(&nonce, data.as_ref())
        .expect("encryption failure!");

    Ok([ciphertext, nonce.to_vec(), salt])
}

/// Decrypt a Record to original plaintext using the original password components and iterations
pub fn decrypt(
    password_components: Vec<&str>,
    iterations: u32,
    salt: Vec<u8>,
    nonce: Vec<u8>,
    data: &[u8],
) -> Result<Vec<u8>> {
    let key_bytes = derive_key(&password_components, &salt, iterations);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    // let key = Key::from_slice(&key_bytes);

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce);

    // self.data = cipher.decrypt(nonce, data).unwrap();

    let plain = cipher
        .decrypt(nonce, data)
        .map_err(|e| anyhow!("Decryption failed: {:?}", e))?; // manually convert?;
    Ok(plain)
}

fn generate_salt(len: usize) -> Vec<u8> {
    let mut salt = vec![0u8; len];
    rand::rng().fill_bytes(&mut salt);
    salt
}
