use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../../assets/techscript.ico");
        res.set("ProductName", "TechScript");
        res.set("FileDescription", "TechScript Runtime Engine");
        if let Err(e) = res.compile() {
            eprintln!("Failed to embed Windows icon: {}", e);
        }
    }
}
