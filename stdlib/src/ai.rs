use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    context::Capability, error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue,
};

impl StdlibRegistry {
    pub fn register_ai(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "generate_text".to_string(),
            Rc::new(StdFunction {
                name: "generate_text".to_string(),
                arity: 3,
                callback: |ctx, args| {
                    let provider = args[0].try_into_string()?;
                    let prompt = args[1].try_into_string()?;
                    let _config = match &args[2] {
                        RuntimeValue::Map { entries, .. } => Some(entries.clone()),
                        _ => None,
                    };

                    // Check environment capability for retrieving API keys
                    if !ctx.config.capabilities.contains(&Capability::Environment) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation("Security policy violation: Environment capability is denied".to_string()),
                            None,
                            None,
                        ));
                    }

                    // Also requires Network capability to make HTTP requests
                    if !ctx.config.capabilities.contains(&Capability::Network) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation("Security policy violation: Network capability is denied".to_string()),
                            None,
                            None,
                        ));
                    }

                    match provider.as_str() {
                        "openai" => {
                            let key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
                            if key.is_empty() {
                                return Ok(RuntimeValue::Str(format!("[Mock OpenAI Response] Prompt: {}", prompt)));
                            }

                            // Real HTTP call to OpenAI Chat Completion
                            let body = serde_json::json!({
                                "model": "gpt-4o-mini",
                                "messages": [{"role": "user", "content": prompt}]
                            });

                            let resp = ureq::post("https://api.openai.com/v1/chat/completions")
                                .set("Authorization", &format!("Bearer {}", key))
                                .set("Content-Type", "application/json")
                                .send_json(body)
                                .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("OpenAI request failed: {}", e)), None, None))?;

                            let json: serde_json::Value = resp.into_json()
                                .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("Failed to parse OpenAI JSON response: {}", e)), None, None))?;

                            let content = json["choices"][0]["message"]["content"].as_str()
                                .ok_or_else(|| RuntimeError::new(RuntimeErrorKind::InvalidOperation("OpenAI response content empty".to_string()), None, None))?;

                            Ok(RuntimeValue::Str(content.to_string()))
                        }
                        "gemini" => {
                            let key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
                            if key.is_empty() {
                                return Ok(RuntimeValue::Str(format!("[Mock Gemini Response] Prompt: {}", prompt)));
                            }

                            // Real HTTP call to Gemini API
                            let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", key);
                            let body = serde_json::json!({
                                "contents": [{
                                    "parts": [{"text": prompt}]
                                }]
                            });

                            let resp = ureq::post(&url)
                                .set("Content-Type", "application/json")
                                .send_json(body)
                                .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("Gemini request failed: {}", e)), None, None))?;

                            let json: serde_json::Value = resp.into_json()
                                .map_err(|e| RuntimeError::new(RuntimeErrorKind::InvalidOperation(format!("Failed to parse Gemini JSON response: {}", e)), None, None))?;

                            let content = json["candidates"][0]["content"]["parts"][0]["text"].as_str()
                                .ok_or_else(|| RuntimeError::new(RuntimeErrorKind::InvalidOperation("Gemini response content empty".to_string()), None, None))?;

                            Ok(RuntimeValue::Str(content.to_string()))
                        }
                        "local" => {
                            // Mock local Llama.cpp inference endpoint check (e.g. running on localhost:8080)
                            let local_url = "http://127.0.0.1:8080/completion";
                            let body = serde_json::json!({
                                "prompt": prompt,
                                "n_predict": 128
                            });

                            match ureq::post(local_url).set("Content-Type", "application/json").send_json(body) {
                                Ok(resp) => {
                                    if let Ok(json) = resp.into_json::<serde_json::Value>() {
                                        if let Some(content) = json["content"].as_str() {
                                            return Ok(RuntimeValue::Str(content.to_string()));
                                        }
                                    }
                                    Ok(RuntimeValue::Str("[Mock Local LLM Response] (local server responded with invalid content)".to_string()))
                                }
                                Err(_) => {
                                    Ok(RuntimeValue::Str(format!("[Mock Local LLM Response] Prompt: {}", prompt)))
                                }
                            }
                        }
                        _ => Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("Unknown AI provider: {}", provider)),
                            None,
                            None,
                        ))
                    }
                },
            }),
        );

        self.register_module(
            "std.ai",
            StdlibModule {
                name: "std.ai".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Environment, Capability::Network],
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use std::cell::RefCell;
    use techscript_runtime::context::{RuntimeConfig, RuntimeContext};

    #[test]
    fn test_register_ai() {
        let mut registry = StdlibRegistry::new();
        registry.register_ai();

        assert!(registry.has_module("std.ai"));

        let module = registry
            .get_module("std.ai")
            .expect("std.ai module should exist");
        assert_eq!(module.name, "std.ai");
        assert_eq!(module.version, "1.0.0");
        assert!(module
            .required_capabilities
            .contains(&Capability::Environment));
        assert!(module.required_capabilities.contains(&Capability::Network));

        let gen_text = module
            .exports
            .get("generate_text")
            .expect("generate_text function should exist");
        assert_eq!(gen_text.name(), "generate_text");
        assert_eq!(gen_text.arity(), 3);
    }

    #[test]
    fn test_generate_text_capabilities() {
        let mut registry = StdlibRegistry::new();
        registry.register_ai();

        let module = registry.get_module("std.ai").unwrap();
        let gen_text = module.exports.get("generate_text").unwrap();

        // 1. Missing both capabilities
        let mut config_none = RuntimeConfig::default();
        config_none.capabilities.remove(&Capability::Environment);
        config_none.capabilities.remove(&Capability::Network);
        let mut ctx_none = RuntimeContext::new(config_none);

        let args = vec![
            RuntimeValue::Str("openai".to_string()),
            RuntimeValue::Str("test".to_string()),
            RuntimeValue::Map {
                entries: Rc::new(RefCell::new(IndexMap::new())),
                is_const: false,
            },
        ];
        let err = gen_text.call(&mut ctx_none, args.clone()).unwrap_err();
        assert!(err.message.contains("Environment capability is denied"));

        // 2. Has Environment, missing Network
        let mut config_env = RuntimeConfig::default();
        config_env.capabilities.insert(Capability::Environment);
        config_env.capabilities.remove(&Capability::Network);
        let mut ctx_env = RuntimeContext::new(config_env);

        let err = gen_text.call(&mut ctx_env, args.clone()).unwrap_err();
        assert!(err.message.contains("Network capability is denied"));
    }

    #[test]
    fn test_generate_text_providers_and_errors() {
        let mut registry = StdlibRegistry::new();
        registry.register_ai();

        let module = registry.get_module("std.ai").unwrap();
        let gen_text = module.exports.get("generate_text").unwrap();

        let mut config = RuntimeConfig::default();
        config.capabilities.insert(Capability::Environment);
        config.capabilities.insert(Capability::Network);
        let mut ctx = RuntimeContext::new(config);

        let map_val = RuntimeValue::Map {
            entries: Rc::new(RefCell::new(IndexMap::new())),
            is_const: false,
        };

        // Unknown provider
        let res_err = gen_text.call(
            &mut ctx,
            vec![
                RuntimeValue::Str("unknown_provider".to_string()),
                RuntimeValue::Str("hello".to_string()),
                map_val.clone(),
            ],
        );
        assert!(res_err.is_err());
        assert!(res_err
            .unwrap_err()
            .message
            .contains("Unknown AI provider: unknown_provider"));

        // Gemini mock response (when GEMINI_API_KEY is unset)
        let res_gemini = gen_text
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Str("gemini".to_string()),
                    RuntimeValue::Str("hello gemini".to_string()),
                    map_val.clone(),
                ],
            )
            .unwrap();
        assert!(res_gemini
            .as_string()
            .unwrap()
            .contains("[Mock Gemini Response] Prompt: hello gemini"));

        // Local mock response (when local server is down)
        let res_local = gen_text
            .call(
                &mut ctx,
                vec![
                    RuntimeValue::Str("local".to_string()),
                    RuntimeValue::Str("hello local".to_string()),
                    map_val.clone(),
                ],
            )
            .unwrap();
        assert!(res_local
            .as_string()
            .unwrap()
            .contains("[Mock Local LLM Response] Prompt: hello local"));
    }
}
