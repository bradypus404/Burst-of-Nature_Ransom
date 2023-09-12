use std::fs::File;
use std::io::{Read, Write};
use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, KeyInit,
    generic_array::{GenericArray, typenum::U16},
};

fn main() {
    let key = GenericArray::from([0u8; 16]);
    println!("{:?}",key);
    let mut input_file = File::open("C:\\Rust\\aes3\\test_aes.txt").expect("Error open file");

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data).expect("Error read file");

    println!("{:?}", input_data);

    // Initialize cipher
    let cipher = Aes128::new(&key);
    
    // Ensure the input_data has the correct length
    let mut input_data_array = GenericArray::default();
    let len = input_data.len();
    assert_eq!(len, input_data_array.len());
    input_data_array.copy_from_slice(&input_data);

    // Encrypt the data
    let mut encrypted_data = input_data_array;
    cipher.encrypt_block(&mut encrypted_data);

    let mut output_file = File::create("C:\\Rust\\aes3\\output.txt").expect("Error create file");

    output_file.write_all(encrypted_data.as_slice()).expect("Error write file");
}
