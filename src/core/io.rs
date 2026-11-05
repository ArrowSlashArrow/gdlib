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

/// Returns the encrypted level string as `Vec<u8>` from a `GDLevel` object string
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

/// Parses an XML dictionary to a string that matches GD savefile format.
///
/// # Arguments
/// * `dict`: `plist::Dictionary` to parse.
/// * `root`: Is the input the root dict?
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
        let _ = write!(dict_str, "<k>{}</k>", escape_xml(key));
        match value {
            Value::String(s) => {
                let _ = write!(dict_str, "<s>{}</s>", escape_xml(s));
            }
            Value::Integer(int) => {
                let _ = write!(dict_str, "<i>{int}</i>");
            }
            Value::Dictionary(dict) => {
                dict_str.push_str(&stringify_xml(dict, false));
            }
            Value::Boolean(b) => {
                if *b {
                    dict_str.push_str("<t />");
                } else {
                    dict_str.push_str("<f />");
                }
            }
            Value::Real(float) => {
                let _ = write!(dict_str, "<r>{float}</r>");
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

/// Decompresses a `Vec<u8>` of a base64ed gzip-compressed payload.
///
/// # Arguments
/// * `data`: compressed input data
///
/// Returns decompressed base64ed gzip as a `Vec<u8>` if it sucessfully decoded, otherwise returns `GDError`
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

/// Returns CCLocalLevels.dat decrypted if it exists
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
    let mut tempfile = NamedTempFile::new_in(path_parent).unwrap();
    tempfile.write_all(data)?;
    tempfile.flush()?;

    tempfile.as_file().sync_all()?;
    tempfile.persist(path)?;

    Ok(())
}

/// Returns the standard file path of GD savefiles on linux.
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
