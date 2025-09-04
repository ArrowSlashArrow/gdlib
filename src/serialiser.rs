use std::{io::Write};

use base64::Engine;
use flate2::{write::ZlibEncoder, Compression};
use plist::{Dictionary, Value};

use crate::deserialiser::xor;

fn zlib_compress(s: String) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(s.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

pub fn encrypt_level_str(s: String) -> Vec<u8> {
    let compress = zlib_compress(s.clone());
    let mut data = b"H4sIAAAAAA".to_vec();
    data.extend_from_slice(&compress[2..compress.len() - 4]);
    let crc_checksum = crc32fast::hash(&s.as_bytes()[..]).to_le_bytes();
    let size = s.len().to_le_bytes();

    data.extend_from_slice(&crc_checksum);
    data.extend_from_slice(&size);
    let base64 = base64::engine::general_purpose::STANDARD.encode(data)
        .replace("+", "-").replace("/", "_");

    let mut header = b"H4sIAAAAAAAAC".to_vec();
    header.extend_from_slice(&base64.as_bytes()[13..]);
    return header;
}

pub fn encrypt_savefile_str(s: String) -> Vec<u8> {
    let compressed = zlib_compress(s.clone());

    let mut gzip_signature = b"\x1f\x8b\x08\x00\x00\x00\x00\x00\x00\x0b".to_vec();
    let deflate = &compressed[2..compressed.len() - 4];
    let crc_checksum = crc32fast::hash(&s.as_bytes()[..]).to_le_bytes();
    let size = s.len().to_le_bytes();

    gzip_signature.extend_from_slice(&deflate);
    gzip_signature.extend_from_slice(&crc_checksum);
    gzip_signature.extend_from_slice(&size);
    let base64 = base64::engine::general_purpose::URL_SAFE.encode(gzip_signature);
    return xor(base64.as_bytes().to_vec(), 11);
}

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