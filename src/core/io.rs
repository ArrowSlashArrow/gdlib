//! Low-level encoding and decoding for GD savefiles and level strings.
//!
//! Most programs don't need this module, because [`GDLevel`](crate::cclocallevels::gdlevel::GDLevel) and [`CCLocalLevels`](crate::cclocallevels::gdlevel::CCLocalLevels) call it for you.
//!
//! # Encoding layers
//!
//! | Data | Writing | Reading |
//! |---|---|---|
//! | Level strings and `.gdlvl` files | gzip, then URL-safe base64: [`encrypt_level_str`] | [`decompress`] |
//! | Savefiles (`.dat`) | The above, then XOR with [`XOR_KEY_PLAYER_DATA`]: [`encrypt_savefile_str`] | [`decrypt`], or [`decrypt_file`] for a path |
//!
//! Plists are written in GD's shortened tag format by [`stringify_xml`]. To read them, convert the tags with [`proper_plist_tags`](crate::core::proper_plist_tags) first.
//!
//! # Common pitfalls
//!
//! - **[`decrypt_file`] removes all non-ASCII bytes.** It does this to cope with stray characters in CCGameManager.dat, but it also damages non-ASCII text such as level names.
//! - **[`decompress`] assumes a gzip header.** It skips a fixed 10 bytes rather than reading the header.

use std::{
    borrow::Cow,
    env,
    fmt::Write,
    fs,
    io::{Read, Write as IoWrite},
    path::{Path, PathBuf},
};

use crate::core::{GDError, b64_encode};
use base64::{Engine, engine::general_purpose};
use flate2::{Compression, read::DeflateDecoder, write::ZlibEncoder};
use plist::{Dictionary, Value};
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use tempfile::NamedTempFile;

