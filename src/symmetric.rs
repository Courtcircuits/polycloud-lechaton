use aes_gcm_siv::{
    Aes256GcmSiv, KeyInit, Nonce,
    aead::{Aead, OsRng},
};

fn main() {
    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(b"hello world");
    let ciphertext = cipher
        .encrypt(nonce, b"plaintext message".as_ref())
        .unwrap();

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    println!("plaintext: {}", String::from_utf8_lossy(&plaintext));
}
