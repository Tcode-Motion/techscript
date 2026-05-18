// ── TechScript execution helpers ─────────────────────────────────────
use std::fs;
use std::path::Path;

use crate::compiler::Compiler;
use crate::error::{format_error, TechResult};
use crate::lexer::Lexer;
use crate::parser;
use crate::vm::VM;

pub const VERSION: &str = "1.0.6";

pub fn exit(code: i32) -> ! {
    #[cfg(target_os = "windows")]
    {
        extern "system" {
            fn GetConsoleProcessList(lpdwProcessList: *mut u32, dwProcessCount: u32) -> u32;
            fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> *mut std::ffi::c_void;
            fn QueryFullProcessImageNameW(
                hProcess: *mut std::ffi::c_void,
                dwFlags: u32,
                lpExeName: *mut u16,
                lpdwSize: *mut u32,
            ) -> i32;
            fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
            fn GetCurrentProcessId() -> u32;
        }

        const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;

        // Write debug info to a file in the workspace directory to see what processes are attached
        let mut debug_log = String::new();
        debug_log.push_str(&format!("Exit called with code {}\n", code));

        let mut process_list = [0u32; 64];
        let count = unsafe { GetConsoleProcessList(process_list.as_mut_ptr(), 64) };
        debug_log.push_str(&format!("GetConsoleProcessList count: {}\n", count));
        
        let mut shell_found = false;
        let my_pid = unsafe { GetCurrentProcessId() };
        debug_log.push_str(&format!("My PID: {}\n", my_pid));

        if count > 0 {
            for i in 0..(count as usize).min(64) {
                let pid = process_list[i];
                debug_log.push_str(&format!("Checking PID: {}\n", pid));
                if pid == 0 || pid == my_pid {
                    continue;
                }

                let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
                if !handle.is_null() {
                    let mut buf = [0u16; 512];
                    let mut size = buf.len() as u32;
                    let success = unsafe {
                        QueryFullProcessImageNameW(handle, 0, buf.as_mut_ptr(), &mut size)
                    };
                    unsafe { CloseHandle(handle); }

                    if success != 0 {
                        if let Ok(path) = String::from_utf16(&buf[..size as usize]) {
                            debug_log.push_str(&format!("  Process path: {}\n", path));
                            let path_lower = path.to_lowercase();
                            if path_lower.contains("cmd.exe")
                                || path_lower.contains("powershell.exe")
                                || path_lower.contains("pwsh.exe")
                                || path_lower.contains("bash.exe")
                                || path_lower.contains("nu.exe")
                            {
                                shell_found = true;
                                debug_log.push_str("  -> Match: SHELL FOUND!\n");
                            }
                        }
                    } else {
                        debug_log.push_str("  Failed to query image name\n");
                    }
                } else {
                    debug_log.push_str("  Failed to open process handle\n");
                }
            }
        }

        debug_log.push_str(&format!("shell_found decision: {}\n", shell_found));
        let _ = std::fs::write("debug_exit.txt", debug_log);

        // If no hosting command line shell was found, we were launched via double-click from Explorer
        if !shell_found {
            println!("\nPress Enter to exit...");
            let mut buf = String::new();
            let _ = std::io::stdin().read_line(&mut buf);
        }
    }
    std::process::exit(code);
}

pub fn compile_source(source: &str, filename: &str) -> TechResult<crate::value::Function> {
    let tokens = Lexer::new(source, filename).tokenize()?;
    let program = parser::Parser::new(tokens, filename).parse()?;
    Compiler::new().compile(&program)
}

pub fn run_source(source: &str, filename: &str) -> TechResult<()> {
    let function = compile_source(source, filename)?;
    let mut vm = VM::new();
    vm.run(function)
}

pub fn run_file(filepath: &str) -> TechResult<()> {
    if filepath.ends_with(".txbc") {
        return run_txbc_file(filepath);
    }
    let source = fs::read_to_string(filepath).map_err(|_| {
        crate::error::TechError::runtime(format!("File not found: {}", filepath))
    })?;
    run_source(&source, filepath)
}

pub fn run_txbc_file(filepath: &str) -> TechResult<()> {
    let data = fs::read(filepath).map_err(|_| {
        crate::error::TechError::runtime(format!("File not found: {}", filepath))
    })?;
    let function = crate::bytecode::deserialize_function(&data)
        .map_err(|msg| crate::error::TechError::runtime(msg))?;
    let mut vm = VM::new();
    vm.run(function)
}

pub fn check_file(filepath: &str) -> TechResult<()> {
    let source = fs::read_to_string(filepath).map_err(|_| {
        crate::error::TechError::runtime(format!("File not found: {}", filepath))
    })?;
    compile_source(&source, filepath).map(|_| ())
}

pub fn format_run_error(err: &crate::error::TechError, source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    format_error(err, &lines)
}

pub fn format_file_error(err: &crate::error::TechError, filepath: &str) -> String {
    let source = fs::read_to_string(filepath).unwrap_or_default();
    format_run_error(err, &source)
}

pub fn collect_example_files() -> Vec<String> {
    let mut files = Vec::new();
    for dir in ["runtime_examples", "examples"] {
        let path = Path::new("..").join(dir);
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) == Some("txs") {
                    if let Some(s) = p.to_str() {
                        files.push(s.replace('\\', "/"));
                    }
                }
            }
        }
    }
    files.sort();
    files
}
