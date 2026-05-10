//! Here lies the decryption functions for GD savefiles
use std::{fs, io::Read};

use base64::{Engine, engine::general_purpose};
use flate2::read::DeflateDecoder;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::{core::GDError, core::get_local_levels_path};

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
pub fn decode_levels_to_string() -> Result<String, GDError> {
    let path = get_local_levels_path()
        .ok_or(GDError::NoAvailableSavefile)?;
    let savefile = fs::read(&path)?;
    let data = decrypt(savefile)?;

    Ok(String::from_utf8(data)?)
}
