// Force rebuild to update ultra-vibrant icons
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../assets/icons/icon.ico");
        res.set("ProductName", "TechScript");
        res.set("FileDescription", "TechScript Language Runtime");
        res.set("LegalCopyright", "Copyright © 2026");
        res.compile().unwrap();
    }
}
