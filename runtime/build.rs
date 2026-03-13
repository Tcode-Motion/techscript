use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(target_os = "windows") {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let icon_path = PathBuf::from(manifest_dir).join("../assets/icons/icon.ico");
        
        let mut res = winres::WindowsResource::new();
        res.set_toolkit_path(r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64");
        res.set_icon(icon_path.to_str().unwrap());
        res.compile().unwrap();
    }
}
