use std::process::Command;

fn main(){
    //바탕화면 이미지 파일 경로
    let wallpaper_path = "C:\\test.jpg";

    //바탕화면 변경 명령어 실행
    let output = Command::new("powershell").arg("-Command").arg(format!(r#"Set-itemProperty-Path "HKCU:\Control Panel\Desktop"-Name Wallpaer-Value"{}""#,wallpaper_path)).output().expect("Failed to execute command");

    if output.status.success(){
        println!("바탕화면이 변경되었습니다.");
    }
    else {
        println!("바탕화면 변경에 실패했습니다.");
    }
}