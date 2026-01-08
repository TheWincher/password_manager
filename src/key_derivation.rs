use argon2::{Algorithm, Argon2, Params, Version, password_hash::SaltString};
use rand::rngs::OsRng;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;


pub fn derive_key(master_password: &str) -> ([u8; 32], [u8; 16]) {
    let salt = SaltString::generate(&mut OsRng);
    derive_key_with_salt(master_password, salt.as_str().as_bytes().try_into().expect("error"))
}

fn derive_key_with_salt(master_password: &str, salt: &[u8; 16]) -> ([u8; 32], [u8; 16]) {
    let mut key = vec![0u8; 32];

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(65536, 2, 1, Some(32)).unwrap());
    argon2.hash_password_into(master_password.as_bytes(), salt, &mut key).expect("Error when hash password");

    (key.try_into().expect("error"), *salt)
}

pub fn create_verifier(derive_key: &[u8; 32]) -> [u8; 32] {
    let magic_key = b"magic-pwd";

    let mut mac = Hmac::<Sha256>::new_from_slice(magic_key).expect("Invalid key");
    mac.update(derive_key);

    mac.finalize().into_bytes().try_into().expect("error")
}

pub fn verify_password(master_password: &str, salt: &[u8; 16], verifier: &[u8;32]) -> bool {
    let (derive_key, _) = derive_key_with_salt(master_password, salt);
    let computed_verifier = create_verifier(&derive_key);

    return verifier.ct_eq(&computed_verifier).unwrap_u8() == 1
}