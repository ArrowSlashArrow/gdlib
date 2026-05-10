//! This module contains various utilities for debugging and processing structs
use aho_corasick::AhoCorasick;
use base64::{DecodeError, Engine};
use std::{
    env,
    error::Error,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

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

/// `GDLib`'s primary error type.
#[derive(Debug)]
#[non_exhaustive]
pub enum GDError {
    /// Standard `std::io::Error`
    Io(std::io::Error),
    /// Data could not be parsed from its raw form
    DecodeError(DecodeError),
    /// Unsuccessful plist parse
    BadPlist(plist::Error),
    /// `AhoCorasick` error (UTF-8 parsing issues)
    AhoCorasick(aho_corasick::BuildError),
    /// `FromUtf8Error` when converting decrypted bytes to string
    FromUtf8Error(std::string::FromUtf8Error),
    /// The savefile was invalid and could not be parsed
    BadParse,
    /// No available save file was found to use.
    NoAvailableSavefile,
}

impl Error for GDError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::DecodeError(e) => e.source(),
            Self::BadPlist(e) => e.source(),
            Self::AhoCorasick(e) => Some(e),
            Self::FromUtf8Error(e) => Some(e),
            Self::BadParse | Self::NoAvailableSavefile => None,
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
impl From<aho_corasick::BuildError> for GDError {
    fn from(value: aho_corasick::BuildError) -> Self {
        Self::AhoCorasick(value)
    }
}
impl From<std::string::FromUtf8Error> for GDError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(value)
    }
}
// TODO: make this more verbose, probably better?
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
            Self::BadParse => write!(f, "Savefile was invalid and could not be parsed!"),
            Self::NoAvailableSavefile => write!(f, "No available save file found!"),
        }
    }
}

/// Returns path of CCLocalLevels.dat if it exists
#[must_use]
// -- TODO --: The fn only supports Windows, need to implement other OSes
pub fn get_local_levels_path() -> Option<PathBuf> {
    // TODO: need better way to validate/ensure Windows platforms
    // TODO: need better way to check all Windows locations for gd?
    // TODO: make into const
    if let Ok(local_appdata) = env::var("LOCALAPPDATA")
        && Path::new(&local_appdata).exists()
    {
        Some(format!("{local_appdata}/GeometryDash/CCLocalLevels.dat").into())
    } else {
        None
    }
}

/// Replaces Robtop's plist format with actual plist tags; i.e. `<s>` becomes `<string>`
pub fn proper_plist_tags(s: String) -> Result<String, GDError> {
    // replace gd plist with proper plist; use aho-corasick for single-pass instead of many .replace()s
    let ac = AhoCorasick::new(PLIST_TAGS_FIND)?;
    Ok(ac.replace_all(&s, &PLIST_TAGS_REPLACE))
}

/// Quick function for decoding base64 bytes
#[inline]
pub fn b64_decode<T: AsRef<[u8]>>(encoded: T) -> Result<Vec<u8>, GDError> {
    Ok(base64::engine::general_purpose::URL_SAFE
        .decode(encoded)?)
}

/// Quick function for encoding base64 bytes
#[inline]
#[must_use]
pub fn b64_encode<T: AsRef<[u8]>>(encoded: T) -> String {
    base64::engine::general_purpose::URL_SAFE.encode(encoded)
}