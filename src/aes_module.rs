extern crate crypto;
use crypto::symmetriccipher::{SynchronousStreamCipher, Decryptor, Encryptor};
use crypto::aes::KeySize;
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

fn main() {
    // AES 키와 IV를 정의
    let key = b"0123456789abcdef"; // 16바이트 키
    let iv = b"0123456789abcdef";  // 16바이트 IV

    // 평문을 바이트 슬라이스로 정의
    let plaintext = b"aesnoanswerhahahahahhawhat";
    println!("plaintext : {:?}",&plaintext); // checkkkkkkkk

    // AES 암호화기를 생성
    let mut encryptor = crypto::aes::cbc_encryptor(
        KeySize::KeySize128,
        key,
        iv,
        crypto::blockmodes::PkcsPadding,
    );

    // 암호화 결과를 저장할 버퍼를 생성
    let mut ciphertext = Vec::<u8>::new();
    println!("ciphertext1 : {:?}",&ciphertext); // checkkkkkkkk

    // 평문을 암호화
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(plaintext);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut ciphertext);


    encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    println!("Ciphertext: {:?}", &ciphertext);
    



    // AES 복호화기를 생성
    let mut decryptor = crypto::aes::cbc_decryptor(
        KeySize::KeySize128,
        key,
        iv,
        crypto::blockmodes::PkcsPadding,
    );

    // 복호화 결과를 저장할 버퍼를 생성
    let mut decrypted_text = Vec::<u8>::new();

    // 암호문을 복호화
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&ciphertext);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut decrypted_text);

    decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();

    println!(
        "Decrypted: {:?}",
        String::from_utf8(decrypted_text).unwrap()
    );

}