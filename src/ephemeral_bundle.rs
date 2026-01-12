use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use ed25519_dalek::{Keypair as Ed25519Keypair, Signer};
use sha2::{Digest, Sha256};

/// Ephemeral keypair bundle which is used for a single session/message in NEWTWORK.
#[derive(Clone)] //we can duplicate this struct easily when needed
pub struct EphemeralBundle {
    pub x25519_secret: EphemeralSecret,
    pub x25519_public: X25519PublicKey,
    pub ed25519_keypair: Ed25519Keypair,
    pub identity_tag: [u8; 32], 
}//keys ko separately handel karne ke bajaye i wrapped them in a bundle such that, 

impl EphemeralBundle {
    /// Generate karega fresh ephemeral identity.
    pub fn generate() -> Self {
        // Generates X25519 ephemeral keypair
        let x25519_secret = EphemeralSecret::random_from_rng(OsRng);
        let x25519_public = X25519PublicKey::from(&x25519_secret);

        // Generate Ed25519 signing keypair
        let ed25519_keypair = Ed25519Keypair::generate(&mut OsRng);

        // Derive public identity tag from X25519 public key
        let mut hasher = Sha256::new();
        hasher.update(x25519_public.as_bytes());
        let identity_tag = hasher.finalize().into();//short-lived tag that represents the ephemeral identity without revealing the actual public key

        Self {
            x25519_secret,
            x25519_public,
            ed25519_keypair,
            identity_tag,
        }
    }
}


// to pehle keypair generation then signing of it then derive the public identity from peerkeys
