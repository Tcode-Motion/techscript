// ── String interning for fast global lookups ─────────────────────────
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct StringInterner {
    strings: Vec<String>,
    index: HashMap<String, u32>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(&id) = self.index.get(s) {
            return id;
        }
        let id = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.index.insert(s.to_string(), id);
        id
    }

    pub fn get(&self, id: u32) -> Option<&str> {
        self.strings.get(id as usize).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intern_dedup() {
        let mut i = StringInterner::new();
        assert_eq!(i.intern("hello"), 0);
        assert_eq!(i.intern("hello"), 0);
        assert_eq!(i.intern("world"), 1);
    }
}
