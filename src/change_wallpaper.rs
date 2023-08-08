use std::io;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;

fn main() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("\\Control Panel\\Desktop");
    let (_key, _disp) = hkcu.create_subkey(&path)?;

    println!("{}", path.display());

    _key.set_value("WallpPaper", &"C:\\Windows\\Web\\Wallpaper\\Windows\\img19.jpg")?;

    Ok(())
}