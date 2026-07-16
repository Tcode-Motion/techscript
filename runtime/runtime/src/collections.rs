use crate::error::{RuntimeError, RuntimeErrorKind};
use crate::value::RuntimeValue;

/// Retrieves an item from a list value at the specified index, supporting negative indexing.
pub fn list_get(list: &RuntimeValue, idx: i64) -> Result<RuntimeValue, RuntimeError> {
    if let RuntimeValue::List { items, .. } = list {
        let vec = items.borrow();
        let len = vec.len() as i64;
        let final_idx = if idx < 0 { len + idx } else { idx };
        if final_idx < 0 || final_idx >= len {
            return Err(RuntimeError::new(
                RuntimeErrorKind::IndexOutOfBounds,
                None,
                None,
            ));
        }
        Ok(vec[final_idx as usize].clone())
    } else {
        Err(RuntimeError::new(
            RuntimeErrorKind::TypeMismatch {
                expected: "list".to_string(),
                found: list.runtime_type().to_string(),
            },
            None,
            None,
        ))
    }
}

/// Sets an item in a list value at the specified index, checking mutability.
pub fn list_set(list: &RuntimeValue, idx: i64, val: RuntimeValue) -> Result<(), RuntimeError> {
    if let RuntimeValue::List { items, is_const } = list {
        if *is_const {
            return Err(RuntimeError::new(
                RuntimeErrorKind::InvalidOperation("Cannot mutate const list".to_string()),
                None,
                None,
            ));
        }
        let mut vec = items.borrow_mut();
        let len = vec.len() as i64;
        let final_idx = if idx < 0 { len + idx } else { idx };
        if final_idx < 0 || final_idx >= len {
            return Err(RuntimeError::new(
                RuntimeErrorKind::IndexOutOfBounds,
                None,
                None,
            ));
        }
        vec[final_idx as usize] = val;
        Ok(())
    } else {
        Err(RuntimeError::new(
            RuntimeErrorKind::TypeMismatch {
                expected: "list".to_string(),
                found: list.runtime_type().to_string(),
            },
            None,
            None,
        ))
    }
}

/// Retrieves an item from a map value using a string key.
pub fn map_get(map: &RuntimeValue, key: &str) -> Result<RuntimeValue, RuntimeError> {
    if let RuntimeValue::Map { entries, .. } = map {
        if let Some(val) = entries.borrow().get(key) {
            Ok(val.clone())
        } else {
            Ok(RuntimeValue::Null)
        }
    } else {
        Err(RuntimeError::new(
            RuntimeErrorKind::TypeMismatch {
                expected: "map".to_string(),
                found: map.runtime_type().to_string(),
            },
            None,
            None,
        ))
    }
}

/// Inserts or updates an entry in a map value, checking mutability.
pub fn map_set(map: &RuntimeValue, key: String, val: RuntimeValue) -> Result<(), RuntimeError> {
    if let RuntimeValue::Map { entries, is_const } = map {
        if *is_const {
            return Err(RuntimeError::new(
                RuntimeErrorKind::InvalidOperation("Cannot mutate const map".to_string()),
                None,
                None,
            ));
        }
        entries.borrow_mut().insert(key, val);
        Ok(())
    } else {
        Err(RuntimeError::new(
            RuntimeErrorKind::TypeMismatch {
                expected: "map".to_string(),
                found: map.runtime_type().to_string(),
            },
            None,
            None,
        ))
    }
}
