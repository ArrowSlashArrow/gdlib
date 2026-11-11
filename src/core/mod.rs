//! Shared foundations: the error type, savefile lookup and encoding helpers.
//!
//! - [`GDError`]: the error type used across the crate.
//! - [`get_cclocallevels_path`] and [`get_ccgamemanager_path`]: find GD's savefiles on Windows and Linux.
//! - [`proper_plist_tags`]: converts GD's shortened plist tags into standard ones.
//! - [`b64_encode`] and [`b64_decode`]: URL-safe base64, as GD uses.
//!
//! # Modules
//!
//! - [`io`]: savefile and level string encoding.
//! - [`rand`]: GD's random number generator.
//! - [`crypto`]: password hashing.
//! - [`structs`]: types shared by more than one savefile.

use aho_corasick::AhoCorasick;
use base64::{DecodeError, Engine};
use std::{
    env,
    error::Error,
    fmt::{Debug, Display},
    path::PathBuf,
    sync::LazyLock,
};

use crate::core::io::linux_gd_files_path;

pub mod crypto;
pub mod io;
pub mod rand;
pub mod structs;

macro_rules! count_exprs {
    () => { 0 };
    ($e:expr) => { 1 };
    ($e:expr, $($es:expr),+) => { 1 + count_exprs!($($es),*) };
}
macro_rules! build_plist_tags {
    ($($find:expr => $replace:expr),* $(,)?) => {
        const PLIST_TAGS_FIND: [&str; count_exprs!($($find),*)] = [$($find),*];
        const PLIST_TAGS_REPLACE: [&str; count_exprs!($($replace),*)] = [$($replace),*];
    };
}

build_plist_tags! {
    "<k>" => "<key>",
    "</k>" => "</key>",
    "<i>" => "<integer>",
    "</i>" => "</integer>",
    "<d>" => "<dict>",
    "</d>" => "</dict>",
    "<d />" => "<dict />",
    "<t/>" => "<true/>",
    "<f/>" => "<false/>",
    "<t />" => "<true />",
    "<f />" => "<false />",
    "<s>" => "<string>",
    "</s>" => "</string>",
    "<r>" => "<real>",
    "</r>" => "</real>",
}

static PLIST_TAGS_AC: LazyLock<AhoCorasick> = LazyLock::new(|| {
    AhoCorasick::new(PLIST_TAGS_FIND).expect("PLIST_TAGS_FIND contains valid patterns")
});

/// `GDLib`'s primary error type.
#[derive(Debug)]
#[non_exhaustive]
pub enum GDError {
    /// Standard `std::io::Error`
    Io(std::io::Error),
    /// Base64 decoding failed, e.g. for a level string or savefile that isn't valid URL-safe base64
    DecodeError(DecodeError),
    /// Unsuccessful plist parse
    BadPlist(plist::Error),
    /// Corrupted savefile
    CorruptedSavefile(String),
    /// Unable to find the savefile
    MissingSavefile,
    /// Building the Aho-Corasick automaton used by [`proper_plist_tags`] failed.
    ///
    /// The automaton's patterns are fixed and it is built once, so the crate no longer returns this variant.
    AhoCorasick(aho_corasick::BuildError),
    /// `FromUtf8Error` when converting decrypted bytes to string
    FromUtf8Error(std::string::FromUtf8Error),
}

impl Error for GDError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::DecodeError(e) => e.source(),
            Self::BadPlist(e) => e.source(),
            Self::AhoCorasick(e) => Some(e),
            Self::FromUtf8Error(e) => Some(e),
            Self::CorruptedSavefile(_) | Self::MissingSavefile => None,
        }
    }
}

// macro just for GDError
macro_rules! from_error {
    ($fp:path => $err:ident) => {
        impl From<$fp> for GDError {
            fn from(value: $fp) -> Self {
                Self::$err(value)
            }
        }
    };
}

from_error!(DecodeError => DecodeError);
from_error!(std::io::Error => Io);
from_error!(plist::Error => BadPlist);
from_error!(aho_corasick::BuildError => AhoCorasick);
from_error!(std::string::FromUtf8Error => FromUtf8Error);

impl From<GDError> for std::fmt::Error {
    fn from(_: GDError) -> Self {
        std::fmt::Error
    }
}

impl Display for GDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DecodeError(d) => write!(f, "File decode failed: {d}"),
            Self::BadPlist(p) => write!(f, "Bad plist: {p}"),
            Self::Io(io) => write!(f, "IO failed: {io}"),
            Self::AhoCorasick(a) => write!(f, "AhoCorasick UTF-8 build error: {a}"),
            Self::FromUtf8Error(e) => write!(f, "UTF-8 conversion error: {e}"),
            Self::CorruptedSavefile(e) => write!(f, "Corrupted savefile: {e}"),
            Self::MissingSavefile => write!(f, "No available save file found!"),
        }
    }
}

/// Returns path of CCLocalLevels.dat if it exists
///
/// Checks `%LOCALAPPDATA%\GeometryDash` first, then the Steam Proton path from [`linux_gd_files_path`].
#[must_use]
pub fn get_cclocallevels_path() -> Option<PathBuf> {
    if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
        let path = PathBuf::from(format!("{local_appdata}/GeometryDash/CCLocalLevels.dat"));
        if path.exists() {
            return Some(path);
        }
    }

    let linux_path = linux_gd_files_path()?.join("CCLocalLevels.dat");
    if linux_path.exists() {
        return Some(linux_path);
    }

    None
}

/// Returns path to CCGameManager.dat if it exists
///
/// Checks `%LOCALAPPDATA%\GeometryDash` first, then the Steam Proton path from [`linux_gd_files_path`].
pub fn get_ccgamemanager_path() -> Option<PathBuf> {
    if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
        let path = PathBuf::from(format!("{local_appdata}/GeometryDash/CCGameManager.dat"));
        if path.exists() {
            return Some(path);
        }
    }

    let linux_path = linux_gd_files_path()?.join("CCGameManager.dat");
    if linux_path.exists() {
        return Some(linux_path);
    }

    None
}

/// Replaces Robtop's plist format with actual plist tags; i.e. `<s>` becomes `<string>`
///
/// This function currently always returns `Ok`.
pub fn proper_plist_tags(s: String) -> Result<String, GDError> {
    // replace gd plist with proper plist; use aho-corasick for single-pass instead of many .replace()s
    Ok(PLIST_TAGS_AC.replace_all(&s, &PLIST_TAGS_REPLACE))
}

/// Decodes base64 bytes using the URL-safe alphabet (`-` and `_`), which is what GD uses.
#[inline]
pub fn b64_decode<T: AsRef<[u8]>>(encoded: T) -> Result<Vec<u8>, GDError> {
    Ok(base64::engine::general_purpose::URL_SAFE.decode(encoded)?)
}

/// Encodes bytes as padded base64 using the URL-safe alphabet (`-` and `_`), which is what GD uses.
#[inline]
#[must_use]
pub fn b64_encode<T: AsRef<[u8]>>(encoded: T) -> String {
    base64::engine::general_purpose::URL_SAFE.encode(encoded)
}
