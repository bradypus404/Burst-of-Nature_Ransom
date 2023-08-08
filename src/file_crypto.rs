use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::io::Write;
use std::path::PathBuf;


//* 디렉토리에서 파일 명 가져오는 함수 *//
fn call_filePath() -> Vec<PathBuf>{
    //파일 구조 가져오기
    let dir_path = "C:\\Users\\guswj\\OneDrive\\바탕 화면\\rust\\crypto_test"; //암호화 시킬 디렉토리 경로
    let mut dir_path_list = Vec::new(); //파일 경로들 담을 리스트 생성

    if let Ok(entries) = fs::read_dir(dir_path) {    // 함수를 사용해 부모 디렉토리 경로에서 자식 디렉토리명과 파일명을 구함
        for entry in entries{
            if let Ok(entry) = entry {
                let path = entry.path();
                println!("{}", path.display()); //check
                dir_path_list.push(path);
            }
        }
    }
    else {
        println!("위치 불러오기 실패");
    }
    println!("{:?}", dir_path_list); //check
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
    let key = 123;//암호화 키 설정
    
    let file_path = call_filePath();//파일 경로 불러오기

    for one_file_path in file_path{ //리스트에서 파일경로 하나씩 꺼내오기
        if let Some(put_file_path) = one_file_path.to_str() { //PathBuf타입을 str타입으로 변환
            let mut crypto_data = Vec::new();// 암호화한 데이터 담을 빈 리스트 생성
            println!("Success"); //check

            match read_binary_file(put_file_path) {
                Ok(data) => {
                    // 데이터 처리
                    println!("읽은 데이터: {:?}", data); //check
        
                    for buf in data{
                        let mut crypto_buf = buf^key; //xor암호화
                        crypto_data.push(crypto_buf);
                    }
                }
                Err(e) => {
                    println!("파일 읽기 오류: {}", e);
                }
            }

            println!("crypto data : {:?}",crypto_data); //check

            //파일 생성하여 원본파일을 암호화된 파일로 덮어쓰기
            let mut crypto_file = File::create(put_file_path).expect("파일 생성 실패...");
            crypto_file.write_all(&crypto_data).expect("쓰기 실패");

            println!("암호화 실행 완료");

        } else {
            println!("failed");
        }
    }
}

fn main(){
    encode_file();
}