//! Encryption and Decryption using Blowfish with ECB mode.
// Copyright (c) 2017 Daniel Rivas
// Used under permission of the MIT license from
// https://github.com/CMatri/pandora-rs2/blob/master/src/crypt.rs
// SPDX-License-Identifier: MIT

use blowfish::Blowfish;
use cipher::block_padding::NoPadding;
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit};

const PADDING_BYTE: u8 = 2;
const BLOCK_LEN: usize = 8;

/// Returns the encrypted input using the given key.
///
/// The returned string is encoded in hexadecimal notation,
/// which is a UTF-8 string, so it's fine to return it using
/// the `String` type.
pub fn encrypt(key: &str, input: &str) -> String {
    let mut inputbytes = input.as_bytes().to_vec();
    let padded_len = round_len(inputbytes.len(), BLOCK_LEN);
    inputbytes.resize(padded_len, PADDING_BYTE);

    let encryptor: Blowfish =
        Blowfish::new_from_slice(key.as_bytes()).expect("Invalid key: unsupported key length");

    let cipherbytes = encryptor
        .encrypt_padded_mut::<NoPadding>(&mut inputbytes, padded_len)
        .expect("Error encrypting input");

    // Generate hexadecimal representation of `cipherbytes`.
    let mut output = String::with_capacity(cipherbytes.len() * 2);
    for b in cipherbytes {
        output.push_str(&format!("{b:02x}"));
    }
    output
}

/// Returns the decrypted input using the given key.
///
/// Because Strings must be UTF-8 compilant, and decrypting
/// doesn't guarantees an UTF-8 string, we return
/// a OsString which doesn't have to be UTF-8 compilant.
pub fn decrypt(key: &str, hex_input: &str) -> Vec<u8> {
    use std::str;
    use std::u8;

    // Gets bytes from hexadecimal representation.
    let mut inputbytes = Vec::with_capacity(hex_input.len());
    for chunk in hex_input.as_bytes().chunks(2) {
        // `chunk` is utf-8 since it is comming from &str.
        let fragment = unsafe { str::from_utf8_unchecked(chunk) };
        let byte = u8::from_str_radix(fragment, 16).unwrap_or(0);
        inputbytes.push(byte);
    }

    let decryptor: Blowfish =
        Blowfish::new_from_slice(key.as_bytes()).expect("Invalid key: unsupported key length");
    let mut cipherbytes = decryptor
        .decrypt_padded_mut::<NoPadding>(&mut inputbytes)
        .expect("Error decrypting input")
        .to_vec();

    // Ignore up to `PADDING_BYTE`.
    if let Some(index) = cipherbytes.iter().position(|&b| b == PADDING_BYTE) {
        cipherbytes.truncate(index);
    }

    cipherbytes
}

/// Rounds the given len so that it contains blocks
/// of the same size.
fn round_len(len: usize, block_size: usize) -> usize {
    let remainder = len % block_size;
    if remainder == 0 {
        len
    } else {
        len + block_size - remainder
    }
}

#[cfg(test)]
mod tests {
    use super::encrypt;

    struct Test {
        key: String,
        plain_text: String,
        cipher_text: String,
    }

    fn get_test_vector() -> Vec<Test> {
        vec![Test {
            key: "R=U!LH$O2B#".to_owned(),
            plain_text: "è.<Ú1477631903".to_owned(),
            cipher_text: "4a6b45612b018614c92c50dc73462bbd".to_owned(),
        }]
    }

    #[test]
    fn encrypt_test_vector() {
        for test in get_test_vector() {
            let cipher_text = encrypt(&test.key, &test.plain_text);
            assert_eq!(test.cipher_text, cipher_text);
        }
    }
}
