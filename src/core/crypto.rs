//! Cryptographic algorithms

use sha1::{Digest, Sha1};

const PASSWORD_SALT: &str = "mI29fmAnxgTs";

/// Generate hash from given password according to the GJP2 cipher and return the hash as a hexdigest
pub fn generate_gjp2_hexdigest(password: String) -> String {
    let salted = password + PASSWORD_SALT;
    hex::encode(Sha1::digest(salted.as_bytes()))
}
