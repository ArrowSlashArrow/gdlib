//! This module contains various utilities for debugging and processing structs
use aho_corasick::AhoCorasick;
use base64::{DecodeError, Engine};
use std::{
    env,
    error::Error,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

pub mod io;
pub mod rand;

/// Error enum
#[derive(Debug)]
pub enum GDError {
    /// Standard IO failure
    Io(std::io::Error),
    /// Data could not be parsed from its raw form
    DecodeError(DecodeError),
    /// Unsuccessful plist parse
    BadPlist(plist::Error),
    /// Corrupted savefile
    CorruptedSavefile(String),
}

impl Error for GDError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::DecodeError(e) => e.source(),
            Self::BadPlist(e) => e.source(),
            Self::CorruptedSavefile(_) => None,
        }
    }
}

impl From<DecodeError> for GDError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}
impl From<std::io::Error> for GDError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<plist::Error> for GDError {
    fn from(value: plist::Error) -> Self {
        Self::BadPlist(value)
    }
}

impl Display for GDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DecodeError(d) => write!(f, "File decode failed: {d}"),
            Self::BadPlist(p) => write!(f, "Bad plist: {p}"),
            Self::Io(io) => write!(f, "{io}"),
            Self::CorruptedSavefile(reason) => write!(f, "Corrupted savefile: {reason}"),
        }
    }
}

/// Returns path of CCLocalLevels.dat if it exists
pub fn get_local_levels_path() -> Option<PathBuf> {
    if let Ok(local_appdata) = env::var("LOCALAPPDATA")
        && Path::new(&local_appdata).exists()
    {
        Some(format!("{local_appdata}/GeometryDash/CCLocalLevels.dat").into())
    } else {
        None
    }
}

/// Replaces Robtop's plist format with actual plist tags; i.e. `<s>` becomes `<string>`
pub fn proper_plist_tags(s: String) -> String {
    // replace gd plist with proper plist
    // using aho-corasick for single-pass instead of many .replace()s
    let find = &[
        "<k>", "</k>", "<i>", "</i>", "<d>", "</d>", "<d />", "<t/>", "<f/>", "<t />", "<f />",
        "<s>", "</s>", "<r>", "</r>",
    ];
    let replace = &[
        "<key>",
        "</key>",
        "<integer>",
        "</integer>",
        "<dict>",
        "</dict>",
        "<dict />",
        "<true/>",
        "<false/>",
        "<true />",
        "<false />",
        "<string>",
        "</string>",
        "<real>",
        "</real>",
    ];
    let ac = AhoCorasick::new(find).unwrap();
    ac.replace_all(&s, replace)
}

/// Quick function for decoding base64 bytes
#[inline(always)]
pub fn b64_decode<T: AsRef<[u8]> + Debug>(encoded: T) -> Vec<u8> {
    base64::engine::general_purpose::URL_SAFE
        .decode(encoded)
        .unwrap()
}

/// Quick function for encoding base64 bytes
#[inline(always)]
pub fn b64_encode(encoded: Vec<u8>) -> String {
    base64::engine::general_purpose::URL_SAFE.encode(encoded)
}

#[inline(always)]
/// Quick function for converting a slice of u8 to an owned String
pub fn vec_as_str(data: &[u8]) -> String {
    String::from_utf8(data.to_vec()).unwrap()
}
