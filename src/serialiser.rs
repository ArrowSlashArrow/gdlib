//! This module contains all of the encryption code for GD savefiles.
use std::{fmt::Write, io::Write as IoWrite};

use flate2::{Compression, write::ZlibEncoder};
use plist::{Dictionary, Value};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::core::b64_encode;

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
