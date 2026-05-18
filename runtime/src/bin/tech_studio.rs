#![windows_subsystem = "windows"]

fn main() {
    // We launch the techscript visual IDE studio entirely natively,
    // ensuring no background console windows are spawned on Windows!
    techscript::studio::start_studio();
}
