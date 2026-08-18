// runtime/native_runtime/src/lib.rs
#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::missing_safety_doc)]

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::os::raw::{c_char, c_void};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TsTag {
    Null = 0,
    Bool = 1,
    Int = 2,
    Float = 3,
    String = 4,
    List = 5,
    Map = 6,
    Struct = 7,
    Model = 8,
    Enum = 9,
}

#[repr(C)]
pub struct TsValue {
    pub tag: u32,
    pub data: TsData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union TsData {
    pub boolean: bool,
    pub integer: i64,
    pub float: f64,
    pub pointer: *mut c_void,
}

// Structures for heap-allocated types
pub struct TsStruct {
    pub name: String,
    pub fields: HashMap<String, *mut TsValue>,
}

pub struct TsModel {
    pub name: String,
    pub fields: HashMap<String, *mut TsValue>,
}

pub struct TsEnum {
    pub name: String,
    pub variant: String,
    pub value: *mut TsValue,
}

// Allocator helpers
#[no_mangle]
pub extern "C" fn ts_alloc_null() -> *mut TsValue {
    let val = Box::new(TsValue {
        tag: TsTag::Null as u32,
        data: TsData { integer: 0 },
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ts_alloc_bool(b: bool) -> *mut TsValue {
    let val = Box::new(TsValue {
        tag: TsTag::Bool as u32,
        data: TsData { boolean: b },
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ts_alloc_int(i: i64) -> *mut TsValue {
    let val = Box::new(TsValue {
        tag: TsTag::Int as u32,
        data: TsData { integer: i },
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ts_alloc_float(f: f64) -> *mut TsValue {
    let val = Box::new(TsValue {
        tag: TsTag::Float as u32,
        data: TsData { float: f },
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ts_alloc_string(s: *const c_char) -> *mut TsValue {
    let rust_str = if s.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(s).to_string_lossy().into_owned() }
    };
    let ptr = Box::into_raw(Box::new(rust_str)) as *mut c_void;
    let val = Box::new(TsValue {
        tag: TsTag::String as u32,
        data: TsData { pointer: ptr },
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ts_alloc_list() -> *mut TsValue {
    let list: Vec<*mut TsValue> = Vec::new();
    let ptr = Box::into_raw(Box::new(list)) as *mut c_void;
    let val = Box::new(TsValue {
        tag: TsTag::List as u32,
        data: TsData { pointer: ptr },
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ts_alloc_map() -> *mut TsValue {
    let map: HashMap<String, *mut TsValue> = HashMap::new();
    let ptr = Box::into_raw(Box::new(map)) as *mut c_void;
    let val = Box::new(TsValue {
        tag: TsTag::Map as u32,
        data: TsData { pointer: ptr },
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ts_alloc_struct(name: *const c_char) -> *mut TsValue {
    let name_str = if name.is_null() { String::new() } else { unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() } };
    let ts_struct = TsStruct {
        name: name_str,
        fields: HashMap::new(),
    };
    let ptr = Box::into_raw(Box::new(ts_struct)) as *mut c_void;
    let val = Box::new(TsValue {
        tag: TsTag::Struct as u32,
        data: TsData { pointer: ptr },
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ts_alloc_model(name: *const c_char) -> *mut TsValue {
    let name_str = if name.is_null() { String::new() } else { unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() } };
    let ts_model = TsModel {
        name: name_str,
        fields: HashMap::new(),
    };
    let ptr = Box::into_raw(Box::new(ts_model)) as *mut c_void;
    let val = Box::new(TsValue {
        tag: TsTag::Model as u32,
        data: TsData { pointer: ptr },
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ts_alloc_enum(
    name: *const c_char,
    variant: *const c_char,
    val_opt: *mut TsValue,
) -> *mut TsValue {
    let name_str = if name.is_null() { String::new() } else { unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() } };
    let variant_str = if variant.is_null() { String::new() } else { unsafe { CStr::from_ptr(variant).to_string_lossy().into_owned() } };
    let ts_enum = TsEnum {
        name: name_str,
        variant: variant_str,
        value: val_opt,
    };
    let ptr = Box::into_raw(Box::new(ts_enum)) as *mut c_void;
    let val = Box::new(TsValue {
        tag: TsTag::Enum as u32,
        data: TsData { pointer: ptr },
    });
    Box::into_raw(val)
}

// Free memory helper (non-panicking cleanup)
#[no_mangle]
pub unsafe extern "C" fn ts_free_value(val: *mut TsValue) {
    if val.is_null() {
        return;
    }
    let value = Box::from_raw(val);
    if value.tag == TsTag::String as u32 {
        let _ = Box::from_raw(value.data.pointer as *mut String);
    } else if value.tag == TsTag::List as u32 {
        let list = Box::from_raw(value.data.pointer as *mut Vec<*mut TsValue>);
        for &item in list.iter() {
            ts_free_value(item);
        }
    } else if value.tag == TsTag::Map as u32 {
        let map = Box::from_raw(value.data.pointer as *mut HashMap<String, *mut TsValue>);
        for (_, item) in map.into_iter() {
            ts_free_value(item);
        }
    } else if value.tag == TsTag::Struct as u32 {
        let s = Box::from_raw(value.data.pointer as *mut TsStruct);
        for (_, item) in s.fields.into_iter() {
            ts_free_value(item);
        }
    } else if value.tag == TsTag::Model as u32 {
        let m = Box::from_raw(value.data.pointer as *mut TsModel);
        for (_, item) in m.fields.into_iter() {
            ts_free_value(item);
        }
    } else if value.tag == TsTag::Enum as u32 {
        let e = Box::from_raw(value.data.pointer as *mut TsEnum);
        ts_free_value(e.value);
    }
}

// Convert TsValue to Rust String helper
unsafe fn value_to_string(val: *mut TsValue) -> String {
    if val.is_null() {
        return "null".to_string();
    }
    let v = &*val;
    if v.tag == TsTag::Null as u32 {
        "null".to_string()
    } else if v.tag == TsTag::Bool as u32 {
        v.data.boolean.to_string()
    } else if v.tag == TsTag::Int as u32 {
        v.data.integer.to_string()
    } else if v.tag == TsTag::Float as u32 {
        v.data.float.to_string()
    } else if v.tag == TsTag::String as u32 {
        (*(v.data.pointer as *const String)).clone()
    } else if v.tag == TsTag::List as u32 {
        let list = &*(v.data.pointer as *const Vec<*mut TsValue>);
        let mut parts = Vec::new();
        for &item in list {
            parts.push(value_to_string(item));
        }
        format!("[{}]", parts.join(", "))
    } else if v.tag == TsTag::Map as u32 {
        let map = &*(v.data.pointer as *const HashMap<String, *mut TsValue>);
        let mut parts = Vec::new();
        for (k, &val) in map {
            parts.push(format!("{}: {}", k, value_to_string(val)));
        }
        format!("{{{}}}", parts.join(", "))
    } else if v.tag == TsTag::Struct as u32 {
        let s = &*(v.data.pointer as *const TsStruct);
        let mut parts = Vec::new();
        for (k, &val) in &s.fields {
            parts.push(format!("{}: {}", k, value_to_string(val)));
        }
        format!("{} {{{}}}", s.name, parts.join(", "))
    } else if v.tag == TsTag::Model as u32 {
        let m = &*(v.data.pointer as *const TsModel);
        let mut parts = Vec::new();
        for (k, &val) in &m.fields {
            parts.push(format!("{}: {}", k, value_to_string(val)));
        }
        format!("{} {{{}}}", m.name, parts.join(", "))
    } else if v.tag == TsTag::Enum as u32 {
        let e = &*(v.data.pointer as *const TsEnum);
        if e.value.is_null() {
            format!("{}.{}", e.name, e.variant)
        } else {
            format!("{}.{}({})", e.name, e.variant, value_to_string(e.value))
        }
    } else {
        "unknown".to_string()
    }
}

// Built-in functions
#[no_mangle]
pub unsafe extern "C" fn ts_say(val: *mut TsValue) {
    let s = value_to_string(val);
    println!("{}", s);
    io::stdout().flush().unwrap_or(());
}

#[no_mangle]
pub unsafe extern "C" fn ts_ask(prompt: *mut TsValue) -> *mut TsValue {
    if !prompt.is_null() {
        print!("{}", value_to_string(prompt));
        io::stdout().flush().unwrap_or(());
    }
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim_end().to_string();
        let ptr = Box::into_raw(Box::new(trimmed)) as *mut c_void;
        let val = Box::new(TsValue {
            tag: TsTag::String as u32,
            data: TsData { pointer: ptr },
        });
        Box::into_raw(val)
    } else {
        ts_alloc_null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn ts_len(val: *mut TsValue) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_int(0);
    }
    let v = &*val;
    if v.tag == TsTag::String as u32 {
        let s = &*(v.data.pointer as *const String);
        ts_alloc_int(s.len() as i64)
    } else if v.tag == TsTag::List as u32 {
        let list = &*(v.data.pointer as *const Vec<*mut TsValue>);
        ts_alloc_int(list.len() as i64)
    } else if v.tag == TsTag::Map as u32 {
        let map = &*(v.data.pointer as *const HashMap<String, *mut TsValue>);
        ts_alloc_int(map.len() as i64)
    } else {
        ts_alloc_int(0)
    }
}

// List operations
#[no_mangle]
pub unsafe extern "C" fn ts_list_push(list_val: *mut TsValue, item_val: *mut TsValue) {
    if list_val.is_null() {
        return;
    }
    let v = &mut *list_val;
    if v.tag == TsTag::List as u32 {
        let list = &mut *(v.data.pointer as *mut Vec<*mut TsValue>);
        list.push(item_val);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ts_list_get(
    list_val: *mut TsValue,
    index_val: *mut TsValue,
) -> *mut TsValue {
    if list_val.is_null() || index_val.is_null() {
        return ts_alloc_null();
    }
    let v = &*list_val;
    let idx_v = &*index_val;
    if v.tag == TsTag::List as u32 && idx_v.tag == TsTag::Int as u32 {
        let list = &*(v.data.pointer as *const Vec<*mut TsValue>);
        let idx = idx_v.data.integer;
        if idx >= 0 && idx < list.len() as i64 {
            return list[idx as usize];
        }
    }
    ts_alloc_null()
}

#[no_mangle]
pub unsafe extern "C" fn ts_list_set(
    list_val: *mut TsValue,
    index_val: *mut TsValue,
    item_val: *mut TsValue,
) {
    if list_val.is_null() || index_val.is_null() {
        return;
    }
    let v = &mut *list_val;
    let idx_v = &*index_val;
    if v.tag == TsTag::List as u32 && idx_v.tag == TsTag::Int as u32 {
        let list = &mut *(v.data.pointer as *mut Vec<*mut TsValue>);
        let idx = idx_v.data.integer;
        if idx >= 0 && idx < list.len() as i64 {
            list[idx as usize] = item_val;
        }
    }
}

// Map operations
#[no_mangle]
pub unsafe extern "C" fn ts_map_get(map_val: *mut TsValue, key_val: *mut TsValue) -> *mut TsValue {
    if map_val.is_null() || key_val.is_null() {
        return ts_alloc_null();
    }
    let v = &*map_val;
    let k_v = &*key_val;
    if v.tag == TsTag::Map as u32 && k_v.tag == TsTag::String as u32 {
        let map = &*(v.data.pointer as *const HashMap<String, *mut TsValue>);
        let key = &*(k_v.data.pointer as *const String);
        if let Some(&item) = map.get(key) {
            return item;
        }
    }
    ts_alloc_null()
}

#[no_mangle]
pub unsafe extern "C" fn ts_map_set(
    map_val: *mut TsValue,
    key_val: *mut TsValue,
    item_val: *mut TsValue,
) {
    if map_val.is_null() || key_val.is_null() {
        return;
    }
    let v = &mut *map_val;
    let k_v = &*key_val;
    if v.tag == TsTag::Map as u32 && k_v.tag == TsTag::String as u32 {
        let map = &mut *(v.data.pointer as *mut HashMap<String, *mut TsValue>);
        let key = (*(k_v.data.pointer as *const String)).clone();
        map.insert(key, item_val);
    }
}

// Struct/Model property access
#[no_mangle]
pub unsafe extern "C" fn ts_struct_get(
    struct_val: *mut TsValue,
    field: *const c_char,
) -> *mut TsValue {
    if struct_val.is_null() || field.is_null() {
        return ts_alloc_null();
    }
    let v = &*struct_val;
    let field_str = CStr::from_ptr(field).to_string_lossy().into_owned();
    if v.tag == TsTag::Struct as u32 {
        let s = &*(v.data.pointer as *const TsStruct);
        if let Some(&item) = s.fields.get(&field_str) {
            return item;
        }
    } else if v.tag == TsTag::Model as u32 {
        let m = &*(v.data.pointer as *const TsModel);
        if let Some(&item) = m.fields.get(&field_str) {
            return item;
        }
    }
    ts_alloc_null()
}

#[no_mangle]
pub unsafe extern "C" fn ts_struct_set(
    struct_val: *mut TsValue,
    field: *const c_char,
    item_val: *mut TsValue,
) {
    if struct_val.is_null() || field.is_null() {
        return;
    }
    let v = &mut *struct_val;
    let field_str = CStr::from_ptr(field).to_string_lossy().into_owned();
    if v.tag == TsTag::Struct as u32 {
        let s = &mut *(v.data.pointer as *mut TsStruct);
        s.fields.insert(field_str, item_val);
    } else if v.tag == TsTag::Model as u32 {
        let m = &mut *(v.data.pointer as *mut TsModel);
        m.fields.insert(field_str, item_val);
    }
}

// Dynamic Arithmetic operations
#[no_mangle]
pub unsafe extern "C" fn ts_add(left: *mut TsValue, right: *mut TsValue) -> *mut TsValue {
    if left.is_null() || right.is_null() {
        return ts_alloc_null();
    }
    let l = &*left;
    let r = &*right;

    // Concatenation if either is a string
    if l.tag == TsTag::String as u32 || r.tag == TsTag::String as u32 {
        let l_str = value_to_string(left);
        let r_str = value_to_string(right);
        let combined = format!("{}{}", l_str, r_str);
        let combined_cstr = CString::new(combined).unwrap();
        return ts_alloc_string(combined_cstr.as_ptr());
    }

    if l.tag == TsTag::Int as u32 && r.tag == TsTag::Int as u32 {
        return ts_alloc_int(l.data.integer + r.data.integer);
    }
    if l.tag == TsTag::Float as u32 || r.tag == TsTag::Float as u32 {
        let lf = if l.tag == TsTag::Int as u32 {
            l.data.integer as f64
        } else {
            l.data.float
        };
        let rf = if r.tag == TsTag::Int as u32 {
            r.data.integer as f64
        } else {
            r.data.float
        };
        return ts_alloc_float(lf + rf);
    }
    ts_alloc_null()
}

#[no_mangle]
pub unsafe extern "C" fn ts_sub(left: *mut TsValue, right: *mut TsValue) -> *mut TsValue {
    if left.is_null() || right.is_null() {
        return ts_alloc_null();
    }
    let l = &*left;
    let r = &*right;
    if l.tag == TsTag::Int as u32 && r.tag == TsTag::Int as u32 {
        return ts_alloc_int(l.data.integer - r.data.integer);
    }
    if l.tag == TsTag::Float as u32 || r.tag == TsTag::Float as u32 {
        let lf = if l.tag == TsTag::Int as u32 {
            l.data.integer as f64
        } else {
            l.data.float
        };
        let rf = if r.tag == TsTag::Int as u32 {
            r.data.integer as f64
        } else {
            r.data.float
        };
        return ts_alloc_float(lf - rf);
    }
    ts_alloc_null()
}

#[no_mangle]
pub unsafe extern "C" fn ts_mul(left: *mut TsValue, right: *mut TsValue) -> *mut TsValue {
    if left.is_null() || right.is_null() {
        return ts_alloc_null();
    }
    let l = &*left;
    let r = &*right;
    if l.tag == TsTag::Int as u32 && r.tag == TsTag::Int as u32 {
        return ts_alloc_int(l.data.integer * r.data.integer);
    }
    if l.tag == TsTag::Float as u32 || r.tag == TsTag::Float as u32 {
        let lf = if l.tag == TsTag::Int as u32 {
            l.data.integer as f64
        } else {
            l.data.float
        };
        let rf = if r.tag == TsTag::Int as u32 {
            r.data.integer as f64
        } else {
            r.data.float
        };
        return ts_alloc_float(lf * rf);
    }
    ts_alloc_null()
}

#[no_mangle]
pub unsafe extern "C" fn ts_div(left: *mut TsValue, right: *mut TsValue) -> *mut TsValue {
    if left.is_null() || right.is_null() {
        return ts_alloc_null();
    }
    let l = &*left;
    let r = &*right;

    let lf = if l.tag == TsTag::Int as u32 {
        l.data.integer as f64
    } else {
        l.data.float
    };
    let rf = if r.tag == TsTag::Int as u32 {
        r.data.integer as f64
    } else {
        r.data.float
    };
    if rf == 0.0 {
        return ts_alloc_null(); // Avoid division by zero panic
    }
    if l.tag == TsTag::Int as u32 && r.tag == TsTag::Int as u32 {
        return ts_alloc_int(l.data.integer / r.data.integer);
    }
    ts_alloc_float(lf / rf)
}

#[no_mangle]
pub unsafe extern "C" fn ts_mod(left: *mut TsValue, right: *mut TsValue) -> *mut TsValue {
    if left.is_null() || right.is_null() {
        return ts_alloc_null();
    }
    let l = &*left;
    let r = &*right;
    if l.tag == TsTag::Int as u32 && r.tag == TsTag::Int as u32 {
        if r.data.integer == 0 {
            return ts_alloc_null();
        }
        return ts_alloc_int(l.data.integer % r.data.integer);
    }
    ts_alloc_null()
}

#[no_mangle]
pub unsafe extern "C" fn ts_pow(left: *mut TsValue, right: *mut TsValue) -> *mut TsValue {
    if left.is_null() || right.is_null() {
        return ts_alloc_null();
    }
    let lf = if (&*left).tag == TsTag::Int as u32 {
        (&*left).data.integer as f64
    } else {
        (&*left).data.float
    };
    let rf = if (&*right).tag == TsTag::Int as u32 {
        (&*right).data.integer as f64
    } else {
        (&*right).data.float
    };
    ts_alloc_float(lf.powf(rf))
}

// Dynamic Comparison helpers
#[no_mangle]
pub unsafe extern "C" fn ts_eq(left: *mut TsValue, right: *mut TsValue) -> bool {
    if left.is_null() || right.is_null() {
        return left == right;
    }
    let l = &*left;
    let r = &*right;
    if l.tag != r.tag {
        // Coerce numbers
        if (l.tag == TsTag::Int as u32 || l.tag == TsTag::Float as u32)
            && (r.tag == TsTag::Int as u32 || r.tag == TsTag::Float as u32)
        {
            let lf = if l.tag == TsTag::Int as u32 {
                l.data.integer as f64
            } else {
                l.data.float
            };
            let rf = if r.tag == TsTag::Int as u32 {
                r.data.integer as f64
            } else {
                r.data.float
            };
            return lf == rf;
        }
        return false;
    }
    if l.tag == TsTag::Null as u32 {
        return true;
    }
    if l.tag == TsTag::Bool as u32 {
        return l.data.boolean == r.data.boolean;
    }
    if l.tag == TsTag::Int as u32 {
        return l.data.integer == r.data.integer;
    }
    if l.tag == TsTag::Float as u32 {
        return l.data.float == r.data.float;
    }
    if l.tag == TsTag::String as u32 {
        let ls = &*(l.data.pointer as *const String);
        let rs = &*(r.data.pointer as *const String);
        return ls == rs;
    }
    l.data.pointer == r.data.pointer
}

#[no_mangle]
pub unsafe extern "C" fn ts_ne(left: *mut TsValue, right: *mut TsValue) -> bool {
    !ts_eq(left, right)
}

#[no_mangle]
pub unsafe extern "C" fn ts_lt(left: *mut TsValue, right: *mut TsValue) -> bool {
    if left.is_null() || right.is_null() {
        return false;
    }
    let l = &*left;
    let r = &*right;
    if l.tag == TsTag::Int as u32 && r.tag == TsTag::Int as u32 {
        return l.data.integer < r.data.integer;
    }
    if (l.tag == TsTag::Int as u32 || l.tag == TsTag::Float as u32)
        && (r.tag == TsTag::Int as u32 || r.tag == TsTag::Float as u32)
    {
        let lf = if l.tag == TsTag::Int as u32 {
            l.data.integer as f64
        } else {
            l.data.float
        };
        let rf = if r.tag == TsTag::Int as u32 {
            r.data.integer as f64
        } else {
            r.data.float
        };
        return lf < rf;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn ts_le(left: *mut TsValue, right: *mut TsValue) -> bool {
    if left.is_null() || right.is_null() {
        return false;
    }
    ts_lt(left, right) || ts_eq(left, right)
}

#[no_mangle]
pub unsafe extern "C" fn ts_gt(left: *mut TsValue, right: *mut TsValue) -> bool {
    if left.is_null() || right.is_null() {
        return false;
    }
    let l = &*left;
    let r = &*right;
    if l.tag == TsTag::Int as u32 && r.tag == TsTag::Int as u32 {
        return l.data.integer > r.data.integer;
    }
    if (l.tag == TsTag::Int as u32 || l.tag == TsTag::Float as u32)
        && (r.tag == TsTag::Int as u32 || r.tag == TsTag::Float as u32)
    {
        let lf = if l.tag == TsTag::Int as u32 {
            l.data.integer as f64
        } else {
            l.data.float
        };
        let rf = if r.tag == TsTag::Int as u32 {
            r.data.integer as f64
        } else {
            r.data.float
        };
        return lf > rf;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn ts_ge(left: *mut TsValue, right: *mut TsValue) -> bool {
    if left.is_null() || right.is_null() {
        return false;
    }
    ts_gt(left, right) || ts_eq(left, right)
}

// Cast operator
#[no_mangle]
pub unsafe extern "C" fn ts_cast(val: *mut TsValue, target_tag: u32) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_null();
    }
    let v = &*val;
    if v.tag == target_tag {
        return val;
    }
    if target_tag == TsTag::String as u32 {
        let s = value_to_string(val);
        let s_cstr = CString::new(s).unwrap();
        return ts_alloc_string(s_cstr.as_ptr());
    }
    if target_tag == TsTag::Int as u32 {
        if v.tag == TsTag::Bool as u32 {
            return ts_alloc_int(if v.data.boolean { 1 } else { 0 });
        }
        if v.tag == TsTag::Float as u32 {
            return ts_alloc_int(v.data.float as i64);
        }
        if v.tag == TsTag::String as u32 {
            let s = &*(v.data.pointer as *const String);
            if let Ok(i) = s.parse::<i64>() {
                return ts_alloc_int(i);
            }
        }
        return ts_alloc_int(0);
    }
    if target_tag == TsTag::Float as u32 {
        if v.tag == TsTag::Int as u32 {
            return ts_alloc_float(v.data.integer as f64);
        }
        if v.tag == TsTag::String as u32 {
            let s = &*(v.data.pointer as *const String);
            if let Ok(f) = s.parse::<f64>() {
                return ts_alloc_float(f);
            }
        }
        return ts_alloc_float(0.0);
    }
    if target_tag == TsTag::Bool as u32 {
        if v.tag == TsTag::Null as u32 {
            return ts_alloc_bool(false);
        }
        if v.tag == TsTag::Int as u32 {
            return ts_alloc_bool(v.data.integer != 0);
        }
        return ts_alloc_bool(true);
    }
    ts_alloc_null()
}

// Math builtins
#[no_mangle]
pub unsafe extern "C" fn ts_math_abs(val: *mut TsValue) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_null();
    }
    let v = &*val;
    if v.tag == TsTag::Int as u32 {
        ts_alloc_int(v.data.integer.abs())
    } else if v.tag == TsTag::Float as u32 {
        ts_alloc_float(v.data.float.abs())
    } else {
        ts_alloc_null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn ts_math_sin(val: *mut TsValue) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_null();
    }
    let v = &*val;
    let lf = if v.tag == TsTag::Int as u32 {
        v.data.integer as f64
    } else {
        v.data.float
    };
    ts_alloc_float(lf.sin())
}

#[no_mangle]
pub unsafe extern "C" fn ts_math_cos(val: *mut TsValue) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_null();
    }
    let v = &*val;
    let lf = if v.tag == TsTag::Int as u32 {
        v.data.integer as f64
    } else {
        v.data.float
    };
    ts_alloc_float(lf.cos())
}

#[no_mangle]
pub unsafe extern "C" fn ts_math_tan(val: *mut TsValue) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_null();
    }
    let v = &*val;
    let lf = if v.tag == TsTag::Int as u32 {
        v.data.integer as f64
    } else {
        v.data.float
    };
    ts_alloc_float(lf.tan())
}

#[no_mangle]
pub unsafe extern "C" fn ts_math_log(val: *mut TsValue) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_null();
    }
    let v = &*val;
    let lf = if v.tag == TsTag::Int as u32 {
        v.data.integer as f64
    } else {
        v.data.float
    };
    ts_alloc_float(lf.ln())
}

#[no_mangle]
pub unsafe extern "C" fn ts_math_exp(val: *mut TsValue) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_null();
    }
    let v = &*val;
    let lf = if v.tag == TsTag::Int as u32 {
        v.data.integer as f64
    } else {
        v.data.float
    };
    ts_alloc_float(lf.exp())
}

#[no_mangle]
pub unsafe extern "C" fn ts_math_sqrt(val: *mut TsValue) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_null();
    }
    let v = &*val;
    let lf = if v.tag == TsTag::Int as u32 {
        v.data.integer as f64
    } else {
        v.data.float
    };
    ts_alloc_float(lf.sqrt())
}

// Range function helper (say 0..5 returns List of Ints)
#[no_mangle]
pub unsafe extern "C" fn ts_range(start_val: *mut TsValue, end_val: *mut TsValue) -> *mut TsValue {
    if start_val.is_null() || end_val.is_null() {
        return ts_alloc_list();
    }
    let s_v = &*start_val;
    let e_v = &*end_val;
    if s_v.tag == TsTag::Int as u32 && e_v.tag == TsTag::Int as u32 {
        let start = s_v.data.integer;
        let end = e_v.data.integer;
        let list_val = ts_alloc_list();
        for i in start..end {
            ts_list_push(list_val, ts_alloc_int(i));
        }
        return list_val;
    }
    ts_alloc_list()
}

// Unified indexing helpers
#[no_mangle]
pub unsafe extern "C" fn ts_index_get(base: *mut TsValue, index: *mut TsValue) -> *mut TsValue {
    if base.is_null() || index.is_null() {
        return ts_alloc_null();
    }
    let b = &*base;
    if b.tag == TsTag::List as u32 {
        return ts_list_get(base, index);
    } else if b.tag == TsTag::Map as u32 {
        return ts_map_get(base, index);
    } else if b.tag == TsTag::String as u32 {
        let s = &*(b.data.pointer as *const String);
        let idx_v = &*index;
        if idx_v.tag == TsTag::Int as u32 {
            let idx = idx_v.data.integer;
            if idx >= 0 && idx < s.len() as i64 {
                if let Some(ch) = s.chars().nth(idx as usize) {
                    let char_str = ch.to_string();
                    let cstr = CString::new(char_str).unwrap();
                    return ts_alloc_string(cstr.as_ptr());
                }
            }
        }
    }
    ts_alloc_null()
}

#[no_mangle]
pub unsafe extern "C" fn ts_index_set(base: *mut TsValue, index: *mut TsValue, val: *mut TsValue) {
    if base.is_null() || index.is_null() {
        return;
    }
    let b = &mut *base;
    if b.tag == TsTag::List as u32 {
        ts_list_set(base, index, val);
    } else if b.tag == TsTag::Map as u32 {
        ts_map_set(base, index, val);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ts_await(val: *mut TsValue) -> *mut TsValue {
    if val.is_null() {
        return ts_alloc_null();
    }
    let ts_val = &*val;
    if ts_val.tag == TsTag::Map as u32 {
        let map_ptr = ts_val.data.pointer as *mut HashMap<String, *mut TsValue>;
        if !map_ptr.is_null() {
            let map = &*map_ptr;
            if map.contains_key("state") && map.contains_key("value") {
                loop {
                    let state_val = map.get("state").cloned().unwrap_or(std::ptr::null_mut());
                    if !state_val.is_null() && (*state_val).tag == TsTag::String as u32 {
                        let state_str_ptr = (*state_val).data.pointer as *mut String;
                        if !state_str_ptr.is_null() {
                            let state_str = &*state_str_ptr;
                            if state_str == "pending" {
                                std::thread::yield_now();
                                continue;
                            } else if state_str == "resolved" {
                                let res = map.get("value").cloned().unwrap_or(std::ptr::null_mut());
                                return if res.is_null() { ts_alloc_null() } else { res };
                            } else if state_str == "rejected" {
                                return ts_alloc_null();
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
    val
}
