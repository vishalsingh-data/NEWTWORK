use x25519_dalek::{EphemeralSecret, PublicKey};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use sha2::{Sha256, Digest};

/// Identity struct hold karega ephemeral keypair (X25519 + Ed25519) and a derived identity tag.
pub struct Identity {
    pub x25519_secret: Option<EphemeralSecret>,
    pub x25519_public: PublicKey,
    pub ed25519_secret: SigningKey,
    pub ed25519_public: VerifyingKey,
    pub tag: Vec<u8>,
}

impl Identity {
    /// Generates a new ephemeral identity with both encryption and signing keys.
    pub fn new() -> Self {
        let x25519_secret = EphemeralSecret::random_from_rng(OsRng);//Generate random X25519 secret (ECDH private key).
        let x25519_public = PublicKey::from(&x25519_secret);//derive it's public key

        let ed25519_secret = SigningKey::generate(&mut OsRng);//generate random Ed25519 signing key 
        let ed25519_public = ed25519_secret.verifying_key();//derive its verifying pub key

        let mut hasher = Sha256::new();//hash both pub keys together
        hasher.update(x25519_public.as_bytes());
        hasher.update(ed25519_public.as_bytes());
        let tag = hasher.finalize().to_vec();

        Identity {
            x25519_secret: Some(x25519_secret),
            x25519_public,
            ed25519_secret,
            ed25519_public,
            tag,
        }
    }

    pub fn get_x25519_public(&self) -> PublicKey {
        self.x25519_public
    }

    pub fn get_ed25519_public(&self) -> VerifyingKey {
        self.ed25519_public
    }
}

//sharing keys with another peer
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PeerKeys {
    pub x25519: [u8; 32],
    pub ed25519: [u8; 32],
}
