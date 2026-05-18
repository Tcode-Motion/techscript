// ── Environment diagnostics for `tech doctor` ────────────────────────
use std::env;
use std::path::Path;

use crate::run::VERSION;

pub struct DoctorReport {
    pub checks: Vec<(String, bool, String)>,
}

impl DoctorReport {
    pub fn run() -> Self {
        let mut checks = Vec::new();

        checks.push((
            "TechScript version".into(),
            true,
            format!("v{}", VERSION),
        ));

        let cwd = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "?".into());
        checks.push(("Working directory".into(), true, cwd));

        let path = env::var("PATH").unwrap_or_default();
        let on_path = env::current_exe()
            .ok()
            .map(|exe| {
                let exe_str = exe.to_string_lossy().to_string();
                path.split(';').chain(path.split(':')).any(|p| {
                    Path::new(p).join("tech.exe").exists()
                        || Path::new(p).join("tech").exists()
                        || exe_str.contains(p)
                })
            })
            .unwrap_or(true);
        checks.push((
            "Binary on PATH".into(),
            on_path,
            if on_path { "OK".into() } else { "Add tech to PATH".into() },
        ));

        let write_ok = env::temp_dir().join("techscript_doctor_test");
        let can_write = fs_write_test(&write_ok);
        checks.push((
            "Write permissions".into(),
            can_write,
            if can_write { "OK".into() } else { "Cannot write temp files".into() },
        ));

        checks.push((
            "Module cache dir".into(),
            true,
            ".tech/cache/ (created on first pkg install)".into(),
        ));

        DoctorReport { checks }
    }

    pub fn all_ok(&self) -> bool {
        self.checks.iter().all(|(_, ok, _)| *ok)
    }

    pub fn format(&self) -> String {
        let mut out = String::from("TechScript Doctor\n\n");
        for (name, ok, detail) in &self.checks {
            let mark = if *ok { "✓" } else { "✗" };
            out.push_str(&format!("  {} {} — {}\n", mark, name, detail));
        }
        out
    }
}

fn fs_write_test(path: &Path) -> bool {
    if std::fs::write(path, "ok").is_ok() {
        let _ = std::fs::remove_file(path);
        true
    } else {
        false
    }
}
