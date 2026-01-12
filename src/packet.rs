use serde::{Serialize, Deserialize};


/// This structure is serialized using `bincode` and sent over the transport.
#[derive(Serialize, Deserialize, Debug)]
pub struct NewtworkPacket {
    
    pub version: u8,

    /// Sender’s ephemeral identity tag (usually SHA256 of keys)
    pub source_tag: Vec<u8>, //currently not used because we are using traditional broadcast routing

    /// Receiver’s expected ephemeral identity tag
    pub destination_tag: Vec<u8>, //currently not used because we are using traditional broadcast routing

    /// Nonce for ChaCha20Poly1305 (12 bytes)
    pub nonce: Vec<u8>,

    /// AEAD-encrypted message payload
    pub ciphertext: Vec<u8>,

    /// Ed25519 signature over (nonce || ciphertext)
    pub signature: Vec<u8>,

    /// Ephemeral X25519 public key of the sender
    pub ephemeral_pubkey: Vec<u8>,

    pub ed25519_pubkey: Vec<u8>, 

}
