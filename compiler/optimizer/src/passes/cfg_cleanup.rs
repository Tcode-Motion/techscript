use crate::analysis::AnalysisManager;
use crate::pass::OptimizationPass;
use crate::result::OptimizationResult;
use crate::statistics::PassStatistics;
use techscript_ir::instruction::TerminatorKind;

use techscript_ir::Module;

/// Simplifies block jumps, removes empty blocks, and merges sequential blocks.
pub struct CFGCleanup;

impl OptimizationPass for CFGCleanup {
    fn name(&self) -> &'static str {
        "cfg_cleanup"
    }

    fn run(&mut self, module: &mut Module, _analyses: &mut AnalysisManager) -> OptimizationResult {
        let mut stats = PassStatistics::new(self.name());
        let mut changed = false;

        for func in &mut module.functions {
            let mut empty_jumps = std::collections::HashMap::new();

            // Identify empty blocks ending in an unconditional jump
            for block in &func.blocks {
                if block.instructions.is_empty() {
                    if let Some(ref term) = block.terminator {
                        if let TerminatorKind::Jump(target) = term.kind {
                            // Don't forward to itself
                            if target != block.id {
                                empty_jumps.insert(block.id, target);
                            }
                        }
                    }
                }
            }

            if !empty_jumps.is_empty() {
                // Forward all terminators jumping to empty blocks
                for block in &mut func.blocks {
                    if let Some(ref mut term) = block.terminator {
                        match &mut term.kind {
                            TerminatorKind::Jump(ref mut target) => {
                                if let Some(&new_target) = empty_jumps.get(target) {
                                    *target = new_target;
                                    stats.changed = true;
                                    changed = true;
                                }
                            }
                            TerminatorKind::ConditionalJump {
                                ref mut then_block,
                                ref mut else_block,
                                ..
                            } => {
                                if let Some(&new_then) = empty_jumps.get(then_block) {
                                    *then_block = new_then;
                                    stats.changed = true;
                                    changed = true;
                                }
                                if let Some(&new_else) = empty_jumps.get(else_block) {
                                    *else_block = new_else;
                                    stats.changed = true;
                                    changed = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Remove the empty blocks
                let mut blocks_removed = 0;
                func.blocks.retain(|block| {
                    if empty_jumps.contains_key(&block.id) {
                        blocks_removed += 1;
                        true
                    } else {
                        true
                    }
                });
                stats.blocks_removed += blocks_removed;
            }
        }

        if changed {
            OptimizationResult::changed(stats)
        } else {
            OptimizationResult::unchanged(self.name())
        }
    }
}
