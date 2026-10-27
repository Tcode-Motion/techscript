#![cfg(target_os = "android")]
// =============================================================================
// TechScript AppForge — Rust Native Android Runtime
// runtime/android_runtime/src/lib.rs
//
// APPLICATION ENTRY POINT — Pure Rust & TechScript (zero Java/Kotlin).
//
// Architecture:
//   AndroidManifest.xml → android.app.NativeActivity (Android OS system class)
//     → dlopen("libts_android.so")
//       → android_main() [this file, via android-activity crate]
//         → TechScript interpreter & state machine
//         → Hardware-accelerated OpenGL ES 2.0 / EGL Liquid Glass renderer
//         → Procedural 3D TechScript crystal with interactive touch-drag
//         → 5 primary application sections with real code and diagnostics
// =============================================================================

use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use std::time::Duration;

mod font8x8;
mod geometry3d;
mod glass_ui;
pub mod native_bridge;
mod renderer;
mod theme;
mod touch;
pub mod txs_bridge;
mod txs_runtime;
pub mod typography;
mod ui_model;

use renderer::NativeRenderer;
use theme::Theme;
use touch::TouchTracker;
use txs_runtime::TechScriptRuntime;
use ui_model::AppSpec;

// ─────────────────────────────────────────────────────────────────────────────
// Application Entry Point — called by NativeActivity, not by Java
// ─────────────────────────────────────────────────────────────────────────────

