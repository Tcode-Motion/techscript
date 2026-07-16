use techscript_bytecode::{BytecodeModule, BytecodeSerializer, BytecodeValidator};

/// Loader responsible for validating and parsing compiled bytecode binaries.
pub struct BytecodeLoader;

impl BytecodeLoader {
    /// Loads, validates headers, and returns a compiled module.
    pub fn load(bytes: &[u8]) -> Result<BytecodeModule, String> {
        // Deserialize module
        let module = BytecodeSerializer::deserialize(bytes)?;

        // Run integrity validation checks
        let validator = BytecodeValidator::new();
        validator.validate(&module)?;

        Ok(module)
    }
}
