// =============================================================================
// TechScript AppForge — Native System Bridge (JNI & File Storage)
// runtime/android_runtime/src/native_bridge.rs
//
// Zero-Java Native System Integrations:
//   - Dispatches android.content.Intent.ACTION_VIEW via JNI
//   - Persists user theme & state to internal app storage
//   - Manages state restoration between configuration changes
// =============================================================================

use android_activity::AndroidApp;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    pub theme_mode: String,
    pub last_screen: String,
    pub auto_rotate: bool,
    pub anim_speed: f32,
    pub scroll_y: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme_mode: "dark".into(),
            last_screen: "home".into(),
            auto_rotate: true,
            anim_speed: 1.0,
            scroll_y: 0.0,
        }
    }
}

pub struct NativeBridge;

impl NativeBridge {
    /// Launches an external URL in the Android system web browser via JNI Intent.
    pub fn open_url(app: &AndroidApp, url: &str) -> Result<(), String> {
        log::info!("[TechScript NativeBridge] Opening URL: {url}");

        let vm_ptr = app.vm_as_ptr() as *mut jni::sys::JavaVM;
        let activity_ptr = app.activity_as_ptr() as *mut ndk_sys::ANativeActivity;

        if vm_ptr.is_null() || activity_ptr.is_null() {
            return Err("VM or Activity pointer is null".into());
        }

        let url_owned = url.to_string();

        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            move || -> Result<(), String> {
                unsafe {
                    use jni::signature::{MethodSignature, RuntimeMethodSignature};
                    use jni::strings::JNIString;

                    let vm = jni::JavaVM::from_raw(vm_ptr);
                    let raw_activity = (*activity_ptr).clazz as jni::sys::jobject;
                    if raw_activity.is_null() {
                        return Err("Activity clazz is null".into());
                    }

                    vm.attach_current_thread(|env| {
                        if env.exception_check() {
                            let _ = env.exception_clear();
                        }

                        // 1. Uri.parse(url)
                        let url_jstr = env.new_string(&url_owned)?;
                        let uri_class = env.find_class(JNIString::from("android/net/Uri"))?;
                        let parse_sig = RuntimeMethodSignature::from_str(
                            "(Ljava/lang/String;)Landroid/net/Uri;",
                        )
                        .map_err(|e| jni::errors::Error::JavaException)?;
                        let uri_val = env.call_static_method(
                            &uri_class,
                            JNIString::from("parse"),
                            MethodSignature::from(&parse_sig),
                            &[jni::objects::JValue::Object(&url_jstr)],
                        )?;
                        let uri_obj = uri_val.l()?;

                        // 2. new Intent(Intent.ACTION_VIEW, uri)
                        let intent_class =
                            env.find_class(JNIString::from("android/content/Intent"))?;
                        let action_str = env.new_string("android.intent.action.VIEW")?;
                        let init_sig = RuntimeMethodSignature::from_str(
                            "(Ljava/lang/String;Landroid/net/Uri;)V",
                        )
                        .map_err(|e| jni::errors::Error::JavaException)?;
                        let intent_obj = env.new_object(
                            &intent_class,
                            MethodSignature::from(&init_sig),
                            &[
                                jni::objects::JValue::Object(&action_str),
                                jni::objects::JValue::Object(&uri_obj),
                            ],
                        )?;

                        // 3. Add Intent.FLAG_ACTIVITY_NEW_TASK (0x10000000)
                        let flags_sig =
                            RuntimeMethodSignature::from_str("(I)Landroid/content/Intent;")
                                .map_err(|e| jni::errors::Error::JavaException)?;
                        let _ = env.call_method(
                            &intent_obj,
                            JNIString::from("addFlags"),
                            MethodSignature::from(&flags_sig),
                            &[jni::objects::JValue::Int(0x10000000)],
                        );

                        // 4. Call startActivity on the NativeActivity clazz
                        let activity_obj = jni::objects::JObject::from_raw(env, raw_activity);
                        let act_sig =
                            RuntimeMethodSignature::from_str("(Landroid/content/Intent;)V")
                                .map_err(|e| jni::errors::Error::JavaException)?;
                        let call_res = env.call_method(
                            &activity_obj,
                            JNIString::from("startActivity"),
                            MethodSignature::from(&act_sig),
                            &[jni::objects::JValue::Object(&intent_obj)],
                        );

                        // Prevent Drop from calling DeleteLocalRef on the OS-owned global NativeActivity clazz!
                        std::mem::forget(activity_obj);

                        if env.exception_check() {
                            let _ = env.exception_describe();
                            let _ = env.exception_clear();
                            log::warn!(
                                "[TechScript NativeBridge] Cleared Java exception in startActivity"
                            );
                        }

                        call_res?;
                        log::info!("[TechScript NativeBridge] Successfully opened URL in browser");
                        Ok(())
                    })
                    .map_err(|e: jni::errors::Error| e.to_string())
                }
            },
        ));

        match res {
            Ok(inner_res) => inner_res,
            Err(_) => {
                log::error!("[TechScript NativeBridge] Caught panic during open_url JNI call");
                Err("Panic in open_url".into())
            }
        }
    }

    /// Resolves the app's persistent storage file path.
    pub fn get_settings_path(app: &AndroidApp) -> Option<PathBuf> {
        let dir = app.internal_data_path()?;
        Some(dir.join("techscript_settings.json"))
    }

    /// Loads saved user settings from internal app storage.
    pub fn load_settings(app: &AndroidApp) -> AppSettings {
        if let Some(path) = Self::get_settings_path(app) {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                        log::info!(
                            "[TechScript NativeBridge] Restored settings: theme={}, screen={}",
                            settings.theme_mode,
                            settings.last_screen
                        );
                        return settings;
                    }
                }
            }
        }
        AppSettings::default()
    }

    /// Persists settings to internal app storage.
    pub fn save_settings(app: &AndroidApp, settings: &AppSettings) {
        if let Some(path) = Self::get_settings_path(app) {
            if let Ok(json) = serde_json::to_string_pretty(settings) {
                if let Err(e) = fs::write(&path, json) {
                    log::warn!("[TechScript NativeBridge] Failed to write settings: {e}");
                } else {
                    log::info!("[TechScript NativeBridge] Persisted settings to {:?}", path);
                }
            }
        }
    }
}
