use crate::analysis::AnalysisManager;
use crate::pass::OptimizationPass;
use crate::result::OptimizationResult;
use crate::statistics::PassStatistics;
use techscript_ast::LiteralVal;
use techscript_ir::instruction::TerminatorKind;
use techscript_ir::value::Value;
use techscript_ir::Module;

/// Simplifies conditional branches with constant conditional values into direct jumps.
pub struct BranchSimplification;

impl OptimizationPass for BranchSimplification {
    fn name(&self) -> &'static str {
        "branch_simplification"
    }

    fn run(&mut self, module: &mut Module, _analyses: &mut AnalysisManager) -> OptimizationResult {
        let mut stats = PassStatistics::new(self.name());
        let mut changed = false;

        for func in &mut module.functions {
            for block in &mut func.blocks {
                if let Some(ref mut term) = block.terminator {
                    if let TerminatorKind::ConditionalJump {
                        cond: Value::Const(LiteralVal::Bool(val)),
                        then_block,
                        else_block,
                    } = &term.kind
                    {
                        let target = if *val { *then_block } else { *else_block };
                        term.kind = TerminatorKind::Jump(target);
                        stats.changed = true;
                        stats.branches_simplified += 1;
                        changed = true;
                    }
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