fn zlib_compress(s: &str) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(s.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

/// Xor encryption key for CCLocalLevels.dat and CCGameManager.dat
pub const XOR_KEY_PLAYER_DATA: u8 = 0x0b;

/// Gzip-compresses `s` and encodes it as URL-safe base64, which is the format of a level's `k4` string and of `.gdlvl` files.
///
/// Despite the name, no encryption is involved. Reverse it with [`decompress`].
#[must_use]
pub fn encrypt_level_str(s: &str) -> Vec<u8> {
    let compress = zlib_compress(s);
    let mut data = b"H4sIAAAAAA".to_vec();
    data.extend_from_slice(&compress[2..compress.len() - 4]);
    let crc_checksum = crc32fast::hash(s.as_bytes()).to_le_bytes();
    let size = s.len().to_le_bytes();

    data.extend_from_slice(&crc_checksum);
    data.extend_from_slice(&size);
    let base64 = b64_encode(data);

    let mut header = b"H4sIAAAAAAAAC".to_vec();
    header.extend_from_slice(&base64.as_bytes()[13..]);
    header
}

/// Encrypts a payload into the format of a GD savefile. This function is used to export `CCLocalLevels` and `CCGameManager`.
///
/// The raw payload is first encrypted according to [`encrypt_level_str`] and then xored with [`XOR_KEY_PLAYER_DATA`].
#[must_use]
pub fn encrypt_savefile_str(s: &str) -> Vec<u8> {
    let mut encrypted = encrypt_level_str(s);
    #[cfg(feature = "parallel")]
    encrypted
        .par_iter_mut()
        .for_each(|c| *c ^= XOR_KEY_PLAYER_DATA);

    #[cfg(not(feature = "parallel"))]
    for byte in &mut encrypted {
        *byte ^= XOR_KEY_PLAYER_DATA;
    }
    encrypted
}

/// Serialises a plist dictionary to a string in GD's shortened plist format (`<k>`, `<s>`, `<d>`, ...).
///
/// # Arguments
/// * `dict`: `plist::Dictionary` to serialise.
/// * `root`: Is the input the root dict? The root dict is wrapped in `<dict>`, and nested dicts in `<d>`.
///
/// Keys and strings have `&`, `<` and `>` escaped. Only strings, integers, reals, booleans and dictionaries are written;
/// for a value of any other type (arrays, data, dates, UIDs), the key is still written but the value is silently left out.
///
/// Returns the stringified dictionary
#[must_use]
pub fn stringify_xml(dict: &Dictionary, root: bool) -> String {
    if dict.is_empty() {
        return "<d />".to_owned();
    };

    let mut dict_str = String::with_capacity(4_096);
    dict_str.push_str(match root {
        true => "<dict>",
        false => "<d>",
    });

    for (key, value) in dict {
        let escaped_key = escape_xml(&key);
        match value {
            Value::String(s) => {
                let _ = write!(dict_str, "<k>{escaped_key}</k><s>{}</s>", escape_xml(s));
            }
            Value::Integer(int) => {
                let _ = write!(dict_str, "<k>{escaped_key}</k><i>{int}</i>");
            }
            Value::Dictionary(dict) => {
                let _ = write!(dict_str, "<k>{escaped_key}</k>");
                dict_str.push_str(&stringify_xml(dict, false));
            }
            Value::Boolean(b) => {
                let _ = write!(dict_str, "<k>{escaped_key}</k>");
                if *b {
                    dict_str.push_str("<t />");
                } else {
                    dict_str.push_str("<f />");
                }
            }
            Value::Real(float) => {
                let _ = write!(dict_str, "<k>{escaped_key}</k><r>{float}</r>");
            }
            _ => {}
        }
    }

    dict_str.push_str(match root {
        true => "</dict>",
        false => "</d>",
    });
    dict_str
}

// ------------ deserialiser ------------

/// Decodes a URL-safe base64, gzip-compressed payload, such as a level's `k4` string or a `.gdlvl` file. This reverses [`encrypt_level_str`].
///
/// # Arguments
/// * `data`: compressed input data. Null bytes are removed before decoding.
///
/// Returns the decompressed bytes. Returns [`GDError::DecodeError`] if the base64 is invalid,
/// [`GDError::CorruptedSavefile`] if the decoded payload is 10 bytes or shorter, and [`GDError::Io`] if inflating fails.
pub fn decompress(mut data: Vec<u8>) -> Result<Vec<u8>, GDError> {
    data.retain(|c| *c != 0);

    // decode DEFLATE payload
    let decoded = general_purpose::URL_SAFE.decode(data)?;
    if decoded.len() <= 10 {
        return Err(GDError::CorruptedSavefile(
            "compressed payload is not gzipped".into(),
        ));
    }
    // remove gzip header
    let sliced = &decoded[10..];

    let mut decoder = DeflateDecoder::new(sliced);
    let mut decompressed_buf = Vec::with_capacity(decoder.total_out() as usize);

    decoder.read_to_end(&mut decompressed_buf)?;

    Ok(decompressed_buf)
}

/// Decrypts data by xoring with [`XOR_KEY_PLAYER_DATA`] and decompressing it, which yields the raw savefile string when used on GD savefiles - CCLocalLevels.dat and CCGameManager.dat.
///
/// The raw file's contents are returned as a byte array.
#[inline]
#[must_use]
pub fn decrypt(mut data: Vec<u8>) -> Result<Vec<u8>, GDError> {
    #[cfg(feature = "parallel")]
    data.par_iter_mut().for_each(|c| *c ^= XOR_KEY_PLAYER_DATA);

    #[cfg(not(feature = "parallel"))]
    for c in &mut data {
        *c ^= XOR_KEY_PLAYER_DATA;
    }
    decompress(data)
}

/// Reads the savefile at `file` (CCLocalLevels.dat or CCGameManager.dat) and returns its decrypted contents, using [`decrypt`].
///
/// **Every non-ASCII byte is removed** before the bytes are converted to a string, so non-ASCII text such as level names is lost or altered.
pub fn decrypt_file(file: PathBuf) -> Result<String, GDError> {
    let savefile = match fs::read(file) {
        Ok(v) => v,
        Err(e) => return Err(GDError::Io(e)),
    };
    let mut data = decrypt(savefile)?;
    data.retain(|c| *c < 128); // sometimes there are some weird chars in the ccgamemanager file

    Ok(String::from_utf8(data)?)
}

/// Warning when using this fn: if the data isn't valid UTF8, the fn WILL panic!
#[inline]
pub(crate) fn vec_as_str(data: &[u8]) -> String {
    core::str::from_utf8(data).unwrap().to_string()
}

#[inline]
pub(crate) fn vec_as_str_lossy(data: &[u8]) -> String {
    String::from_utf8_lossy(data).to_string()
}

/// Exports files atomically to prevent corruption on important files
pub(crate) fn atomic_export<P: Into<PathBuf>>(path: P, data: &[u8]) -> Result<(), std::io::Error> {
    let path = path.into();
    let path_parent = path.parent().unwrap_or_else(|| Path::new("."));
    let mut tempfile = NamedTempFile::new_in(path_parent)?;
    tempfile.write_all(data)?;
    tempfile.flush()?;

    tempfile.as_file().sync_all()?;
    tempfile.persist(path)?;

    Ok(())
}

/// Returns the standard file path of GD savefiles on linux.
///
/// This is the default Steam Proton prefix under `$HOME`. The path is not checked for existence, and `None` is returned only if `HOME` is unset.
pub fn linux_gd_files_path() -> Option<PathBuf> {
    Some(PathBuf::from(env::var("HOME").ok()?).join(".local/share/Steam/steamapps/compatdata/322170/pfx/drive_c/users/steamuser/AppData/Local/GeometryDash"))
}

/// Escapes the characters that are not allowed in XML text.
fn escape_xml(s: &str) -> Cow<'_, str> {
    if !s.contains(['&', '<', '>']) {
        return Cow::Borrowed(s);
    }

    let mut escaped = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(c),
        }
    }
    Cow::Owned(escaped)
}
