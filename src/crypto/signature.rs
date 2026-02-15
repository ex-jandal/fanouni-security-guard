use ed25519_dalek::{Signature, Signer, SigningKey};
use rand_core::OsRng;

use crate::SIG_KEY;

pub fn sign_artwork(data: &[u8]) -> String {
    let priv_key_hex = SIG_KEY.as_str(); 
    let priv_key_bytes = hex::decode(priv_key_hex).unwrap();
    
    let signing_key = SigningKey::from_bytes(
        priv_key_bytes.as_slice().try_into().unwrap()
    );
    
    // Sign the raw bytes of the image
    let signature: Signature = signing_key.sign(data);
    
    hex::encode(signature.to_bytes())
}

pub fn generate_keys() {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    println!("ED25519 PRIVATE_KEY (Hex): {}", 
        hex::encode(signing_key.to_bytes())
    );
}
