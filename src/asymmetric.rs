use core::str;

use aes_gcm_siv::aead::OsRng;
// the rsa crate is still in development
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

///////////////////
// This is by far my worst code example, none of this should be used in production,
// it serves solely as a piece of example.
//////////////////

fn main() {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Encrypt
    let data = b"Hello";
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]) // Some functions underlying this
                                                       // implementation are note in constant
                                                       // time https://people.redhat.com/~hkario/marvin/
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = priv_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);

    let dec_data_str = str::from_utf8(&dec_data).unwrap();

    println!("{}", dec_data_str);
}
