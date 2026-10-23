//! # `tsc app create` Project Generator
//!
//! Scaffolds a complete, self-contained TechScript Appforge application
//! with TechScript source code, Android host project, configuration, assets, and tests.

use crate::commands::app::project::{
    AndroidConfig, AppMeta, AppTheme, AppforgeManifest, WindowsConfig,
};
use crate::exit_code::ExitCode;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn execute(name: &str, template: Option<&str>, verbose: bool) -> ExitCode {
    let project_dir = PathBuf::from(name);
    if project_dir.exists() {
        eprintln!(
            "{} Directory '{}' already exists. Choose a different name or remove the existing directory.",
            "error:".red().bold(),
            name
        );
        return ExitCode::Failure;
    }

    println!(
        "{} Creating Appforge application: '{}'...",
        "⚙️ ".bold(),
        name.cyan().bold()
    );

    match scaffold_project(&project_dir, name, template, verbose) {
        Ok(_) => {
            println!(
                "{}",
                "\n========================================================"
                    .green()
                    .bold()
            );
            println!(
                "  ✨ {} created successfully!",
                format!("Appforge Application '{}'", name).bold()
            );
            println!(
                "{}",
                "========================================================"
                    .green()
                    .bold()
            );
            println!("\nNext steps:");
            println!("  cd {}", name.cyan());
            println!("  tsc app run android          # Run on connected Android device");
            println!("  tsc app build android        # Build debug APK");
            println!("  tsc app build android --aab  # Build Google Play App Bundle");
            println!("  tsc app run windows          # Run on Windows");
            println!("  tsc app doctor               # Inspect mobile & desktop toolchains\n");
            ExitCode::Success
        }
        Err(e) => {
            eprintln!(
                "{} Failed to scaffold project: {}",
                "error:".red().bold(),
                e
            );
            ExitCode::Failure
        }
    }
}

pub fn scaffold_project(
    dir: &Path,
    name: &str,
    _template: Option<&str>,
    verbose: bool,
) -> Result<(), anyhow::Error> {
    fs::create_dir_all(dir)?;

    let clean_name = name
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    let pkg_id = format!(
        "com.techscript.{}",
        if clean_name.is_empty() {
            "app"
        } else {
            &clean_name
        }
    );

    // 1. appforge.toml
    let manifest = AppforgeManifest {
        app: AppMeta {
            id: pkg_id.clone(),
            name: name.to_string(),
            version: "1.0.0".to_string(),
            version_code: 1,
            entry: "src/main.txs".to_string(),
            description: Some(format!(
                "Cross-platform application built with TechScript Appforge."
            )),
        },
        theme: AppTheme::default(),
        android: AndroidConfig {
            min_sdk: 26,
            target_sdk: 34,
            compile_sdk: 34,
            package: pkg_id.clone(),
            orientation: "portrait".to_string(),
            permissions: vec!["android.permission.INTERNET".to_string()],
        },
        windows: WindowsConfig {
            executable_name: format!("{}.exe", name),
            company: "TechScript".to_string(),
        },
    };
    fs::write(dir.join("appforge.toml"), manifest.to_toml_string()?)?;

    // 2. techscript.toml
    let tech_toml = format!(
        r#"[package]
name = "{}"
version = "1.0.0"
entry = "src/main.txs"
framework = "appforge"

[capabilities]
permissions = ["Network", "FileSystem", "Display"]
"#,
        name
    );
    fs::write(dir.join("techscript.toml"), tech_toml)?;

    // 3. .gitignore
    let gitignore = r#"# TechScript Build Artifacts
build/
dist/
.appforge/
.tsc-cache/
logs/
reports/

# Android Build Outputs
android/.gradle/
android/app/build/
android/build/
android/local.properties
*.apk
*.aab

# Native libraries
*.so
*.dll
*.dylib
"#;
    fs::write(dir.join(".gitignore"), gitignore)?;

    // 4. .editorconfig
    let editorconfig = r#"root = true

[*]
indent_style = space
indent_size = 4
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true
"#;
    fs::write(dir.join(".editorconfig"), editorconfig)?;

    // 5. README.md
    let readme = format!(
        r#"# {}

Cross-platform mobile and desktop application created with **TechScript Appforge**.

## Getting Started

### Run on Android
```bash
tsc app run android
```

### Build Android APK / AAB
```bash
tsc app build android        # Debug APK
tsc app build android --release
tsc app build android --aab  # Android App Bundle for Google Play
```

### Run on Windows
```bash
tsc app run windows
```

### Run Tests
```bash
tsc app test
```
"#,
        name
    );
    fs::write(dir.join("README.md"), readme)?;

    // 6. Source Tree: src/
    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(src_dir.join("screens"))?;
    fs::create_dir_all(src_dir.join("widgets"))?;
    fs::create_dir_all(src_dir.join("state"))?;

    let main_txs = r#"# TechScript Appforge Entry Point

do main()
    say "Starting TechScript Appforge Application..."
    say "Platform Engine: Appforge v2.0"
end
"#;
    fs::write(src_dir.join("main.txs"), main_txs)?;

    let app_txs = format!(
        r##"# TechScript Appforge UI Specification
app "{}":
    theme:
        mode is "dark"
        primary is "#4f46e5"
        background is "#0f172a"
        text is "#f8fafc"

    screen Home:
        navbar:
            title is "{}"

        column:
            padding is 20
            spacing is 16

            card:
                heading "Welcome to Appforge"
                text "Build Android APKs and cross-platform native apps with TechScript."

            card:
                heading "Real Device Integration"
                text "Hardware-accelerated UI with full native Android capability."

            button "Click Me" called ActionBtn:
                on_click:
                    say "Button clicked in TechScript Appforge!"
"##,
        name, name
    );
    fs::write(src_dir.join("app.txs"), app_txs)?;

    let home_screen_txs = r#"# Home Screen Component
screen HomeScreen:
    column:
        padding is 16
        text "Appforge Home Screen"
"#;
    fs::write(
        src_dir.join("screens").join("home_screen.txs"),
        home_screen_txs,
    )?;

    // 7. Assets
    let assets_dir = dir.join("assets");
    fs::create_dir_all(&assets_dir)?;
    fs::write(
        assets_dir.join("app_info.txt"),
        format!("TechScript Appforge Asset Directory for {}", name),
    )?;

    // 8. Test
    let test_dir = dir.join("test");
    fs::create_dir_all(&test_dir)?;
    let app_test_txs = r#"# Appforge Application Unit Tests
do test_appforge_initialization()
    say "Appforge initialization test passed."
end

test_appforge_initialization()
"#;
    fs::write(test_dir.join("app_test.txs"), app_test_txs)?;

    // 9. Android Host Scaffolding
    let android_dir = dir.join("android");
    scaffold_android_host(&android_dir, name, &pkg_id, verbose)?;

    Ok(())
}

