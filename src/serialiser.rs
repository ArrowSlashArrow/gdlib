//! This module contains all of the encryption code for GD savefiles.
use std::{io::Write};

use flate2::{write::ZlibEncoder, Compression};
use plist::{Dictionary, Value};

use crate::{utils::b64_encode};

fn zlib_compress(s: String) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(s.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

/// Returns the encrypted level string as `Vec<u8>` from a `GDLevel` object string
pub fn encrypt_level_str(s: String) -> Vec<u8> {
    let compress = zlib_compress(s.clone());
    let mut data = b"H4sIAAAAAA".to_vec();
    data.extend_from_slice(&compress[2..compress.len() - 4]);
    let crc_checksum = crc32fast::hash(&s.as_bytes()[..]).to_le_bytes();
    let size = s.len().to_le_bytes();

    data.extend_from_slice(&crc_checksum);
    data.extend_from_slice(&size);
    let base64 = b64_encode(data)
        .replace("+", "-").replace("/", "_");

    let mut header = b"H4sIAAAAAAAAC".to_vec();
    header.extend_from_slice(&base64.as_bytes()[13..]);
    return header;
}

/// Returns the encrypted savefile string from a stringified `Levels` struct
pub fn encrypt_savefile_str(s: String) -> Vec<u8> {
    return encrypt_level_str(s).iter().map(|c| *c ^ 11).collect();
}

/// Parses an XML dictionary to a string that matches GD savefile format.
/// 
/// # Arguments
/// * `dict`: plist::Dictionary to parse.
/// * `root`: Is the input the root dict?
/// 
/// Returns the stringified dictionary
pub fn stringify_xml(dict: &Dictionary, root: bool) -> String {
    if dict.is_empty() {
        return "<d />".to_owned();
    };

    let mut dict_str = String::from(match root {
        true => "<dict>",
        false => "<d>"
    });
    for (key, value) in dict.iter() {
        dict_str += &format!("<k>{key}</k>");
        match value {
            Value::String(s) => {
                dict_str += &format!("<s>{s}</s>");
            },
            Value::Integer(int) => {
                dict_str += &format!("<i>{int}</i>");
            },
            Value::Dictionary(dict) => {
                dict_str += &stringify_xml(dict, false);
            },
            Value::Boolean(b) => {
                dict_str += &format!("<{} />", match b {
                    true => 't',
                    false => 'f'
                });
            },
            Value::Real(float) => {
                dict_str += &format!("<r>{float}</r>");
            },
            _ => {}
        }
    }

    dict_str += match root {
        true => "</dict>",
        false => "</d>"
    };
    return dict_str
}