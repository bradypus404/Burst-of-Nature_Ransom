use rand::Rng;
use std::collections::HashMap;
//getfileinf
use std::fs;
use std::io::{self, Read};
use std::io::Write;
use std::path::PathBuf;
//aes
use std::fs::File;
use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, KeyInit, BlockDecrypt,
    generic_array::GenericArray,
};
//rsa
use rsa::RsaPrivateKey;
use rsa::pkcs1v15;

/* rsa key 생성 */
fn rsa_k()-> rsa::RsaPrivateKey{
    let mut rng = rand::thread_rng();
    let bits = 1024;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("키 생성 실패");
    return private_key;
}

/* aes키생성 => rsa암호화 => 파일에 저장*/
fn rsa(r_k: RsaPrivateKey) {
    let mut file = File::create(r"C:\Users\guswj\바탕 화면\rust\key.txt").expect("파일 생성 실패");
    let mut count = 0;
    let file_len = call_file_path().len();
    let mut rng = rand::thread_rng();

    // 키 생성
    let public_key = r_k.to_public_key();
    
    println!("{:?}",file_len);

    while count < file_len{
        count += 1;
        //aes 랜덤 키 생성
        let mut randg = rand::thread_rng();
        let mut random_bytes: [u8; 16] = [0; 16]; // 16 바이트 배열 초기화
        randg.fill(&mut random_bytes);
        println!("(+)키 원본 : {:?}",random_bytes);
        // 암호화할 데이터
        let key_data = GenericArray::from(random_bytes);

        // 데이터 암호화
        let data_encrypt = public_key.encrypt(&mut rng, pkcs1v15::Pkcs1v15Encrypt, &key_data).expect("데이터 암호화 실패");
        println!("(+)aes키를 rsa암호화 한 값 :{:?}",data_encrypt);
        file.write_all(&data_encrypt).expect("쓰기 실패");
    }
}

/* key파일 가져와서 rsa로 암호화된 key값 list에 저장 */
fn read_kdata() -> Vec<Vec<u8>>{
    let mut rk_list = Vec::new();
    match read_binary_file(r"C:\Users\guswj\바탕 화면\rust\key.txt") {
        Ok(data) => {
            let chunk_size = 128;
            let mut start = 0;
            while start < data.len() {
                let end = (start + chunk_size).min(data.len());
                let chunk = &data[start..end];
                //println!("data: {:?}", chunk);
                rk_list.push(chunk.to_vec());
                start = end;
            }
        }
        Err(e) => {
            println!("파일 읽기 오류: {}", e);
        }
    }
    return rk_list;
}

/* 파일이름:rsa암호화된 key값 딕셔너리 생성 */
fn mk_map() -> HashMap<PathBuf, Vec<u8>> {
    let file_paths = call_file_path();
    let mut fk_map = HashMap::new();
    let data_encrypt = read_kdata();

    for (i, encrypted_data) in data_encrypt.iter().enumerate() {
        if i < file_paths.len() {
            let cloned_data = encrypted_data.clone();
            fk_map.insert(file_paths[file_paths.len() - i - 1].clone(), cloned_data);
        }
    }
    println!("(+)파일이름:key 딕셔너리 : {:?}", fk_map);
    fk_map
}

/* rsa복호화 */
fn rsa_decrypt(private_key: RsaPrivateKey, map: HashMap<PathBuf, Vec<u8>>, path: PathBuf) -> Result<Vec<u8>, &'static str> {
    let file_path = PathBuf::from(path);

    if let Some(encrypted_data) = map.get(&file_path) {
        let data_decrypt = private_key.decrypt(pkcs1v15::Pkcs1v15Encrypt, encrypted_data).expect("데이터 복호화 실패");
        println!("복호화된 데이터: {:?}", data_decrypt);
        Ok(data_decrypt)
    } else {
        println!("해당 파일의 복호화된 데이터를 찾을 수 없습니다.");
        Err("데이터를 찾을 수 없음")
    }
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

fn encode_file(private_key: RsaPrivateKey, map: HashMap<PathBuf, Vec<u8>>, division:bool) {
    let file_path = call_file_path();//파일 경로 불러오기

    for one_file_path in file_path{ //리스트에서 파일경로 하나씩 꺼내오기
        if let Some(put_file_path) = one_file_path.to_str() { //PathBuf타입을 str타입으로 변환

            match read_binary_file(put_file_path) {
                Ok(data) => {
                    // 데이터 처리
                    println!("(+)file_path: {:?}",put_file_path);
                    aes_process(data, private_key.clone(), map.clone(), put_file_path, division);
                }
                Err(e) => {
                    println!("파일 읽기 오류: {}", e);
                }
            }
            println!("암호화 및 복호화 실행 완료");

        } else {
            println!("failed");
        }
    }
}

fn aes_process(mut input_data:Vec<u8>, private_key: RsaPrivateKey, map: HashMap<PathBuf, Vec<u8>>, output_file_path:&str, encrypt:bool) {
    let key_result = rsa_decrypt(private_key, map, output_file_path.into());
    println!("key_result : {:?}", key_result);

    match key_result {
        Ok(key) => {
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
                    let mut processed_data = GenericArray::clone_from_slice(&padded_chunk);
                    if encrypt {
                        println!("암호화");
                        cipher.encrypt_block(&mut processed_data);
                    } else {
                        println!("복호화");
                        cipher.decrypt_block(&mut processed_data);
                    }
                    output_file.write_all(processed_data.as_slice()).expect("Error writing file");
                } else {
                    let mut processed_data = GenericArray::clone_from_slice(chunk);
                    if encrypt {
                        println!("암호화");
                        cipher.encrypt_block(&mut processed_data);
                    } else {
                        println!("복호화");
                        cipher.decrypt_block(&mut processed_data);
                    }
                    output_file.write_all(processed_data.as_slice()).expect("Error writing file");
                }
            }
        }
        Err(err_msg) => {
            eprintln!("키 데이터 읽기 오류: {}", err_msg);
        }
    }
    
}

fn choice_crypt(){
    let r_k = rsa_k();
    rsa(r_k.clone());
    let map = mk_map();
    for i in 0..2{
        let mut input = String::new();
        println!("입력 (암호화:en 또는 복호화:de)");
        io::stdin().read_line(&mut input)
            .expect("입력을 읽을 수 없음.");
    
        // 입력값을 문자열로 변환합니다.
        let language = input.trim();
    
        // 입력값에 따라 다른 동작을 수행합니다.
        match language {
            "en" => {
                println!("암호화");
                encode_file(r_k.clone(), map.clone(), true);
            },
            "de" => {
                println!("복호화");
                encode_file(r_k.clone(), map.clone(), false);
            },
            _ => {
                println!("지원하지 않는 기능.");
            },
        }
    }
}
fn main() {
    choice_crypt();
}