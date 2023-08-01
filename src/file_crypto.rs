use std::fs::File;
use std::io::{self, Read};
use std::io::Write;

fn read_binary_file(file_path: &str) -> io::Result<Vec<u8>> {
    // 파일을 읽기 모드로 열기
    let mut file = File::open(file_path)?;

    // 파일의 크기를 구하여 버퍼를 할당
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let key = 123;
    let mut crypto_data = Vec::new();

    let file_path = "encode_image";
    match read_binary_file(file_path) {
        Ok(data) => {
            // 데이터 처리
            println!("읽은 데이터: {:?}", data);

            
            for buf in data{
                let mut crypto_buf = buf^key;
                crypto_data.push(crypto_buf);
            }
        }
        Err(e) => {
            println!("파일 읽기 오류: {}", e);
        }
    }

    println!("crypto data : {:?}",crypto_data);
    let mut crypto_file = File::create("decode_image.png").expect("파일 생성 실패...");
    crypto_file.write_all(&crypto_data).expect("쓰기 실패");

    println!("암호화 실행 완료");
}

