use std::collections::{HashMap, HashSet};
use techscript_ir::block::BasicBlock;
use techscript_ir::function::Function;
use techscript_ir::module::Module;

/// Validator checking AST-lowered IR structural and CFG consistency.
pub struct IRVerifier;

impl Default for IRVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl IRVerifier {
    /// Creates a new IRVerifier.
    pub fn new() -> Self {
        Self
    }

    /// Verifies the structural correctness of the module.
    pub fn verify(&self, module: &Module) -> Result<(), String> {
        for func in &module.functions {
            self.verify_function(func)?;
        }
        Ok(())
    }

    fn verify_function(&self, func: &Function) -> Result<(), String> {
        let mut block_ids: HashSet<_> = HashSet::with_capacity(func.blocks.len());
        // ⚡ Bolt Performance Optimization:
        // Use HashMap for O(1) block lookups instead of O(N) linear scans during CFG verification.
        let mut block_map = HashMap::with_capacity(func.blocks.len());
        for b in &func.blocks {
            block_ids.insert(b.id);
            block_map.insert(b.id, b);
        }

        for block in &func.blocks {
            self.verify_block(block, &block_ids)?;
        }

        // Verify CFG link mapping consistency
        for block in &func.blocks {
            for &succ in &block.successors {
                if let Some(&sb) = block_map.get(&succ) {
                    if !sb.predecessors.contains(&block.id) {
                        return Err(format!(
                            "CFG inconsistency: Block '{}' lists '{}' as successor, but '{}' does not list '{}' as predecessor",
                            block.label, sb.label, sb.label, block.label
                        ));
                    }
                } else {
                    return Err(format!(
                        "CFG inconsistency: Block '{}' references non-existent successor BlockId({:?})",
                        block.label, succ
                    ));
                }
            }
        }

        Ok(())
    }

    fn verify_block(
        &self,
        block: &BasicBlock,
        block_ids: &HashSet<techscript_ir::types::BlockId>,
    ) -> Result<(), String> {
        // Every block must have exactly one terminator
        if block.terminator.is_none() {
            return Err(format!(
                "Block '{}' is missing a control-flow terminator",
                block.label
            ));
        }

        // Verify successor references
        for &succ in &block.successors {
            if !block_ids.contains(&succ) {
                return Err(format!(
                    "Block '{}' references non-existent successor BlockId({:?})",
                    block.label, succ
                ));
            }
        }

        Ok(())
    }
}
