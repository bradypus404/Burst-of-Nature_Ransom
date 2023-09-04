//파일 시스템, 파일 입출력
use std::fs;
use std::io;

// 확장자 변경하는 함수
fn change_extension(dir_path: &str, new_extension: &str) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(dir_path)?;          // 폴더 내 파일을 읽어옴

    for entry in entries {
        let entry = entry?;             //결과를 생성하기 위해 프로세스의 반복
        let old_path = entry.path();    // 현재 파일의 경로를 가져옴

        if let Some(file_name) = old_path.file_name() {     // old_path에서 파일 이름을 추출, 있으면 실행
                                                            // 추출되면 Some(파일 이름) 형태로 반환
                                                            // 즉 반환 값이 Some이면 실행 
            let new_file_name = format!(
                "{}.{}",
                file_name.to_string_lossy().split('.').next().unwrap(), new_extension
                // file_name.to_string_lossy() 문자열로 변환
                // split() 파일 이름을 .을 기준으로 분리
                // next().unwrap() 분리된 파일 이름의 첫 부분을 가져옴
                // new_extension 새로운 확장자
            );

            let new_path = old_path.with_file_name(new_file_name);  // 기존 경로에 새로운 파일 이름을 적용, 새로운 파일 경로 생성
            fs::rename(old_path, new_path)?;        // rename 함수 사용하여 파일 이름 변경
        }
    }

    Ok(())
}

//폴더 안에 있는 파일 복사하는 함수
fn copy_dir(src: &str, dest: &str) -> io::Result<()> {
    if !fs::metadata(dest).is_ok() {    // dest경로의 디렉토리 정보를 확인하고 
        fs::create_dir_all(dest)?;      // 없으면 생성
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = format!("{}/{}", dest, entry_path.file_name().unwrap().to_str().unwrap());

        if entry_path.is_file() {                                       // entry_path가 파일이면 
            fs::copy(&entry_path, &dest_path)?;                         // 파일 복사
        } else if entry_path.is_dir() {                                 // entry_path가 폴더면
            copy_dir(&entry_path.to_str().unwrap(), &dest_path)?;       // 하위 폴더와 파일 복사
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "C:\\test";                                         //변경할 폴더
    let new_extension = "123";                                         //원하는 파일 확장자
    let dest_dir = "C:\\Users\\rkxsa\\OneDrive\\바탕 화면\\test";       //파일을 복사할 폴더 위치
    
    copy_dir(dir_path, dest_dir)?;
    change_extension(dir_path, new_extension)?; 

    println!("성공");
    Ok(())
}
