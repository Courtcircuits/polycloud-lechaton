use aes_gcm_siv::aead::{OsRng, consts::True};
use std::str;
use x25519_dalek::{EphemeralSecret, PublicKey};

fn generate_key_pair() -> (EphemeralSecret, PublicKey) {
    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public_key = PublicKey::from(&secret);
    (secret, public_key)
}

fn main() {
    let (alice_secret, alice_pk) = generate_key_pair();
    let (bob_secret, bob_pk) = generate_key_pair();

    let bob_shared_secret = bob_secret.diffie_hellman(&alice_pk);
    let alice_shared_secret = alice_secret.diffie_hellman(&bob_pk);

    match bob_shared_secret.as_bytes() == alice_shared_secret.as_bytes() {
        true => {
            let serialized_secret = String::from_utf8_lossy(bob_shared_secret.as_bytes());
            println!("Alice and Bob share the secret {}", serialized_secret);
        }
        _ => {
            let serialized_secret = String::from_utf8_lossy(bob_shared_secret.as_bytes());
            println!(
                "Alice and Bob don't share the same secret {}",
                serialized_secret
            );
        }
    }
}
