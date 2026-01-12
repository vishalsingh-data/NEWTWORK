

use crate::crypto::{decrypt_and_verify, derive_shared_key};
use crate::packet::NewtworkPacket;
use ed25519_dalek::{SigningKey, VerifyingKey};
use std::net::UdpSocket;
use x25519_dalek::{EphemeralSecret, PublicKey};

/// Listens for NEWTWORK packets and securely decrypts & verifies them.
pub fn run_receiver(
    my_port: &str,
    peer_x25519: &PublicKey,
    peer_ed25519: &VerifyingKey,
    my_x25519_secret: EphemeralSecret,
    _my_ed25519_secret: &SigningKey, // ye reserved hai future use ke liye (e.g. bidirectional, etc...)
) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", my_port))?;
    println!("Listening on port {}", my_port);

    let mut buf = [0u8; 4096];//creates a 4kb buffer, waits for a packet to arrive
    let (len, _src) = socket.recv_from(&mut buf)?;

    let packet: NewtworkPacket = bincode::deserialize(&buf[..len])?;

    // Extract sender's ephemeral public key from packet
    let ephemeral_bytes: [u8; 32] = packet
        .ephemeral_pubkey
        .as_slice()
        .try_into()
        .expect("Invalid ephemeral key length");
    let sender_ephemeral = PublicKey::from(ephemeral_bytes);

    // Derive symmetric key from sender's ephemeral key and receiver's private key
    let shared_key = derive_shared_key(my_x25519_secret, &sender_ephemeral);

   
    // Extract and reconstruct Ed25519 verifying key from packet
    let ed_bytes: [u8; 32] = packet.ed25519_pubkey
        .as_slice()
        .try_into()
        .expect("Ed25519 pubkey must be 32 bytes");
    let verifying_key = VerifyingKey::from_bytes(&ed_bytes)
        .expect("Invalid Ed25519 pubkey from packet");


     // Verify the Ed25519 signature and decrypt the payload
    match decrypt_and_verify(
        &shared_key,
    	packet
            .nonce
            .as_slice()
            .try_into()
            .expect("Nonce length mismatch"),
    	&packet.ciphertext,
    	&packet.signature,
    	&verifying_key,
     )
 {
        Ok(plaintext) => {
            println!("Received message: {}", String::from_utf8_lossy(&plaintext));
        }
        Err(e) => {
            eprintln!("❌ Failed to decrypt or verify packet: {}", e);
        }
    }

    Ok(())
}
