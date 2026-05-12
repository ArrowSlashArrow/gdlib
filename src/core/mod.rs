//! This module contains various utilities for debugging and processing structs
use aho_corasick::AhoCorasick;
use base64::{DecodeError, Engine};
use std::{
    env,
    error::Error,
    fmt::{Debug, Display},
    path::PathBuf,
};

pub mod io;
pub mod rand;

/// Standard file path of GD savefiles on linux
pub const LINUX_GD_FILES: &str = "~/.local/share/Steam/steamapps/compatdata/322170/pfx/drive_c/users/steamuser/AppData/Local/GeometryDash";

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
    /// Unable to find the savefile
    MissingSavefile,
}

impl Error for GDError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::DecodeError(e) => e.source(),
            Self::BadPlist(e) => e.source(),
            Self::CorruptedSavefile(_) => None,
            Self::MissingSavefile => None,
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
            Self::MissingSavefile => write!(f, "Unable to find savefile."),
        }
    }
}

/// Checks if the standard path
pub fn get_local_levels_path() -> Option<PathBuf> {
    if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
        let path = PathBuf::from(format!("{local_appdata}/GeometryDash/CCLocalLevels.dat"));
        if path.exists() {
            return Some(path);
        }
    }

    let linux_path = PathBuf::from(format!("{LINUX_GD_FILES}/CCLocalLevels.dat"));
    if linux_path.exists() {
        return Some(linux_path);
    }

    None
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
