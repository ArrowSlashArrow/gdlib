//! Password hashing, as used by GD's servers and savefiles.
//!
//! [`generate_gjp2_hexdigest`] produces a GJP2 hash: the SHA-1 hex digest of the password followed by a fixed salt. It's the format stored in [`GDAccount::hashed_password`](crate::ccgamemanager::structs::GDAccount::hashed_password).
//!
//! ```
//! use gdlib::core::crypto::generate_gjp2_hexdigest;
//!
//! let hash = generate_gjp2_hexdigest("password".to_string());
//! assert_eq!(hash.len(), 40);
//! ```
//!
//! GJP2 is a compatibility format, not secure password storage. Treat these hashes as sensitive anyway.
//!
//! **Note:** This file is intended to store all cryptographic functions (besides encoding/decoding algorithms for savefiles) used in GD. As of the current version of GDLib, there are just not a lot of them implemented.

use sha1::{Digest, Sha1};

const PASSWORD_SALT: &str = "mI29fmAnxgTs";

/// Generate hash from given password according to the GJP2 cipher and return the hash as a hexdigest.
///
/// This hash is used when querying the GD servers and for storing the player's password.
pub fn generate_gjp2_hexdigest(password: String) -> String {
    let salted = password + PASSWORD_SALT;
    hex::encode(Sha1::digest(salted.as_bytes()))
}
