//! Here lies the decryption functions for GD savefiles 
use std::{error::Error, fs, io::Read};

use base64::{engine::general_purpose, Engine};
use flate2::read::DeflateDecoder;

use crate::utils::get_local_levels_path;

/// Decompresses a `Vec<u8>` of a base64ed gzip-compressed payload.
/// 
/// # Arguments
/// * `data`: compressed input data
/// 
/// Returns decompressed base64ed gzip as a `Vec<u8>` if it sucessfully decoded, otherwise return Error
pub fn decompress<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, Box<dyn Error>> {
    // convert from url-safe base64
    let replaced = data.as_ref().iter().filter_map(|c| match c {
        b'-' => Some(b'+'),
        b'_' => Some(b'/'),
        b'\0' => None,
        _ => Some(*c)
    }).collect::<Vec<u8>>();

    // decode DEFLATE payload
    let decoded = general_purpose::STANDARD.decode(&replaced[..])?;
    let sliced = &decoded[10..];

    let mut decoder = DeflateDecoder::new(sliced);
    let mut decompressed_buf = Vec::new();

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
pub fn decrypt(data: Vec<u8>) -> Vec<u8> {
    decompress(data.iter().map(|c| *c ^ 11).collect::<Vec<u8>>()).unwrap()
}

/// Returns CCLocalLevels.dat decrypted if it exists
pub fn decode_levels_to_string() -> Result<String, Box<dyn Error>> {
    let savefile = fs::read(get_local_levels_path()?)?;
    let data = decrypt(savefile);

    Ok(String::from_utf8(data.to_vec()).unwrap())
}

