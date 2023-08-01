use std::fs;

fn main(){
    //파일 구조 가져오기
    if let Ok(entries) = fs::read_dir(".") {    // 함수를 사용해 부모 디렉토리 경로에서 자식 디렉토리명과 파일명을 구함
        for entry in entries{
            if let Ok(entry) = entry {
                let path = entry.path();
                println!("{}", path.display());
            }
        }
    }
    else {
        println!("위치 불러오기 실패");
    }
} 