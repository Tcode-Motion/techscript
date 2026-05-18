pub mod web;
pub mod gui;
pub mod three;
pub mod anime;

use std::collections::HashMap;

use crate::value::Value;

pub fn load_module(name: &str, globals: &mut HashMap<String, Value>) -> Result<(), String> {
    match name {
        "web" => { web::register(globals); Ok(()) }
        "gui" => { gui::register(globals); Ok(()) }
        "3d" => { three::register(globals); Ok(()) }
        "anime" => { anime::register(globals); Ok(()) }
        _ => Err(format!("Unknown module: '{}'. Available: web, gui, 3d, anime", name)),
    }
}

pub fn available_modules() -> &'static [&'static str] {
    &["web", "gui", "3d", "anime"]
}
