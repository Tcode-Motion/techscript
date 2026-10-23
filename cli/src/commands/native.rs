//! # `tsc native add` Native Language Scaffolding
//!
//! Adds native platform language modules (Kotlin, Swift, Rust, C++, C, Java)
//! with proper build definitions, headers, and JNI/FFI conventions.

use crate::exit_code::ExitCode;
use crate::NativeCommands;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub fn execute(cmd: &NativeCommands) -> ExitCode {
    match cmd {
        NativeCommands::Add { language } => add_language(language),
    }
}

fn add_language(lang: &str) -> ExitCode {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let native_dir = current_dir.join("native");
    let l = lang.to_lowercase();

    match l.as_str() {
        "kotlin" => {
            let k_dir = native_dir.join("kotlin");
            let _ = fs::create_dir_all(k_dir.join("src/main/kotlin/com/techscript/nativeplugins"));
            let kt_code = r#"package com.techscript.nativeplugins

object CustomPlugin {
    fun doAction(args: List<Any?>): Any? {
        println("CustomPlugin invoked from TechScript!")
        return mapOf("status" to "ok", "message" to "Executed via Kotlin")
    }
}
"#;
            let _ = fs::write(
                k_dir.join("src/main/kotlin/com/techscript/nativeplugins/CustomPlugin.kt"),
                kt_code,
            );
            println!(
                "{} Added Kotlin native module at: {}",
                "✓".green().bold(),
                k_dir.display().to_string().cyan()
            );
            ExitCode::Success
        }
        "java" => {
            let j_dir = native_dir.join("java");
            let _ = fs::create_dir_all(j_dir.join("src/main/java/com/techscript/nativeplugins"));
            let java_code = r#"package com.techscript.nativeplugins;

import java.util.Map;
import java.util.HashMap;

public class CustomJavaPlugin {
    public static Map<String, Object> doAction() {
        Map<String, Object> res = new HashMap<>();
        res.put("status", "ok");
        return res;
    }
}
"#;
            let _ = fs::write(
                j_dir.join("src/main/java/com/techscript/nativeplugins/CustomJavaPlugin.java"),
                java_code,
            );
            println!(
                "{} Added Java native module at: {}",
                "✓".green().bold(),
                j_dir.display().to_string().cyan()
            );
            ExitCode::Success
        }
        "swift" => {
            let s_dir = native_dir.join("swift");
            let _ = fs::create_dir_all(s_dir.join("Sources/CustomPlugin"));
            let pkg_swift = r#"// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CustomPlugin",
    products: [
        .library(name: "CustomPlugin", type: .dynamic, targets: ["CustomPlugin"])
    ],
    targets: [
        .target(name: "CustomPlugin")
    ]
)
"#;
            let _ = fs::write(s_dir.join("Package.swift"), pkg_swift);
            let swift_code = r#"import Foundation

@_cdecl("custom_plugin_action")
public func customPluginAction() -> Int32 {
    print("Swift plugin called from TechScript!")
    return 42
}
"#;
            let _ = fs::write(s_dir.join("Sources/CustomPlugin/Plugin.swift"), swift_code);
            println!(
                "{} Added Swift native module at: {}",
                "✓".green().bold(),
                s_dir.display().to_string().cyan()
            );
            ExitCode::Success
        }
        "rust" => {
            let r_dir = native_dir.join("rust");
            let _ = fs::create_dir_all(r_dir.join("src"));
            let cargo_toml = r#"[package]
name = "techscript_custom_native"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "staticlib"]

[dependencies]
"#;
            let _ = fs::write(r_dir.join("Cargo.toml"), cargo_toml);
            let lib_rs = r#"#[no_mangle]
pub extern "C" fn techscript_plugin_call(arg: i32) -> i32 {
    println!("Rust native extension called with: {}", arg);
    arg * 2
}
"#;
            let _ = fs::write(r_dir.join("src/lib.rs"), lib_rs);
            println!(
                "{} Added Rust native module at: {}",
                "✓".green().bold(),
                r_dir.display().to_string().cyan()
            );
            ExitCode::Success
        }
        "cpp" => {
            let cpp_dir = native_dir.join("cpp");
            let _ = fs::create_dir_all(cpp_dir.join("src"));
            let cmake = r#"cmake_minimum_required(VERSION 3.22)
project(techscript_native_cpp CXX)

add_library(custom_plugin SHARED src/plugin.cpp)
"#;
            let _ = fs::write(cpp_dir.join("CMakeLists.txt"), cmake);
            let cpp_code = r#"#include <iostream>

extern "C" {
    int plugin_compute(int a, int b) {
        std::cout << "C++ native module executing: " << a << " + " << b << std::endl;
        return a + b;
    }
}
"#;
            let _ = fs::write(cpp_dir.join("src/plugin.cpp"), cpp_code);
            println!(
                "{} Added C++ native module at: {}",
                "✓".green().bold(),
                cpp_dir.display().to_string().cyan()
            );
            ExitCode::Success
        }
        "c" => {
            let c_dir = native_dir.join("c");
            let _ = fs::create_dir_all(c_dir.join("src"));
            let cmake = r#"cmake_minimum_required(VERSION 3.22)
project(techscript_native_c C)

add_library(custom_c_plugin SHARED src/plugin.c)
"#;
            let _ = fs::write(c_dir.join("CMakeLists.txt"), cmake);
            let c_code = r#"#include <stdio.h>

int c_plugin_entry(int x) {
    printf("C native plugin called with value: %d\n", x);
    return x + 100;
}
"#;
            let _ = fs::write(c_dir.join("src/plugin.c"), c_code);
            println!(
                "{} Added C native module at: {}",
                "✓".green().bold(),
                c_dir.display().to_string().cyan()
            );
            ExitCode::Success
        }
        _ => {
            eprintln!(
                "{} Unsupported native language '{}'. Supported languages: kotlin, swift, rust, cpp, c, java",
                "error:".red().bold(),
                lang
            );
            ExitCode::Failure
        }
    }
}
