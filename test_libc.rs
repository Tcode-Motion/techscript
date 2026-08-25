extern crate libc;
fn main() {
    println!("{}", std::mem::size_of::<libc::jmp_buf>());
}
