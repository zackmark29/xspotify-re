use std::error;

use aes_ctr::Aes128Ctr;
use aes_ctr::stream_cipher::generic_array::GenericArray;
use aes_ctr::stream_cipher::{
    NewStreamCipher, SyncStreamCipher
};

const HEX_IV_KEY: &[u8; 32] = b"72E067FBDDCBCF77EBE8BC643F630D93";

pub fn decrypt(encrypted: String) -> Result<String, Box<dyn error::Error>> {
    let mut data = encrypted.into_bytes();

    let key = GenericArray::from_slice(b"very secret key.");
    let nonce = GenericArray::from_slice(HEX_IV_KEY);
    let mut cipher = Aes128Ctr::new(&key, &nonce);

    cipher.apply_keystream(&mut data);


    Ok(String::new())
}