fn scaffold_android_host(
    android_dir: &Path,
    app_name: &str,
    pkg_id: &str,
    verbose: bool,
) -> Result<(), anyhow::Error> {
    fs::create_dir_all(android_dir)?;

    // android/build.gradle.kts (root)
    let root_build_gradle = r#"// TechScript Appforge Root Gradle Configuration
plugins {
    id("com.android.application") version "8.6.0" apply false
}
"#;
    fs::write(android_dir.join("build.gradle.kts"), root_build_gradle)?;

    // android/settings.gradle.kts
    let clean_project_name = app_name.replace(' ', "-").to_lowercase();
    let settings_gradle = format!(
        r#"// TechScript Appforge Settings Gradle
pluginManagement {{
    repositories {{
        google()
        mavenCentral()
        gradlePluginPortal()
    }}
}}
dependencyResolutionManagement {{
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {{
        google()
        mavenCentral()
    }}
}}
rootProject.name = "{}"
include(":app")
"#,
        clean_project_name
    );
    fs::write(android_dir.join("settings.gradle.kts"), settings_gradle)?;

    // android/gradle.properties
    let gradle_properties = r#"org.gradle.jvmargs=-Xmx2048m -Dfile.encoding=UTF-8
android.useAndroidX=true
android.nonTransitiveRClass=true
"#;
    fs::write(android_dir.join("gradle.properties"), gradle_properties)?;

    // android/app/
    let app_dir = android_dir.join("app");
    fs::create_dir_all(&app_dir)?;

    // android/app/build.gradle.kts
    let app_build_gradle = format!(
        r#"plugins {{
    id("com.android.application")
}}

android {{
    namespace = "{}"
    compileSdk = 34

    defaultConfig {{
        applicationId = "{}"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }}

    buildTypes {{
        release {{
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }}
        debug {{
            isDebuggable = true
        }}
    }}
    compileOptions {{
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }}
    sourceSets {{
        getByName("main") {{
            java.srcDirs("build/generated/source/techscript")
            assets.srcDirs("src/main/assets")
        }}
    }}
}}

