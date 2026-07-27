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
