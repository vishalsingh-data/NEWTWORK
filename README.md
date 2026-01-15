# NEWTWORK_PROTOTYPE_1


Hi,
This is my first serious systems project called "**NEWTWORK**".
**A beginner-friendly experiment in secure, identity-free communication**

This repository contains "**Prototype 1**", which is a very small but focused
proof-of-concept for **secure, one-way message transmission** using modern
cryptography and ephemeral identities.

This project is mainly for **learning and experimentation**, not production use.




---

## Why does this project exist?

Most modern network communication relies on **static identifiers** such as:

- IP addresses  
- Ports  
- Long-lived cryptographic keys  
- Reusable identities  

These identifiers make systems easier to build but also easier to **track, analyze, and attack**.

This prototype explores a different idea:

> **What if every communication used fresh identities,  
> revealed no meaningful metadata,  
> and remained secure even if packets were intercepted?**

**NEWTWORK Prototype 1** is a small but working proof-of-concept that demonstrates:

- End-to-end encryption  
- Ephemeral identities  
- Minimal metadata exposure  
- Simple and understandable cryptographic flow  

---



---

## A short story: what if someone intercepts the packet?

To understand what makes this prototype interesting, let’s walk through a simple story.

### Characters

- **A** -> the sender  
- **B** -> the receiver  
- **C** -> a Man-in-the-Middle attacker watching the network  

A sends a message to B using NEWTWORK Prototype 1 over UDP.
**NOTE FOR THIS PARTICULAR PROTOTYPE ONLY, WE ARE USING UDP AND IT'S NOT THE END GOAL AND I AM WORKING ON HOW TO DO IT WITH TCP TOO.**

C manages to **capture the UDP packet** using a network analysis tool.

Now let’s see what C can do and what it can't.

---

## Step 1: C sees the packet

C observes a UDP packet on the network.

C can see:
- random-looking bytes
- no readable message
- no usernames
- no session IDs
- no permanent identifiers

There is **no plaintext data** inside the packet.

---

## Step 2: C tries to read the message

Inside the packet, C finds:

- encrypted ciphertext  
- a random nonce  
- public keys  
- a digital signature  

However:

- the message is encrypted using **ChaCha20-Poly1305**
- the encryption key is derived using **X25519 key exchange**
- private keys **never leave A’s or B’s machines**

Without the private keys, C **cannot decrypt the message**.

---

## Step 3: C tries to modify the packet

C attempts to alter the packet before forwarding it.

This fails because:

- every packet is **digitally signed** using **Ed25519**
- B verifies the signature before decrypting

If even **one bit** of the packet is changed:
- signature verification fails
- the packet is rejected immediately

---

## Step 4: C records the packet for later

C saves the packet, hoping to decrypt it in the future.

This also fails because:

- all keys are **ephemeral**
- fresh keys are generated for every run
- keys are destroyed when the program exits

This provides **Perfect Forward Secrecy**.

Even if keys are compromised later, **past messages remain secure**.

---

## Final result for the attacker

Even after intercepting the packet, C gains:

-  NO message content  
-  NO reusable identity  
-  NO meaningful metadata  
-  NO ability to impersonate sender or receiver  

The packet is **cryptographically protected and practically useless** to an attacker.

---





---

## What This Prototype Does

1. Sends **one encrypted message** from one terminal to another
2. Uses **strong cryptography by default**
3. Does **not send any plaintext data**
4. Uses **fresh keys every time** (no permanent identity)
5. Works completely from the **command line**

This prototype is intentionally simple so that the **entire flow is easy to understand**.

---

## What this prototype does NOT do **(YET)**

- No two-way communication  
- No packet retransmission  
- No fragmentation  
- Uses IP + port (only for prototype purposes)  
- No routing, relays, or anonymity networks  
- No protection against traffic analysis  

---



## Why Someone Might Want to Try This

You may find this project useful if you are:

1. Learning **cryptography** (X25519, Ed25519, AEAD)
2. Learning **Rust** for systems or security work
3. Curious about **how secure messaging works internally**
4. Interested in **ephemeral identities and forward secrecy**
5. Want to understand **ephemeral identity models** 
6. Exploring alternatives to traditional identity-based networking
7. Experimenting with secure communication ideas
8. Serving as the foundation for future NEWTWORK designs  


This project shows how different crypto pieces fit together in a **real running program**.


---

---

## At Last,

This prototype is intentionally simple.

The goal is not to compete with existing protocols,  
but to **learn**, **experiment**, **question** and **resolve**
**the drawbacks and loopholes of traditional networking assumptions and protocols**.

Every future version of NEWTWORK builds on ideas explored here.

---
---
---
---
---
---


# NOW HERE ARE ALL THE TECHNICAL INFO REGARDING THIS PROTOTYPE.
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
         
###STEP 8 : --> THIS WOULD SEND YOUR MESSAGE TO YOUR OTHER TERMINAL AND THE SESSION WOULD END .<br>
###STEP 9 : --> FOR SENDING OTHER MESSAGE REPEAT THE WHOLE PROCESS AGAIN.        

```bash










```
# Technical Details (NEWTWORK Prototype 1)

This prototype is a simple proof-of-concept to demonstrate secure, one-way
encrypted communication using ephemeral identities.

--------------------------------
##CORE TECHNOLOGIES USED
--------------------------------

- Language: Rust (edition 2021)
- Transport Layer: UDP
- Serialization: bincode
- Key Exchange: X25519 (ECDH)
- Encryption: ChaCha20-Poly1305 (AEAD)
- Authentication: Ed25519 digital signatures
- Hashing: SHA256
- CLI-based tool (no GUI)

----------------------------------
##EPHEMERAL IDENTITY MODEL
----------------------------------

- Every run generates fresh keys (no reuse):
  - X25519 keypair for encryption
  - Ed25519 keypair for signing
- Keys are valid for only ONE message/session.
- Provides Perfect Forward Secrecy (PFS).
- Public identity is represented using a SHA256 hash (identity tag).

------------------------------------
##KEY EXCHANGE METHOD
---------------------------------------

- Hybrid manual key exchange (default):
  - Public keys are saved to `my_keys.json`
  - Keys are shared manually (copy file, QR, secure chat, etc.)
- Peer public keys are loaded from `peer_keys.json`
- No automatic discovery in this prototype.

--------------------------------
##PACKET STRUCTURE
---------------------------------

Each UDP packet contains:

- protocol version
- nonce (12 bytes)
- encrypted message (ciphertext)
- Ed25519 signature (nonce + ciphertext)
- sender X25519 public key
- sender Ed25519 public key

Packets are serialized using `bincode`.

------------------------------------------
##PAYLOAD CAPACITY
-----------------------------------------

- UDP buffer size: 4096 bytes (4 KB)
- Practical encrypted message size: ~3500 bytes
- Remaining space is used by:
  - nonce
  - signature
  - public keys
  - packet metadata
- Large messages are NOT supported in this prototype.

-----------------------------------------
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

-----------------------------------------
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
---------------------------------------

- End-to-end encrypted
- Authenticated sender
- Perfect Forward Secrecy
- No plaintext data on the network
- Minimal metadata exposure
- No static identifiers

-----------------------------------------
##LIMITATIONS (KNOWN)
--------------------------------------

- One-way communication only
- No packet retransmission
- No message fragmentation
- Uses IP + port (only for prototype)
- No routing, relays, or multipath yet
- No protection against traffic analysis 

------------------------------------------
```bash
