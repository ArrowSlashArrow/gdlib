//! This module contains all of the de/serialization code for GD savefiles.
use std::{
    fmt::Write,
    fs,
    io::{Read, Write as IoWrite},
    path::PathBuf,
};

use crate::core::{GDError, b64_encode};
use base64::{Engine, engine::general_purpose};
use flate2::{Compression, read::DeflateDecoder, write::ZlibEncoder};
use plist::{Dictionary, Value};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

fn zlib_compress(s: &str) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(s.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

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

/// Returns the encrypted savefile string from a stringified `Levels` struct
#[must_use]
pub fn encrypt_savefile_str(s: &str) -> Vec<u8> {
    let mut encrypted = encrypt_level_str(s);
    #[cfg(feature = "parallel")]
    encrypted.par_iter_mut().for_each(|c| *c ^= 0x000b);

    #[cfg(not(feature = "parallel"))]
    for byte in &mut encrypted {
        *byte ^= 0x000b;
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
        let _ = write!(dict_str, "<k>{key}</k>");
        match value {
            Value::String(s) => {
                let _ = write!(dict_str, "<s>{s}</s>");
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
    // remove gzip header
    let sliced = &decoded[10..];

    let mut decoder = DeflateDecoder::new(sliced);
    let mut decompressed_buf = Vec::with_capacity(decoder.total_out() as usize);

    decoder.read_to_end(&mut decompressed_buf)?;

    Ok(decompressed_buf)
}

/// Decrypts data by xoring with key 11 and decompressing it.
/// This is the algorithm used to decrypt GD savefiles.
///
/// # Arguments
/// * `data`: encrypted payload
///
/// Returns the raw file contents as a `Vec<u8>`
#[inline]
#[must_use]
pub fn decrypt(mut data: Vec<u8>) -> Result<Vec<u8>, GDError> {
    #[cfg(feature = "parallel")]
    data.par_iter_mut().for_each(|c| *c ^= 0x000b);

    #[cfg(not(feature = "parallel"))]
    for c in &mut data {
        *c ^= 0x000b;
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
