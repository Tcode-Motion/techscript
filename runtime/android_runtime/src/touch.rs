// =============================================================================
// TechScript AppForge — Touch & Gesture Dispatch Engine
// runtime/android_runtime/src/touch.rs
//
// Handles raw Android input events:
//   - Touch drag gestures for 3D object rotation
//   - Taps on liquid glass buttons, navigation dock tabs, and controls
//   - Hardware back button routing (returns to previous screen or exits)
// =============================================================================

use android_activity::input::{InputEvent, KeyEvent, KeyAction, MotionEvent, MotionAction};
use crate::txs_runtime::TechScriptRuntime;
use crate::renderer::{NativeRenderer, Action};

#[derive(Default)]
pub struct TouchTracker {
    pub down_x: f32,
    pub down_y: f32,
    pub last_x: f32,
    pub last_y: f32,
    pub is_dragging: bool,
}

pub fn handle_input(
    event: &InputEvent,
    runtime: &mut TechScriptRuntime,
    renderer: &NativeRenderer,
    tracker: &mut TouchTracker,
    needs_redraw: &mut bool,
) -> bool {
    match event {
        InputEvent::KeyEvent(key) => handle_key(key, runtime, needs_redraw),
        InputEvent::MotionEvent(motion) => handle_motion(motion, runtime, renderer, tracker, needs_redraw),
        _ => false,
    }
}

fn handle_key(event: &KeyEvent, runtime: &mut TechScriptRuntime, needs_redraw: &mut bool) -> bool {
    use android_activity::input::Keycode;
    if event.action() == KeyAction::Up && event.key_code() == Keycode::Back {
        let result = runtime.on_back_pressed();
        *needs_redraw = true;
        result.is_some()
    } else {
        false
    }
}

fn handle_motion(
    event: &MotionEvent,
    runtime: &mut TechScriptRuntime,
    renderer: &NativeRenderer,
    tracker: &mut TouchTracker,
    needs_redraw: &mut bool,
) -> bool {
    let pointer = event.pointer_at_index(0);
    let px = pointer.x();
    let py = pointer.y();

    match event.action() {
        MotionAction::Down => {
            tracker.down_x = px;
            tracker.down_y = py;
            tracker.last_x = px;
            tracker.last_y = py;
            tracker.is_dragging = false;
            true
        }
        MotionAction::Move => {
            let dx = px - tracker.last_x;
            let dy = py - tracker.last_y;
            tracker.last_x = px;
            tracker.last_y = py;

            let dist = ((px - tracker.down_x).powi(2) + (py - tracker.down_y).powi(2)).sqrt();
            if dist > 8.0 {
                tracker.is_dragging = true;
            }

            // 3D drag rotation on Home or Labs screens
            let screen = runtime.active_screen();
            if (screen == "home" || screen == "labs") && tracker.is_dragging {
                runtime.auto_rotate = false;
                let rot_sensitivity = 0.012;
                runtime.rot_y += dx * rot_sensitivity;
                runtime.rot_x += dy * rot_sensitivity;
                *needs_redraw = true;
            }
            true
        }
        MotionAction::Up => {
            let total_dist = ((px - tracker.down_x).powi(2) + (py - tracker.down_y).powi(2)).sqrt();

            // Click / tap detection
            if total_dist < 20.0 {
                for region in &renderer.hit_regions {
                    if region.contains(px, py) {
                        log::info!("[TechScript Input] Dispatched Action: {:?}", region.action);
                        match &region.action {
                            Action::Navigate(target) => {
                                runtime.navigate(target);
                            }
                            Action::ToggleTheme => {
                                runtime.cycle_theme();
                            }
                            Action::Cycle3DMesh => {
                                runtime.cycle_mesh();
                            }
                            Action::Cycle3DMaterial => {
                                runtime.cycle_material();
                            }
                            Action::ToggleAutoRotate => {
                                runtime.toggle_auto_rotate();
                            }
                            Action::Reset3DCamera => {
                                runtime.reset_camera();
                            }
                            Action::AdjustZoom(delta) => {
                                runtime.adjust_zoom(*delta);
                            }
                            Action::ToggleSpeed => {
                                runtime.toggle_speed();
                            }
                            Action::SelectExplorerCategory(cat) => {
                                runtime.select_explorer_category(*cat);
                            }
                            Action::IncrementCounter => {
                                runtime.increment_counter();
                            }
                            Action::ToggleSwitch => {
                                runtime.toggle_switch();
                            }
                            Action::ToggleExplorerCode => {
                                runtime.toggle_explorer_code();
                            }
                            Action::SelectProject(idx) => {
                                runtime.select_project(*idx);
                            }
                            Action::ToggleCodeView => {
                                runtime.toggle_source_view();
                            }
                            Action::ValidateProject => {
                                runtime.validate_selected_project();
                            }
                            Action::SelectDocTopic(idx) => {
                                runtime.select_doc_topic(*idx);
                            }
                            Action::OpenAbout => {
                                runtime.navigate("about");
                            }
                            Action::OpenLink(url) => {
                                log::info!("[TechScript Intent] Open URL: {url}");
                            }
                        }
                        *needs_redraw = true;
                        break;
                    }
                }
            }

            tracker.is_dragging = false;
            *needs_redraw = true;
            true
        }
        _ => false,
    }
}
