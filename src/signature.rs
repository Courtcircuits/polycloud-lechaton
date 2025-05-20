use aes_gcm_siv::aead::OsRng;
use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use ed25519_dalek::ed25519::signature::Signer;

fn main() {
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let message: &[u8] = b"Polycloud 2025";
    let signature: Signature = signing_key.sign(message);

    assert!(signing_key.verify(message, &signature).is_ok());
}
