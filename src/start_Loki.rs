use std::io;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;

fn read_registry_values() -> io::Result<()> {
    println!("Reading some system info...");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")?;
    for (name, value) in cur_ver.enum_values().map(|x| x.unwrap()) {
        println!("{} = {:?}", name, value);
    }
    println!("------------------------------------------------------");

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion").join("Run");
    let (_key, _disp) = hkcu.create_subkey(&path)?;

    println!("{}", path.display());

    Ok(())
}

fn add_registry_value() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion").join("Run");
    let (_key, _disp) = hkcu.create_subkey(&path)?;

    println!("{}", path.display());

    _key.set_value("ThisIsLoki", &"C:\\Users\\brady\\VScode\\rust\\LokiLoki.exe")?;

    Ok(())
}

fn main() -> io::Result<()> {
    read_registry_values()?;
    add_registry_value()?;

    Ok(())
}