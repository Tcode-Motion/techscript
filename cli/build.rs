fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../assets/icons/app-icon.ico");
        res.compile().unwrap();
    }
}
