// =============================================================================
// TechScript AppForge — Touch & Gesture Dispatch Engine
// runtime/android_runtime/src/touch.rs
//
// Handles raw Android input events:
//   - Touch drag gestures for 3D object rotation
//   - Taps on liquid glass buttons, navigation dock tabs, and controls
//   - Dynamic code execution dispatches
//   - Hardware back button routing (returns to previous screen or exits)
//   - External link opening via Android JNI
// =============================================================================

use crate::native_bridge::{AppSettings, NativeBridge};
use crate::renderer::{Action, NativeRenderer};
use crate::txs_runtime::TechScriptRuntime;
use android_activity::input::{InputEvent, KeyAction, KeyEvent, MotionAction, MotionEvent};
use android_activity::AndroidApp;

#[derive(Default)]
pub struct TouchTracker {
    pub down_x: f32,
    pub down_y: f32,
    pub last_x: f32,
    pub last_y: f32,
    pub is_dragging: bool,
    pub scroll_velocity: f32,
    pub last_pinch_dist: f32,
    pub is_pinching: bool,
}

pub fn handle_input(
    event: &InputEvent,
    runtime: &mut TechScriptRuntime,
    renderer: &NativeRenderer,
    app: &AndroidApp,
    tracker: &mut TouchTracker,
    needs_redraw: &mut bool,
) -> bool {
    match event {
        InputEvent::KeyEvent(key) => handle_key(key, runtime, needs_redraw),
        InputEvent::MotionEvent(motion) => {
            handle_motion(motion, runtime, renderer, app, tracker, needs_redraw)
        }
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
    app: &AndroidApp,
    tracker: &mut TouchTracker,
    needs_redraw: &mut bool,
) -> bool {
    let pointer_count = event.pointer_count();

    // ── Multi-Touch Pinch-To-Zoom & Two-Finger Pan ───────────────────────────
    if pointer_count >= 2 {
        let p0 = event.pointer_at_index(0);
        let p1 = event.pointer_at_index(1);
        let dist = ((p1.x() - p0.x()).powi(2) + (p1.y() - p0.y()).powi(2)).sqrt();
        let mid_x = (p0.x() + p1.x()) * 0.5;
        let mid_y = (p0.y() + p1.y()) * 0.5;

        if tracker.is_pinching {
            let dist_delta = dist - tracker.last_pinch_dist;
            tracker.last_pinch_dist = dist;
            // Spread fingers: zoom in; pinch fingers: zoom out
            runtime.adjust_zoom(dist_delta * 0.005);

            let mdx = mid_x - tracker.last_x;
            let mdy = mid_y - tracker.last_y;
            tracker.last_x = mid_x;
            tracker.last_y = mid_y;
            // Two finger pan translates the 3D model
            runtime.pan_model(mdx * 0.004, -mdy * 0.004);
            *needs_redraw = true;
        } else {
            tracker.last_pinch_dist = dist;
            tracker.last_x = mid_x;
            tracker.last_y = mid_y;
            tracker.is_pinching = true;
        }
        return true;
    } else {
        tracker.is_pinching = false;
    }

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
            tracker.scroll_velocity = 0.0;
            runtime.scroll_velocity = 0.0; // Cancel ongoing inertia on touch down

            let is_3d_tap = if let Some((vx, vy, vw, vh)) = renderer.viewport_3d_rect {
                px >= vx && px <= vx + vw && py >= vy && py <= vy + vh
            } else {
                false
            };
            runtime.is_touching_3d = is_3d_tap;
            true
        }
        MotionAction::Move => {
            let dx = px - tracker.last_x;
            let dy = py - tracker.last_y;
            tracker.last_x = px;
            tracker.last_y = py;

            let dist = ((px - tracker.down_x).powi(2) + (py - tracker.down_y).powi(2)).sqrt();
            if dist > 22.0 {
                tracker.is_dragging = true;
            }

            // 3D drag rotation on Viewport rect or vertical scroll
            let is_3d_drag = if let Some((vx, vy, vw, vh)) = renderer.viewport_3d_rect {
                tracker.down_x >= vx
                    && tracker.down_x <= vx + vw
                    && tracker.down_y >= vy
                    && tracker.down_y <= vy + vh
            } else {
                false
            };

            if is_3d_drag && tracker.is_dragging {
                runtime.is_touching_3d = true;
                let rot_sensitivity = 0.015;
                runtime.rot_y += dx * rot_sensitivity;
                runtime.rot_x += dy * rot_sensitivity;
                *needs_redraw = true;
            } else if tracker.is_dragging {
                runtime.is_touching_3d = false;
                // Responsive moving average for flick / fling momentum
                tracker.scroll_velocity = tracker.scroll_velocity * 0.3 + (-dy) * 0.7;
                // Direct position update
                runtime.scroll_y = (runtime.scroll_y - dy).clamp(0.0, runtime.max_scroll_y);
                *needs_redraw = true;
            }
            true
        }
        MotionAction::Up => {
            runtime.is_touching_3d = false;
            let total_dist = ((px - tracker.down_x).powi(2) + (py - tracker.down_y).powi(2)).sqrt();

            let mut button_handled = false;
            // Generous tap radius: if displacement is under 32px, check hit regions using both touch-up and touch-down coords
            if total_dist < 32.0 {
                for region in &renderer.hit_regions {
                    if region.contains(px, py) || region.contains(tracker.down_x, tracker.down_y) {
                        button_handled = true;
                        log::info!("[TechScript Input] Dispatched Action: {:?}", region.action);
                        match &region.action {
                            Action::Navigate(target) => {
                                runtime.navigate(target);
                            }
                            Action::ToggleTheme => {
                                runtime.cycle_theme();
                                let settings = AppSettings {
                                    theme_mode: runtime.theme_mode().to_string(),
                                    last_screen: runtime.active_screen().to_string(),
                                    auto_rotate: runtime.auto_rotate,
                                    anim_speed: runtime.anim_speed,
                                    scroll_y: runtime.scroll_y,
                                };
                                NativeBridge::save_settings(app, &settings);
                            }
                            Action::Cycle3DMesh => {
                                runtime.cycle_mesh();
                            }
                            Action::Prev3DMesh => {
                                runtime.mesh_type = runtime.mesh_type.prev();
                            }
                            Action::Cycle3DMaterial => {
                                runtime.cycle_material();
                            }
                            Action::Set3DMesh(m) => {
                                runtime.mesh_type = *m;
                            }
                            Action::Set3DMaterial(m) => {
                                runtime.set_material(*m);
                            }
                            Action::ToggleAutoRotate => {
                                runtime.toggle_auto_rotate();
                            }
                            Action::Reset3DCamera => {
                                runtime.reset_camera();
                            }
                            Action::Pan3D(dx, dy) => {
                                runtime.pan_model(*dx, *dy);
                            }
                            Action::Reset3DPosition => {
                                runtime.reset_3d_position();
                            }
                            Action::SetCustomHue(h) => {
                                runtime.set_custom_hue(*h);
                            }
                            Action::AdjustZoom(delta) => {
                                runtime.adjust_zoom(*delta);
                            }
                            Action::ToggleSpeed => {
                                runtime.toggle_speed();
                            }
                            Action::SelectModelCategory(cat) => {
                                runtime.set_model_category(*cat);
                            }
                            Action::ToggleWireframe => {
                                runtime.toggle_wireframe();
                            }
                            Action::SelectExplorerCategory(cat) => {
                                runtime.select_explorer_category(*cat);
                            }
                            Action::IncrementCounter => {
                                runtime.increment_counter();
                            }
                            Action::DecrementCounter => {
                                runtime.demo_counter = runtime.demo_counter.saturating_sub(1);
                            }
                            Action::ResetCounter => {
                                runtime.demo_counter = 0;
                            }
                            Action::SetSlider(val) => {
                                runtime.demo_slider = *val;
                            }
                            Action::ToggleSwitch => {
                                runtime.toggle_switch();
                            }
                            Action::ToggleExplorerCode => {
                                runtime.toggle_explorer_code();
                            }
                            Action::SelectProject(idx) => {
                                runtime.select_project(*idx);
                                runtime.scroll_y = 0.0;
                                runtime.scroll_velocity = 0.0;
                            }
                            Action::SelectProjectFilter(idx) => {
                                runtime.project_filter = *idx;
                                runtime.scroll_y = 0.0;
                                runtime.scroll_velocity = 0.0;
                            }
                            Action::RunProjectDirect(idx) => {
                                runtime.selected_project = *idx;
                                runtime.show_source = true;
                                runtime.scroll_y = 0.0;
                                runtime.scroll_velocity = 0.0;
                                runtime.run_selected_project();
                            }
                            Action::ValidateProjectDirect(idx) => {
                                runtime.validate_project(*idx);
                            }
                            Action::ToggleCodeView => {
                                runtime.toggle_source_view();
                                runtime.scroll_y = 0.0;
                                runtime.scroll_velocity = 0.0;
                            }
                            Action::ValidateProject => {
                                runtime.validate_selected_project();
                            }
                            Action::RunSelectedProject => {
                                runtime.run_selected_project();
                            }
                            Action::RunQuickAction(cmd) => {
                                log::info!("[TechScript Action] Triggering live sequential execution: {cmd}");
                                runtime.start_command_execution(cmd);
                            }
                            Action::SelectDocTopic(idx) => {
                                runtime.select_doc_topic(*idx);
                                runtime.scroll_y = 0.0;
                                runtime.scroll_velocity = 0.0;
                            }
                            Action::OpenAbout => {
                                runtime.navigate("about");
                            }
                            Action::OpenLink(url) => {
                                log::info!(
                                    "[TechScript Intent] Dispatching ACTION_VIEW for: {url}"
                                );
                                if let Err(e) = NativeBridge::open_url(app, url) {
                                    log::warn!("[TechScript Intent] NativeBridge error: {e}");
                                }
                            }
                        }
                        *needs_redraw = true;
                        break;
                    }
                }
            }

            if !button_handled && tracker.is_dragging {
                // If dragged and flicked with momentum, transfer velocity to runtime for smooth inertia
                if tracker.scroll_velocity.abs() > 2.0 {
                    runtime.scroll_velocity = tracker.scroll_velocity.clamp(-65.0, 65.0);
                }
            }

            tracker.is_dragging = false;
            *needs_redraw = true;
            true
        }
        _ => false,
    }
}
