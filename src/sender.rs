

use crate::crypto::{encrypt_and_sign, derive_shared_key};
use crate::packet::NewtworkPacket;
use crate::identity::PeerKeys; 
use ed25519_dalek::SigningKey;
use std::net::UdpSocket;
use x25519_dalek::{EphemeralSecret, PublicKey};

/// Sends an encrypted and signed NEWTWORK packet to a peer over UDP.
pub fn run_sender(
    my_port: &str,
    peer_ip: &str,
    peer_port: &str,
    peer_keys: &PeerKeys, 
    my_x25519_secret: EphemeralSecret,
    my_x25519_public: &PublicKey,
    my_ed25519_secret: &SigningKey,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Bind to the provided local port.
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", my_port))?;

    //Extract peer's X25519 public key
    let peer_x25519 = PublicKey::from(peer_keys.x25519);

    // Derive a shared symmetric key using X25519 and SHA-256.
    let shared_key = derive_shared_key(my_x25519_secret, &peer_x25519);

    // Encrypt the message and sign (nonce || ciphertext)
    let (ciphertext, nonce, signature) =
        encrypt_and_sign(&shared_key, my_ed25519_secret, message.as_bytes());

    // Create the NEWTWORK packet.
    let packet = NewtworkPacket {
        version: 1,
        source_tag: vec![], 
        destination_tag: vec![], 
        nonce: nonce.to_vec(),
        ciphertext,
        signature,
        ephemeral_pubkey: my_x25519_public.as_bytes().to_vec(),

        
        ed25519_pubkey: my_ed25519_secret.verifying_key().to_bytes().to_vec(), //This includes sender's Ed25519 public key
        //This calls .verifying_key() on our SigningKey (my_ed25519_secret) to get the public keyy
    };

    // Serialize(packet ko bytes mein convert krna taki it can be transmitted) and send the packet.
    let encoded = bincode::serialize(&packet)?;
    socket.send_to(&encoded, format!("{}:{}", peer_ip, peer_port))?;

    println!("Message sent!");
    Ok(())
}
