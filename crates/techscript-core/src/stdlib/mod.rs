pub mod math;
pub mod fs;
pub mod os;
pub mod random;
pub mod json;
pub mod crypto;
pub mod date;
pub mod net;
pub mod web;
pub mod debug;
pub mod api;

use std::collections::HashMap;
use crate::value::Value;

pub fn register_all(globals: &mut HashMap<String, Value>) {
    math::register_math_module(globals);
    fs::register_fs_module(globals);
    os::register_os_module(globals);
    random::register_random_module(globals);
    json::register_json_module(globals);
    crypto::register_crypto_module(globals);
    date::register_date_module(globals);
    net::register_net_module(globals);
    web::register_web_module(globals);
    web::register_gui_module(globals);
    web::register_three_d_module(globals);
    web::register_anime_module(globals);
    debug::register_debug_module(globals);
    api::register_api_module(globals);
}
