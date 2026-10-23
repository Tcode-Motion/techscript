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

use android_activity::{
    AndroidApp, InputStatus, MainEvent, PollEvent,
};
use std::time::{Duration, Instant};

mod font8x8;
pub mod typography;
mod geometry3d;
mod glass_ui;
mod renderer;
mod theme;
mod touch;
mod txs_runtime;
mod ui_model;

use ui_model::AppSpec;
use renderer::NativeRenderer;
use txs_runtime::TechScriptRuntime;
use theme::Theme;
use touch::TouchTracker;

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
    log::info!("App: {} v{} by {}", spec.app.name, spec.app.version, spec.app.creator);

    let mut runtime = TechScriptRuntime::new(spec);
    runtime.on_create();

    let mut renderer: Option<NativeRenderer> = None;
    let mut current_theme = Theme::resolve_system(&app);
    let mut touch_tracker = TouchTracker::default();

    let target_frame_ns = 1_000_000_000u64 / 60;
    let mut last_frame = Instant::now();
    let mut needs_redraw = true;
    let mut should_exit = false;

    // ─────────────────────────────────────────────────────────────────────
    // Main event loop — driven by Android OS native events
    // ─────────────────────────────────────────────────────────────────────
    loop {
        app.poll_events(Some(Duration::from_millis(8)), |event| {
            match event {
                // ── Lifecycle ──────────────────────────────────────────
                PollEvent::Main(MainEvent::InitWindow { .. }) => {
                    log::info!("[TechScript] Window created — initializing EGL & GLES 2.0 renderer");
                    match NativeRenderer::new(&app, &current_theme) {
                        Ok(r) => {
                            log::info!("[TechScript] EGL & GLES 2.0 hardware renderer ready ({}x{})", r.width, r.height);
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
                    current_theme = Theme::resolve_system(&app);
                    needs_redraw = true;
                }
                PollEvent::Main(MainEvent::SaveState { saver, .. }) => {
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
                                        &mut touch_tracker,
                                        &mut needs_redraw,
                                    );
                                    if handled { InputStatus::Handled } else { InputStatus::Unhandled }
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
            let active_theme = if runtime.is_dark() { Theme::dark() } else { Theme::light() };

            match r.render_frame(&mut runtime, &active_theme) {
                Ok(_) => {
                    needs_redraw = false;
                }
                Err(e) => {
                    log::error!("[TechScript] Render error: {e}");
                    renderer = None;
                }
            }
        }

        // Frame pacing — target 60 fps
        let elapsed = last_frame.elapsed().as_nanos() as u64;
        if elapsed < target_frame_ns {
            std::thread::sleep(Duration::from_nanos(target_frame_ns - elapsed));
        }
        last_frame = Instant::now();
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
