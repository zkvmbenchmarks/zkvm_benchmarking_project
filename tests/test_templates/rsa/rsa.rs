use crate::env_adapters::NotImplementedEnv as env;
use rsa::Pkcs1v15Sign;
use rsa::{pkcs8::DecodePublicKey, RsaPublicKey};

#[precompile]
use sha2::{Digest, Sha256};

#[host]
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};


//implementation from https://github.com/succinctlabs/sp1/blob/dev/examples/rsa/program/src/main.rs
fn main() {
    let pk_der: Vec<u8> = env::read(); // #public_key()
    let message: Vec<u8> = env::read(); // #input_message()
    let signature: Vec<u8> = env::read(); // #input_signature()

    let public_key = RsaPublicKey::from_public_key_der(&pk_der).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(message);
    let hashed_msg = hasher.finalize();

    let verification = public_key.verify(Pkcs1v15Sign::new::<Sha256>(), &hashed_msg, &signature);

    let verified = match verification {
        Ok(_) => {
            println!("Signature verified successfully.");
            true
        }
        Err(e) => {
            println!("Failed to verify signature: {:?}", e);
            false
        }
    };

    env::commit(&verified);
}

#[host]
fn input_message() -> Vec<u8> {
    b"Hello, world!".to_vec()
}

#[host]
fn input_signature() -> Vec<u8> {
    vec![
        32, 121, 247, 109, 107, 249, 210, 178, 234, 149, 136, 242, 34, 135, 250, 127, 150, 225, 43,
        137, 241, 39, 139, 78, 179, 49, 169, 111, 200, 96, 183, 227, 70, 15, 46, 227, 114, 103,
        169, 170, 57, 107, 214, 102, 222, 13, 19, 216, 241, 134, 26, 124, 96, 202, 29, 185, 69, 4,
        204, 78, 223, 61, 124, 41, 179, 255, 84, 58, 47, 137, 242, 102, 161, 37, 45, 20, 39, 129,
        67, 55, 210, 164, 105, 82, 214, 223, 194, 201, 143, 114, 99, 237, 157, 42, 73, 50, 175,
        160, 145, 95, 138, 242, 157, 90, 100, 170, 206, 39, 80, 49, 65, 55, 202, 214, 17, 19, 183,
        244, 184, 17, 108, 171, 54, 178, 242, 137, 215, 67, 185, 198, 122, 234, 132, 240, 73, 42,
        123, 46, 201, 19, 197, 248, 9, 122, 16, 86, 67, 250, 237, 245, 43, 199, 65, 62, 153, 160,
        44, 108, 21, 125, 197, 154, 231, 115, 225, 38, 238, 229, 143, 203, 159, 65, 147, 18, 9,
        224, 14, 43, 58, 16, 7, 148, 2, 187, 97, 95, 70, 174, 68, 149, 7, 79, 223, 124, 207, 57,
        214, 242, 126, 2, 7, 3, 198, 202, 26, 136, 237, 106, 205, 11, 227, 120, 162, 104, 22, 167,
        192, 124, 239, 39, 201, 157, 45, 85, 147, 247, 1, 240, 217, 220, 218, 79, 238, 135, 100,
        22, 44, 88, 95, 9, 64, 224, 101, 57, 54, 171, 218, 6, 160, 137, 97, 114, 90, 32, 47, 184,
    ]
}

#[host]
fn public_key() ->  &'static [u8] {
    include_bytes!("rsa2048-pub.der")
}
