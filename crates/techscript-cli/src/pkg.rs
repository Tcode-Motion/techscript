use std::fs;
use std::path::Path;
use std::process::Command;
use techscript_core::ansi::Color;

pub fn install_package(name: &str, url: Option<&str>) {
    let url_str = match url {
        Some(u) => u.to_string(),
        None => format!("https://github.com/techscript-pkg/{}.git", name), // Default registry convention
    };

    println!("{} Installing package '{}' from {}...", Color::bold_cyan("📦"), name, url_str);

    // Ensure modules directory exists
    let modules_dir = Path::new(".techscript-modules");
    if !modules_dir.exists() {
        if let Err(e) = fs::create_dir_all(modules_dir) {
            eprintln!("{} Failed to create module directory: {}", Color::bold_red("error:"), e);
            return;
        }
    }

    let target_dir = modules_dir.join(name);
    
    // If it exists, try to update it via git pull
    if target_dir.exists() {
        println!("{} Module '{}' already exists. Pulling latest changes...", Color::dim("▸"), name);
        let status = Command::new("git")
            .arg("-C")
            .arg(&target_dir)
            .arg("pull")
            .status();

        match status {
            Ok(s) => {
                if s.success() {
                    println!("{} Successfully updated '{}'", Color::bold_green("✓"), name);
                } else {
                    eprintln!("{} Failed to update '{}'", Color::bold_yellow("⚠"), name);
                }
            }
            Err(e) => eprintln!("{} Git failure: {}", Color::bold_red("error:"), e),
        }
    } else {
        // Clone the repo
        println!("{} Cloning module '{}'...", Color::dim("▸"), name);
        let status = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg(&url_str)
            .arg(&target_dir)
            .status();

        match status {
            Ok(s) => {
                if s.success() {
                    println!("{} Successfully installed '{}'", Color::bold_green("✓"), name);
                } else {
                    eprintln!("{} Failed to clone '{}' from {}", Color::bold_red("error:"), name, url_str);
                    return;
                }
            }
            Err(e) => {
                eprintln!("{} Git failure: {}", Color::bold_red("error:"), e);
                return;
            }
        }
    }

    // Update tech.toml
    update_tech_toml(name, &url_str);
}

fn update_tech_toml(name: &str, url: &str) {
    let toml_path = Path::new("tech.toml");
    if !toml_path.exists() {
        return; // No tech.toml, don't update
    }

    let content = match fs::read_to_string(toml_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let dep_line = format!("{} = \"{}\"", name, url);
    
    if content.contains(&format!("{} =", name)) || content.contains(&format!("{} = ", name)) {
        println!("{} '{}' is already in tech.toml", Color::dim("▸"), name);
        return;
    }

    let mut new_content = String::new();
    let mut in_dependencies = false;
    let mut added = false;

    for line in content.lines() {
        new_content.push_str(line);
        new_content.push('\n');

        if line.trim() == "[dependencies]" {
            in_dependencies = true;
            new_content.push_str(&dep_line);
            new_content.push('\n');
            added = true;
        } else if in_dependencies && line.trim().starts_with('[') {
            in_dependencies = false; // Next section
        }
    }

    // If no [dependencies] section was found, append it
    if !added {
        if !new_content.ends_with('\n') {
            new_content.push('\n');
        }
        new_content.push_str("\n[dependencies]\n");
        new_content.push_str(&dep_line);
        new_content.push('\n');
    }

    if let Err(e) = fs::write(toml_path, new_content) {
        eprintln!("{} Failed to update tech.toml: {}", Color::bold_yellow("⚠"), e);
    } else {
        println!("{} Added '{}' to tech.toml [dependencies]", Color::bold_green("✓"), name);
    }
}
