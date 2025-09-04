use std::{error::Error, fs, io::Read};

use base64::{engine::general_purpose, Engine};
use flate2::read::{DeflateDecoder};

use crate::utils::{get_local_levels_path, vec_as_str};

pub fn xor(bytes: Vec<u8>, key: u8) -> Vec<u8> {
    let xored = bytes.iter().map(|c| *c ^ key).collect();
    xored
}

pub fn decompress(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    // convert from url-safe base64
    let replaced = data.iter().filter_map(|c| match c {
        b'-' => Some(b'+'),
        b'_' => Some(b'/'),
        b'\0' => None,
        _ => Some(*c)
    }).collect::<Vec<u8>>();

    // dbg!(vec_as_str(&replaced[..100].to_vec()));

    // remove all trailing null chars
    let last_non_null = replaced.iter().rposition(|&c| c != 0).unwrap_or(0);
    // decode DEFLATE payload
    let decoded = general_purpose::STANDARD.decode(&replaced[..last_non_null + 1]).expect("base64 decode rer");
    let sliced = &decoded[10..];

    let mut decoder = DeflateDecoder::new(sliced);
    let mut decompressed_buf = Vec::new();

    decoder.read_to_end(&mut decompressed_buf).expect("docder error");

    Ok(decompressed_buf)
}

pub fn decrypt(data: Vec<u8>) -> Vec<u8> {
    decompress(xor(data, 11)).unwrap()
}

pub fn decode_levels_to_string() -> Result<String, Box<dyn Error>> {
    let savefile = fs::read(get_local_levels_path()?)?;
    let data = decrypt(savefile);

    Ok(vec_as_str(&data))
}

