fn main() {
    println!("cargo:rustc-link-lib=dylib=EGL");
    println!("cargo:rustc-link-lib=dylib=GLESv2");
    println!("cargo:rustc-link-lib=dylib=android");
    println!("cargo:rustc-link-lib=dylib=log");
}
