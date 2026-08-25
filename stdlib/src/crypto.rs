use crate::{StdFunction, StdlibModule, StdlibRegistry};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use bcrypt;
use sha2::Digest;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_crypto(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "aes_encrypt".to_string(),
            Rc::new(StdFunction {
                name: "aes_encrypt".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let key_str = args[0].try_into_string()?;
                    let text = args[1].try_into_string()?;

                    // Derive a 32-byte key from key_str using Sha256
                    let mut hasher = sha2::Sha256::new();
                    hasher.update(key_str.as_bytes());
                    let hashed_key = hasher.finalize();

                    let cipher = Aes256Gcm::new_from_slice(&hashed_key).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "AES key init error: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;

                    // Nonce is 12-byte zero nonce for simple FFI compatibility
                    let nonce = Nonce::from_slice(&[0u8; 12]);

                    let ciphertext = cipher.encrypt(nonce, text.as_bytes()).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "AES encryption error: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;

                    // Hex encode ciphertext
                    let hex_ciphertext = ciphertext
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>();
                    Ok(RuntimeValue::Str(hex_ciphertext))
                },
            }),
        );

        exports.insert(
            "aes_decrypt".to_string(),
            Rc::new(StdFunction {
                name: "aes_decrypt".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let key_str = args[0].try_into_string()?;
                    let hex_ciphertext = args[1].try_into_string()?;

                    // Decode hex string
                    let mut ciphertext = Vec::new();
                    for i in (0..hex_ciphertext.len()).step_by(2) {
                        if i + 2 <= hex_ciphertext.len() {
                            if let Ok(byte) = u8::from_str_radix(&hex_ciphertext[i..i + 2], 16) {
                                ciphertext.push(byte);
                            } else {
                                return Err(RuntimeError::new(
                                    RuntimeErrorKind::InvalidOperation(
                                        "Invalid hex ciphertext".to_string(),
                                    ),
                                    None,
                                    None,
                                ));
                            }
                        }
                    }

                    let mut hasher = sha2::Sha256::new();
                    hasher.update(key_str.as_bytes());
                    let hashed_key = hasher.finalize();

                    let cipher = Aes256Gcm::new_from_slice(&hashed_key).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "AES key init error: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;

                    let nonce = Nonce::from_slice(&[0u8; 12]);

                    let plaintext_bytes =
                        cipher.decrypt(nonce, ciphertext.as_slice()).map_err(|e| {
                            RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(format!(
                                    "AES decryption error: {}",
                                    e
                                )),
                                None,
                                None,
                            )
                        })?;

                    let plaintext = String::from_utf8(plaintext_bytes).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "Invalid UTF-8 in plaintext: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;

                    Ok(RuntimeValue::Str(plaintext))
                },
            }),
        );

        exports.insert(
            "bcrypt_hash".to_string(),
            Rc::new(StdFunction {
                name: "bcrypt_hash".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let password = args[0].try_into_string()?;
                    let cost = args[1].try_into_int()? as u32;
                    let hashed = bcrypt::hash(&password, cost).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("Bcrypt hash error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Str(hashed))
                },
            }),
        );

        exports.insert(
            "bcrypt_verify".to_string(),
            Rc::new(StdFunction {
                name: "bcrypt_verify".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let password = args[0].try_into_string()?;
                    let hash = args[1].try_into_string()?;
                    let matches = bcrypt::verify(&password, &hash).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!(
                                "Bcrypt verify error: {}",
                                e
                            )),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Bool(matches))
                },
            }),
        );

        self.register_module(
            "std.crypto",
            StdlibModule {
                name: "std.crypto".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
