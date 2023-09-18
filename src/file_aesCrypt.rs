use std::fs::File;
use std::io::{Read, Write};
use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, KeyInit,
    generic_array::{GenericArray, typenum::U16},
};

fn main() {
    let key = GenericArray::from([0u8; 16]);
    println!("key : {:?}",key);
    let block_size = 16; // AES block size in bytes

    let mut input_file = File::open("C:\Users\guswj\OneDrive\바탕 화면\rustCrypt\secret.txt").expect("Error opening file");
    let mut output_file = File::create("C:\Users\guswj\OneDrive\바탕 화면\rustCrypt\result.txt").expect("Error creating file");

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data).expect("Error reading file");
    println!("input_data : {:?}", input_data);

    let cipher = Aes128::new(&key);

    for chunk in input_data.chunks_mut(block_size) {
        // Ensure each chunk has the correct length (padding if necessary)
        let padding_len = block_size - chunk.len();
        if padding_len > 0 {
            let mut padded_chunk = chunk.to_vec();
            padded_chunk.extend_from_slice(&vec![0u8; padding_len]);
            let mut encrypted_data = GenericArray::clone_from_slice(&padded_chunk);
            cipher.encrypt_block(&mut encrypted_data);
            output_file.write_all(encrypted_data.as_slice()).expect("Error writing file");
        } else {
            let mut encrypted_data = GenericArray::clone_from_slice(chunk);
            cipher.encrypt_block(&mut encrypted_data);
            output_file.write_all(encrypted_data.as_slice()).expect("Error writing file");
        }
    }
}