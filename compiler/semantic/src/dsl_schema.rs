use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DSLSchema {
    pub valid_properties: Vec<String>,
    pub required_properties: Vec<String>,
    pub allowed_children: Vec<String>,
}

impl DSLSchema {
    pub fn new(
        valid_properties: Vec<String>,
        required_properties: Vec<String>,
        allowed_children: Vec<String>,
    ) -> Self {
        Self {
            valid_properties,
            required_properties,
            allowed_children,
        }
    }

    pub fn is_valid_property(&self, name: &str) -> bool {
        self.valid_properties.contains(&name.to_string())
    }

    pub fn is_required_property(&self, name: &str) -> bool {
        self.required_properties.contains(&name.to_string())
    }

    pub fn is_allowed_child(&self, kind: &str) -> bool {
        self.allowed_children.contains(&kind.to_string())
    }
}

pub fn build_dsl_registry() -> HashMap<String, DSLSchema> {
    let mut reg = HashMap::new();

    // Web module schemas ──────────────────────────────────────────────
    reg.insert(
        "website".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "description".into(),
                "lang".into(),
                "base_url".into(),
            ],
            vec!["title".into()],
            vec![
                "page".into(),
                "header".into(),
                "footer".into(),
                "nav".into(),
            ],
        ),
    );

    reg.insert(
        "page".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "description".into(),
                "icon".into(),
                "theme".into(),
            ],
            vec!["title".into()],
            vec![
                "hero".into(),
                "section".into(),
                "card".into(),
                "nav".into(),
                "header".into(),
                "main".into(),
                "aside".into(),
                "form".into(),
                "start".into(),
            ],
        ),
    );

    reg.insert(
        "hero".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "subtitle".into(),
                "tagline".into(),
                "background".into(),
                "color".into(),
                "image".into(),
                "align".into(),
                "size".into(),
            ],
            vec![],
            vec![
                "button".into(),
                "link".into(),
                "input".into(),
                "start".into(),
            ],
        ),
    );

    reg.insert(
        "section".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "subtitle".into(),
                "background".into(),
                "color".into(),
                "padding".into(),
                "width".into(),
                "align".into(),
                "divider".into(),
                "id".into(),
            ],
            vec![],
            vec![
                "card".into(),
                "button".into(),
                "link".into(),
                "form".into(),
                "hero".into(),
                "section".into(),
            ],
        ),
    );

    reg.insert(
        "card".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "subtitle".into(),
                "text".into(),
                "icon".into(),
                "image".into(),
                "color".into(),
                "background".into(),
                "width".into(),
                "height".into(),
                "shadow".into(),
                "rounded".into(),
                "border".into(),
            ],
            vec![],
            vec!["button".into(), "link".into(), "input".into()],
        ),
    );

    reg.insert(
        "footer".to_string(),
        DSLSchema::new(
            vec![
                "text".into(),
                "color".into(),
                "background".into(),
                "align".into(),
                "padding".into(),
            ],
            vec![],
            vec!["link".into(), "nav".into(), "section".into()],
        ),
    );

    reg.insert(
        "button".to_string(),
        DSLSchema::new(
            vec![
                "label".into(),
                "color".into(),
                "background".into(),
                "size".into(),
                "rounded".into(),
                "border".into(),
                "icon".into(),
                "width".into(),
                "action".into(),
            ],
            vec!["label".into()],
            vec![],
        ),
    );

    reg.insert(
        "link".to_string(),
        DSLSchema::new(
            vec![
                "label".into(),
                "url".into(),
                "color".into(),
                "size".into(),
                "icon".into(),
                "target".into(),
            ],
            vec!["label".into()],
            vec![],
        ),
    );

    reg.insert(
        "input".to_string(),
        DSLSchema::new(
            vec![
                "label".into(),
                "placeholder".into(),
                "type".into(),
                "value".into(),
                "required".into(),
                "name".into(),
            ],
            vec![],
            vec![],
        ),
    );

    reg.insert(
        "form".to_string(),
        DSLSchema::new(
            vec!["action".into(), "method".into(), "name".into()],
            vec![],
            vec!["input".into(), "button".into()],
        ),
    );

    reg.insert(
        "nav".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "align".into(),
                "background".into(),
                "color".into(),
            ],
            vec![],
            vec!["link".into(), "button".into()],
        ),
    );

    reg.insert(
        "header".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "subtitle".into(),
                "background".into(),
                "color".into(),
                "align".into(),
                "size".into(),
            ],
            vec![],
            vec!["nav".into(), "button".into(), "link".into()],
        ),
    );

    reg.insert(
        "main".to_string(),
        DSLSchema::new(
            vec!["id".into(), "class".into()],
            vec![],
            vec![
                "hero".into(),
                "section".into(),
                "card".into(),
                "aside".into(),
            ],
        ),
    );

    reg.insert(
        "aside".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "background".into(),
                "color".into(),
                "width".into(),
            ],
            vec![],
            vec!["link".into(), "nav".into(), "card".into()],
        ),
    );

    reg.insert(
        "start".to_string(),
        DSLSchema::new(
            vec![
                "label".into(),
                "url".into(),
                "color".into(),
                "background".into(),
                "size".into(),
            ],
            vec!["label".into()],
            vec![],
        ),
    );

    // Canvas module schemas ──────────────────────────────────────────
    reg.insert(
        "logo".to_string(),
        DSLSchema::new(
            vec![
                "text".into(),
                "color".into(),
                "font".into(),
                "size".into(),
                "background".into(),
                "rounded".into(),
                "shadow".into(),
                "padding".into(),
            ],
            vec!["text".into()],
            vec![],
        ),
    );

    reg.insert(
        "rings".to_string(),
        DSLSchema::new(
            vec![
                "count".into(),
                "color".into(),
                "size".into(),
                "thickness".into(),
                "spacing".into(),
                "rotation".into(),
            ],
            vec![],
            vec![],
        ),
    );

    reg.insert(
        "emblem".to_string(),
        DSLSchema::new(
            vec![
                "icon".into(),
                "color".into(),
                "size".into(),
                "background".into(),
                "shape".into(),
                "border".into(),
                "shadow".into(),
            ],
            vec![],
            vec![],
        ),
    );

    reg.insert(
        "core".to_string(),
        DSLSchema::new(
            vec![
                "color".into(),
                "size".into(),
                "shape".into(),
                "glow".into(),
                "pulse".into(),
            ],
            vec![],
            vec![],
        ),
    );

    reg.insert(
        "letter".to_string(),
        DSLSchema::new(
            vec![
                "char".into(),
                "color".into(),
                "font".into(),
                "size".into(),
                "weight".into(),
                "style".into(),
                "transform".into(),
            ],
            vec!["char".into()],
            vec![],
        ),
    );

    reg.insert(
        "circuits".to_string(),
        DSLSchema::new(
            vec![
                "color".into(),
                "density".into(),
                "width".into(),
                "animated".into(),
                "complexity".into(),
            ],
            vec![],
            vec![],
        ),
    );

    reg.insert(
        "title".to_string(),
        DSLSchema::new(
            vec![
                "text".into(),
                "color".into(),
                "font".into(),
                "size".into(),
                "align".into(),
                "weight".into(),
            ],
            vec!["text".into()],
            vec![],
        ),
    );

    reg.insert(
        "subtitle".to_string(),
        DSLSchema::new(
            vec![
                "text".into(),
                "color".into(),
                "font".into(),
                "size".into(),
                "align".into(),
            ],
            vec![],
            vec![],
        ),
    );

    reg.insert(
        "tagline".to_string(),
        DSLSchema::new(
            vec!["text".into(), "color".into(), "font".into(), "size".into()],
            vec![],
            vec![],
        ),
    );

    reg.insert(
        "theme".to_string(),
        DSLSchema::new(
            vec![
                "primary".into(),
                "secondary".into(),
                "background".into(),
                "text".into(),
                "accent".into(),
                "font".into(),
                "rounded".into(),
                "shadow".into(),
            ],
            vec![],
            vec![],
        ),
    );

    reg.insert(
        "animation".to_string(),
        DSLSchema::new(
            vec![
                "type".into(),
                "duration".into(),
                "delay".into(),
                "repeat".into(),
                "easing".into(),
            ],
            vec![],
            vec![],
        ),
    );

    reg.insert(
        "export".to_string(),
        DSLSchema::new(
            vec![
                "format".into(),
                "path".into(),
                "quality".into(),
                "width".into(),
                "height".into(),
            ],
            vec!["format".into()],
            vec![],
        ),
    );

    // Generic DSL blocks ────────────────────────────────────────────
    reg.insert(
        "window".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "width".into(),
                "height".into(),
                "resizable".into(),
                "position".into(),
            ],
            vec!["title".into()],
            vec![],
        ),
    );

    reg.insert(
        "dialog".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "message".into(),
                "buttons".into(),
                "width".into(),
                "height".into(),
            ],
            vec!["title".into(), "message".into()],
            vec!["button".into(), "input".into()],
        ),
    );

    reg.insert(
        "menu".to_string(),
        DSLSchema::new(
            vec![
                "title".into(),
                "align".into(),
                "background".into(),
                "color".into(),
            ],
            vec![],
            vec!["link".into(), "button".into()],
        ),
    );

    reg
}
