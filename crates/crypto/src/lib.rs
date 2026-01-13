//! Ada Remote Cryptography
//!
//! End-to-end encryption using Signal Protocol-inspired approach:
//! - X25519 for key exchange
//! - ChaCha20-Poly1305 for authenticated encryption
//! - Argon2 for password hashing

use ada_remote_core::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

/// Size of encryption keys in bytes
pub const KEY_SIZE: usize = 32;

/// Size of nonce in bytes
pub const NONCE_SIZE: usize = 12;

/// Encrypted message with nonce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Ciphertext
    pub ciphertext: Vec<u8>,
    /// Nonce used for encryption
    pub nonce: [u8; NONCE_SIZE],
}

/// Key pair for X25519 key exchange
pub struct KeyPair {
    secret: EphemeralSecret,
    public: PublicKey,
}

impl KeyPair {
    /// Generate a new random key pair
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let secret = EphemeralSecret::random_from_rng(&mut rng);
        let public = PublicKey::from(&secret);
        Self { secret, public }
    }

    /// Get the public key
    pub fn public_key(&self) -> &PublicKey {
        &self.public
    }

    /// Compute shared secret with peer's public key
    pub fn compute_shared_secret(self, peer_public: &PublicKey) -> SharedSecret {
        self.secret.diffie_hellman(peer_public)
    }
}

/// Encryption context for a session
pub struct EncryptionContext {
    cipher: ChaCha20Poly1305,
}

impl EncryptionContext {
    /// Create a new encryption context from a shared secret
    pub fn from_shared_secret(shared_secret: &SharedSecret) -> Result<Self> {
        // Use the shared secret directly as the key
        let key = chacha20poly1305::Key::from_slice(shared_secret.as_bytes());
        let cipher = ChaCha20Poly1305::new(key);
        Ok(Self { cipher })
    }

    /// Encrypt a message with associated data
    pub fn encrypt(&self, plaintext: &[u8], associated_data: &[u8]) -> Result<EncryptedMessage> {
        // Generate random nonce
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt with associated data
        let payload = Payload {
            msg: plaintext,
            aad: associated_data,
        };

        let ciphertext = self
            .cipher
            .encrypt(nonce, payload)
            .map_err(|e| ada_remote_core::Error::Session(format!("Encryption failed: {}", e)))?;

        Ok(EncryptedMessage {
            ciphertext,
            nonce: nonce_bytes,
        })
    }

    /// Decrypt a message with associated data
    pub fn decrypt(
        &self,
        encrypted: &EncryptedMessage,
        associated_data: &[u8],
    ) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(&encrypted.nonce);

        let payload = Payload {
            msg: &encrypted.ciphertext,
            aad: associated_data,
        };

        let plaintext = self
            .cipher
            .decrypt(nonce, payload)
            .map_err(|e| ada_remote_core::Error::Session(format!("Decryption failed: {}", e)))?;

        Ok(plaintext)
    }
}

/// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ada_remote_core::Error::Authentication(format!("Password hashing failed: {}", e)))?;

    Ok(hash.to_string())
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };

    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| ada_remote_core::Error::Authentication(format!("Invalid hash: {}", e)))?;

    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Generate a random session password (9-digit numeric)
pub fn generate_session_password() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:09}", rng.gen_range(0..1_000_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_exchange() {
        let alice = KeyPair::generate();
        let bob = KeyPair::generate();

        let alice_public = alice.public_key().clone();
        let bob_public = bob.public_key().clone();

        let alice_shared = alice.compute_shared_secret(&bob_public);
        let bob_shared = bob.compute_shared_secret(&alice_public);

        assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());
    }

    #[test]
    fn test_encryption_decryption() {
        let mut rng = rand::thread_rng();
        let secret = EphemeralSecret::random_from_rng(&mut rng);
        let shared_secret = secret.diffie_hellman(&PublicKey::from(&secret));

        let ctx = EncryptionContext::from_shared_secret(&shared_secret).unwrap();

        let plaintext = b"Hello, Ada Remote!";
        let aad = b"session-123";

        let encrypted = ctx.encrypt(plaintext, aad).unwrap();
        let decrypted = ctx.decrypt(&encrypted, aad).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_password_hashing() {
        let password = "secure-password-123";
        let hash = hash_password(password).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong-password", &hash).unwrap());
    }

    #[test]
    fn test_session_password_generation() {
        let password = generate_session_password();
        assert_eq!(password.len(), 9);
        assert!(password.chars().all(|c| c.is_ascii_digit()));
    }
}