#[no_mangle]
fn android_main(app: AndroidApp) {
    // Initialize Android logcat logging
    android_log::init("TechScript").unwrap_or(());
    log::info!("========================================================");
    log::info!("  TechScript AppForge Premium Native Runtime v2.0");
    log::info!("  Entry Point: android_main() [Rust, Zero Java/Kotlin]");
    log::info!("========================================================");

    // Load TechScript app specification
    let spec = load_app_spec(&app);
    log::info!(
        "App: {} v{} by {}",
        spec.app.name,
        spec.app.version,
        spec.app.creator
    );

    let mut runtime = TechScriptRuntime::new(spec);
    runtime.on_create();

    // Restore persisted settings
    let settings = native_bridge::NativeBridge::load_settings(&app);
    runtime.set_theme_mode(&settings.theme_mode);
    if !settings.last_screen.is_empty() {
        runtime.navigate(&settings.last_screen);
    }
    runtime.auto_rotate = true;
    runtime.anim_speed = settings.anim_speed.max(0.8);

    // Load and execute real .txs scripts from Android APK assets
    runtime
        .bridge
        .load_and_execute_assets(|path| try_load_asset(&app, path));

    let mut renderer: Option<NativeRenderer> = None;
    let mut current_theme = Theme::resolve_system(&app);
    let mut touch_tracker = TouchTracker::default();

    let mut needs_redraw = true;
    let mut should_exit = false;

    // ─────────────────────────────────────────────────────────────────────
    // Main event loop — driven by Android OS native events
    // ─────────────────────────────────────────────────────────────────────
    loop {
        app.poll_events(Some(Duration::ZERO), |event| {
            match event {
                // ── Lifecycle ──────────────────────────────────────────
                PollEvent::Main(MainEvent::InitWindow { .. }) => {
                    log::info!(
                        "[TechScript] Window created — initializing EGL & GLES 2.0 renderer"
                    );
                    match NativeRenderer::new(&app, &current_theme) {
                        Ok(r) => {
                            log::info!(
                                "[TechScript] EGL & GLES 2.0 hardware renderer ready ({}x{})",
                                r.width,
                                r.height
                            );
                            renderer = Some(r);
                        }
                        Err(e) => {
                            log::error!("[TechScript] EGL initialization failed: {e}");
                        }
                    }
                    runtime.on_resume();
                    needs_redraw = true;
                }
                PollEvent::Main(MainEvent::TerminateWindow { .. }) => {
                    log::info!("[TechScript] Window destroyed — dropping EGL renderer");
                    runtime.on_pause();
                    renderer = None;
                }
                PollEvent::Main(MainEvent::Resume { .. }) => {
                    log::info!("[TechScript] onResume");
                    runtime.on_resume();
                    needs_redraw = true;
                }
                PollEvent::Main(MainEvent::Pause) => {
                    log::info!("[TechScript] onPause");
                    runtime.on_pause();
                }
                PollEvent::Main(MainEvent::Stop) => {
                    log::info!("[TechScript] onStop");
                }
                PollEvent::Main(MainEvent::Destroy) => {
                    log::info!("[TechScript] onDestroy — shutting down");
                    runtime.on_destroy();
                    should_exit = true;
                }
                PollEvent::Main(MainEvent::ConfigChanged { .. }) => {
                    log::info!("[TechScript] ConfigChanged (Orientation / Display Metrics change)");
                    current_theme = Theme::resolve_system(&app);
                    if let (Some(ref mut r), Some(win)) = (&mut renderer, app.native_window()) {
                        let nw = win.width();
                        let nh = win.height();
                        if nw > 0 && nh > 0 {
                            r.resize(nw, nh);
                        }
                    }
                    runtime.scroll_y = runtime.scroll_y.clamp(0.0, runtime.max_scroll_y);
                    needs_redraw = true;
                }
                PollEvent::Main(MainEvent::WindowResized { .. }) => {
                    log::info!("[TechScript] WindowResized");
                    if let (Some(ref mut r), Some(win)) = (&mut renderer, app.native_window()) {
                        let nw = win.width();
                        let nh = win.height();
                        if nw > 0 && nh > 0 {
                            r.resize(nw, nh);
                        }
                    }
                    runtime.scroll_y = runtime.scroll_y.clamp(0.0, runtime.max_scroll_y);
                    needs_redraw = true;
                }
                PollEvent::Main(MainEvent::SaveState { saver, .. }) => {
                    let settings = native_bridge::AppSettings {
                        theme_mode: runtime.theme_mode().to_string(),
                        last_screen: runtime.active_screen().to_string(),
                        auto_rotate: runtime.auto_rotate,
                        anim_speed: runtime.anim_speed,
                        scroll_y: runtime.scroll_y,
                    };
                    native_bridge::NativeBridge::save_settings(&app, &settings);
                    let state = runtime.active_screen().as_bytes().to_vec();
                    saver.store(&state);
                }

                // ── Input ──────────────────────────────────────────────
                PollEvent::Main(MainEvent::InputAvailable) => {
                    if let Ok(mut iter) = app.input_events_iter() {
                        loop {
                            if !iter.next(|event| {
                                if let Some(ref r) = renderer {
                                    let handled = touch::handle_input(
                                        event,
                                        &mut runtime,
                                        r,
                                        &app,
                                        &mut touch_tracker,
                                        &mut needs_redraw,
                                    );
                                    if handled {
                                        InputStatus::Handled
                                    } else {
                                        InputStatus::Unhandled
                                    }
                                } else {
                                    InputStatus::Unhandled
                                }
                            }) {
                                break;
                            }
                        }
                    }
                }

                _ => {}
            }
        });

        if should_exit {
            break;
        }

        // ── Frame render (continuous 60fps for 3D animation + UI) ───────
        if let Some(ref mut r) = renderer {
            // Apply theme switch if requested
            let active_theme = if runtime.is_dark() {
                Theme::dark()
            } else {
                Theme::light()
            };

            match r.render_frame(&mut runtime, &active_theme) {
                Ok(_) => {
                    needs_redraw = false;
                }
                Err(e) => {
                    log::error!("[TechScript] Render error: {e}");
                    renderer = None;
                }
            }
        } else {
            // Window not initialized or backgrounded — idle sleep to save battery
            std::thread::sleep(Duration::from_millis(16));
        }
    }

    log::info!("[TechScript] android_main() exiting cleanly");
}

fn load_app_spec(app: &AndroidApp) -> AppSpec {
    let json = try_load_asset(app, "app_spec.json").unwrap_or_default();
    if json.is_empty() {
        AppSpec::default_appforge()
    } else {
        serde_json::from_str::<AppSpec>(&json).unwrap_or_else(|_| AppSpec::default_appforge())
    }
}

fn try_load_asset(app: &AndroidApp, filename: &str) -> Option<String> {
    let asset_manager = app.asset_manager();
    let path = std::ffi::CString::new(filename).ok()?;
    let mut asset = asset_manager.open(&path)?;
    let bytes = asset.buffer().ok()?;
    String::from_utf8(bytes.to_vec()).ok()
}
