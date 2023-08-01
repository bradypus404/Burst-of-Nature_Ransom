extern crate winreg;

use std::io;
// use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

fn add_startup_program(name: &str, path: &str) -> Result<(), io::Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
    let key =  hklm.open_subkey(subkey)?;
    key.set_value(name, &path)?;
    Ok(())
}

// fn remove_startup_program(name: &str) -> Result<(), io::Error> {
//     let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
//     let subkey = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
//     let key =  hklm.open_subkey(subkey)?;
//     key.delete_value(name)?;
//     Ok(())
// }

fn main() {
    let program_name = "startup";
    let program_path = r"C:\VScode\RUST\TestMalware\startup.exe";

    match add_startup_program(program_name, program_path) {
        Ok(()) => println!("Added to startup"),
        Err(e) => eprintln!("Error: {}", e),
    }
}