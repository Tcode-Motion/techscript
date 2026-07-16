use crate::analysis::AnalysisManager;
use crate::pass::OptimizationPass;
use crate::result::OptimizationResult;
use crate::statistics::PassStatistics;
use std::collections::HashSet;
use techscript_ir::instruction::Op;
use techscript_ir::types::LocalId;
use techscript_ir::value::Value;
use techscript_ir::Module;

/// Eliminates stores to local variables that are overwritten without intermediate reads.
pub struct DeadStore;

impl OptimizationPass for DeadStore {
    fn name(&self) -> &'static str {
        "dead_store"
    }

    fn run(&mut self, module: &mut Module, _analyses: &mut AnalysisManager) -> OptimizationResult {
        let mut stats = PassStatistics::new(self.name());
        let mut changed = false;

        for func in &mut module.functions {
            for block in &mut func.blocks {
                let mut active_stores: HashSet<LocalId> = HashSet::new();
                let mut to_remove: HashSet<usize> = HashSet::new();

                // Iterate backwards to track stores and intermediate loads
                for (idx, inst) in block.instructions.iter().enumerate().rev() {
                    match &inst.op {
                        Op::Store {
                            target: Value::Local(local_id),
                            ..
                        } => {
                            if active_stores.contains(local_id) {
                                // Overwritten store detected!
                                to_remove.insert(idx);
                            } else {
                                active_stores.insert(*local_id);
                            }
                        }
                        Op::Load(Value::Local(local_id)) => {
                            // Variable read: it is no longer dead store target
                            active_stores.remove(local_id);
                        }
                        _ => {}
                    }
                }

                if !to_remove.is_empty() {
                    let mut new_insts = Vec::new();
                    for (idx, inst) in block.instructions.drain(..).enumerate() {
                        if to_remove.contains(&idx) {
                            stats.instructions_removed += 1;
                            stats.changed = true;
                            changed = true;
                        } else {
                            new_insts.push(inst);
                        }
                    }
                    block.instructions = new_insts;
                }
            }
        }

        if changed {
            OptimizationResult::changed(stats)
        } else {
            OptimizationResult::unchanged(self.name())
        }
    }
}