dependencies {{
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("com.google.android.material:material:1.12.0")
}}
"#,
        pkg_id, pkg_id
    );
    fs::write(app_dir.join("build.gradle.kts"), app_build_gradle)?;

    // Proguard rules
    fs::write(
        app_dir.join("proguard-rules.pro"),
        "# Appforge Proguard Rules\n-keep class com.techscript.** { *; }\n",
    )?;

    // Manifest
    let main_src_dir = app_dir.join("src").join("main");
    fs::create_dir_all(&main_src_dir)?;

    let manifest_xml = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">

    <uses-permission android:name="android.permission.INTERNET" />

    <application
        android:allowBackup="true"
        android:icon="@mipmap/ic_launcher"
        android:label="@string/app_name"
        android:roundIcon="@mipmap/ic_launcher"
        android:supportsRtl="true"
        android:theme="@style/Theme.Appforge"
        android:hardwareAccelerated="true">
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:configChanges="orientation|screenSize|keyboardHidden">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
"#
    );
    fs::write(main_src_dir.join("AndroidManifest.xml"), manifest_xml)?;

    // TechScript Native Android Activity
    let pkg_path = pkg_id.replace('.', "/");
    let ts_pkg_dir = main_src_dir.join("techscript").join(&pkg_path);
    fs::create_dir_all(&ts_pkg_dir)?;

    let main_activity_txs = format!(
        r#"# ==============================================================================
# TechScript AppForge: Native Android Application Activity
# Authored in 100% Pure TechScript (.txs)
# Application ID: {0}
# ==============================================================================

model MainActivity extends Activity:
    package_id = "{0}"
    title = "{1}"
    active_tab = "home"
    theme_mode = "system"
    is_dark = true

    do onCreate(saved_instance_state)
        say "[TechScript Activity] Starting {1}..."
        self.theme_mode = "system"
        self.is_dark = true
        self.mount_ui()
    end

    do onResume()
        say "[TechScript Activity] onResume: Interactive focus gained."
    end

    do onPause()
        say "[TechScript Activity] onPause: State preserved."
    end

    do onDestroy()
        say "[TechScript Activity] onDestroy: Cleaned up native resources."
    end

    do mount_ui()
        say "[TechScript Activity] Mounting native UI widgets..."
    end

    do switch_tab(tab_id)
        self.active_tab = tab_id
        say $"[TechScript Activity] Switched to tab: {{tab_id}}"
    end
end
"#,
        pkg_id, app_name
    );
    fs::write(ts_pkg_dir.join("MainActivity.txs"), main_activity_txs)?;

    // Res/
    let res_dir = main_src_dir.join("res");
    let values_dir = res_dir.join("values");
    fs::create_dir_all(&values_dir)?;

    fs::write(
        values_dir.join("strings.xml"),
        format!(
            r#"<resources>
    <string name="app_name">{}</string>
</resources>
"#,
            app_name
        ),
    )?;

    fs::write(
        values_dir.join("colors.xml"),
        r#"<resources>
    <color name="primary">#4f46e5</color>
    <color name="background">#0f172a</color>
    <color name="text">#f8fafc</color>
</resources>
"#,
    )?;

    fs::write(
        values_dir.join("themes.xml"),
        r#"<resources>
    <style name="Theme.Appforge" parent="Theme.MaterialComponents.DayNight.NoActionBar">
        <item name="colorPrimary">@color/primary</item>
        <item name="android:windowBackground">@color/background</item>
    </style>
</resources>
"#,
    )?;

    // Launcher icon mipmap
    let mipmap_dir = res_dir.join("mipmap-anydpi-v26");
    fs::create_dir_all(&mipmap_dir)?;
    let ic_launcher = r#"<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@color/primary" />
    <foreground android:drawable="@drawable/ic_launcher_foreground" />
</adaptive-icon>
"#;
    fs::write(mipmap_dir.join("ic_launcher.xml"), ic_launcher)?;

    let drawable_dir = res_dir.join("drawable");
    fs::create_dir_all(&drawable_dir)?;
    let ic_foreground = r##"<vector xmlns:android="http://schemas.android.com/apk/res/android"
    android:width="108dp"
    android:height="108dp"
    android:viewportWidth="108"
    android:viewportHeight="108">
    <path
        android:fillColor="#ffffff"
        android:pathData="M31,31h46v46h-46z"/>
</vector>
"##;
    fs::write(
        drawable_dir.join("ic_launcher_foreground.xml"),
        ic_foreground,
    )?;

    // Assets dir
    let assets_host = main_src_dir.join("assets");
    fs::create_dir_all(&assets_host)?;

    // Generate Gradle wrapper
    if let Ok(gradle_cmd) = which_gradle() {
        if verbose {
            println!("Generating Gradle wrapper with '{}'...", gradle_cmd);
        }
        let _ = Command::new(&gradle_cmd)
            .arg("wrapper")
            .arg("--gradle-version")
            .arg("8.7")
            .current_dir(android_dir)
            .output();
    }

    Ok(())
}

fn which_gradle() -> Result<String, anyhow::Error> {
    if let Ok(gh) = std::env::var("GRADLE_HOME") {
        let bat = Path::new(&gh).join("bin").join("gradle.bat");
        if bat.exists() {
            return Ok(bat.to_string_lossy().to_string());
        }
    }
    let candidates = [
        "C:\\Gradle\\gradle-8.7\\bin\\gradle.bat",
        "C:\\Gradle\\bin\\gradle.bat",
        "C:\\Program Files\\Gradle\\bin\\gradle.bat",
    ];
    for c in &candidates {
        if Path::new(c).exists() {
            return Ok(c.to_string());
        }
    }
    if let Ok(output) = Command::new("where").arg("gradle").output() {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if let Some(first_line) = s.lines().next() {
                return Ok(first_line.trim().to_string());
            }
        }
    }
    Ok("gradle".to_string())
}
