# NEWTWORK_PROTOTYPE_1

Hi, this is my first project called NEWTWORK.
And here we have --> prototype1.  
It is a simple one-way message sender and receiver using terminals.




<br>
# Project Directory Structure
<br>
newtwork_prototype/
<br>
├── Cargo.toml                # Rust project configuration and dependencies <br>
├── Cargo.lock                # Locked dependency versions <br>
├── README.md                 # Project documentation <br>
├── my_keys.json              # Generated public keys of current user <br>
├── peer_keys.json            # Public keys of peer (shared manually) <br>
├── src/                      # Source code directory <br>
│   ├── main.rs               # Entry point (CLI handling, mode selection)<br>
│   ├── lib.rs                # Module declarations <br>
│   ├── identity.rs           # Ephemeral identity & key generation <br>
│   ├── crypto.rs             # Encryption, decryption, signing, verification <br>
│   ├── packet.rs             # Packet structure definition<br>
│   ├── sender.rs             # Message encryption & sending logic <br>
│   ├── receiver.rs           # Message receiving & decryption logic <br>
│   └── logger.rs             # Timestamp-based logging <br>
├── target/                   # Build output directory (auto-generated) <br>
│   ├── debug/                # Debug build files <br>
│   ├── release/              # Release build binary<br>
│   └── CACHEDIR.TAG <br>


<br>






---

## How to use it

###STEPS OF USING NEWTWORK PROTOTYPE 1(ONE WAY TRANSMISSION) :

###STEP 1 : OPEN 2 TERMINALS <br>
###STEP 2 : MAKE SURE BOTH TERMINALS ARE IN THE SAME FILE DIRECTORY i.e. cd newtwork_prototype <br>
###STEP 3 : TERMINAL A --> RECEIVER <br>
###STEP 4 : TERMINAL B --> SENDER <br>

###STEP 5 : IN TERMINAL A RUN ---> "./target/release/newtwork_prototype --mode receive --my-port 9000 --peer-keys peer_keys.json"  AND KEEP IT RUNNING.

    //**this will generate a fresh ephemeral identity and save it to "my_keys.json"
    //**then would load peer's public keys from "peer_keys.json"
    //**& would listen on UDP port 9000 for incoming packets
    
    
###STEP 6 : IN TERMINAL B --> "cp my_keys.json peer_keys.json"  # share your public keys with receiver

###STEP 7 : IN TERMINAL B --> "./target/release/newtwork_prototype --mode send --message 'hello again!' --my-port 9001 --peer-ip 127.0.0.1 --peer-port 9000 --peer-keys peer_keys.json"
         
         // REMEMBER TO USE SINGLE QUOTES ('') TO PACK THE MESSAGE
         
###STEP 8 : --> THIS WOULD SEND YOUR MESSAGE TO YOUR OTHER TERMINAL AND THE SESSION WOULD END .
###STEP 9 : --> FOR SENDING OTHER MESSAGE REPEAT THE WHOLE PROCESS AGAIN.        

```bash










```
# Technical Details (NEWTWORK Prototype 1)

This prototype is a simple proof-of-concept to demonstrate secure, one-way
encrypted communication using ephemeral identities.

------------------------------------
##CORE TECHNOLOGIES USED
------------------------------------

- Language: Rust (edition 2021)
- Transport Layer: UDP
- Serialization: bincode
- Key Exchange: X25519 (ECDH)
- Encryption: ChaCha20-Poly1305 (AEAD)
- Authentication: Ed25519 digital signatures
- Hashing: SHA256
- CLI-based tool (no GUI)

------------------------------------
##EPHEMERAL IDENTITY MODEL
------------------------------------

- Every run generates fresh keys (no reuse):
  - X25519 keypair for encryption
  - Ed25519 keypair for signing
- Keys are valid for only ONE message/session.
- Provides Perfect Forward Secrecy (PFS).
- Public identity is represented using a SHA256 hash (identity tag).

------------------------------------
##KEY EXCHANGE METHOD
------------------------------------

- Hybrid manual key exchange (default):
  - Public keys are saved to `my_keys.json`
  - Keys are shared manually (copy file, QR, secure chat, etc.)
- Peer public keys are loaded from `peer_keys.json`
- No automatic discovery in this prototype.

------------------------------------
##PACKET STRUCTURE
------------------------------------

Each UDP packet contains:

- protocol version
- nonce (12 bytes)
- encrypted message (ciphertext)
- Ed25519 signature (nonce + ciphertext)
- sender X25519 public key
- sender Ed25519 public key

Packets are serialized using `bincode`.

------------------------------------
##PAYLOAD CAPACITY
------------------------------------

- UDP buffer size: 4096 bytes (4 KB)
- Practical encrypted message size: ~3500 bytes
- Remaining space is used by:
  - nonce
  - signature
  - public keys
  - packet metadata
- Large messages are NOT supported in this prototype.

------------------------------------
##ENCRYPTION & AUTHENTICATION FLOW
------------------------------------

Send Side:
- Derive shared key using X25519 + SHA256
- Encrypt message using ChaCha20-Poly1305
- Sign (nonce + ciphertext) using Ed25519
- Send packet over UDP

Receive Side:
- Deserialize packet
- Extract sender ephemeral public key
- Derive shared key
- Verify Ed25519 signature
- Decrypt ciphertext
- Display message

------------------------------------
##TIME & SESSION BEHAVIOR
------------------------------------

- One message per run
- After message is sent or received:
  - program exits
  - keys are destroyed
- To send another message, restart both terminals
- No message persistence or session storage

------------------------------------
##SECURITY PROPERTIES
------------------------------------

- End-to-end encrypted
- Authenticated sender
- Perfect Forward Secrecy
- No plaintext data on the network
- Minimal metadata exposure
- No static identifiers

------------------------------------
##LIMITATIONS (KNOWN)
------------------------------------

- One-way communication only
- No packet retransmission
- No message fragmentation
- Uses IP + port (only for prototype)
- No routing, relays, or multipath yet
- No protection against traffic analysis (future work)

------------------------------------
##INTENDED PURPOSE
------------------------------------

- Educational project
- Cryptography + networking learning
- Proof-of-concept for future NEWTWORK protocol
- NOT production-ready


```bash
