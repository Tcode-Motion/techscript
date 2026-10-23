// TechScript AppForge — Android Native Build Configuration
// This build file produces an APK that contains ONLY:
//   - libts_android.so  (Rust, compiled via cargo + Android NDK)
//   - assets/           (app_spec.json, .txs specs — compiled by tsc CLI)
//   - res/              (icon, strings, colors)
//   - AndroidManifest.xml (references android.app.NativeActivity — OS system class)
//
// Zero Java/Kotlin source files in the application.
// Zero Java dependencies for application logic.

plugins {
    id("com.android.application")
}

android {
    namespace  = "com.techscript.appforge"
    compileSdk = 35

    defaultConfig {
        applicationId   = "com.techscript.appforge"
        minSdk          = 26
        targetSdk       = 35
        versionCode     = 1
        versionName     = "2.0.0"

        // Native ABIs to package — arm64-v8a covers virtually all modern Android devices
        ndk {
            abiFilters += listOf("arm64-v8a", "x86_64")
        }

        // NativeActivity uses android:value="android_main" to find the
        // exported entry point in libts_android.so
        manifestPlaceholders["nativeLibraryName"] = "ts_android"
    }

    buildTypes {
        release {
            isMinifyEnabled   = false
            isShrinkResources = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {
            isDebuggable = true
        }
    }

    sourceSets {
        getByName("main") {
            // No Java sources — the java srcDir is emptied
            java.setSrcDirs(emptyList<String>())
            // Assets staged by tsc CLI from .txs compiled specs
            assets.srcDirs("src/main/assets")
            // Pre-built native .so files staged by tsc CLI from cargo build output
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }

    // No Java compilation needed
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    packaging {
        jniLibs {
            // Keep .so files uncompressed for faster loading by NativeActivity
            keepDebugSymbols += "**/*.so"
            useLegacyPackaging = false
        }
    }
}

// No Java/Kotlin dependencies — the app is implemented entirely in Rust.
// Only the Android platform and build-system libraries are referenced.
dependencies {
    // No application dependencies — NativeActivity is a system class.
    // libts_android.so is staged into jniLibs by the tsc build pipeline.
}
