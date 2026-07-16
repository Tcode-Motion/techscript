use crate::analysis::AnalysisManager;
use crate::pass::OptimizationPass;
use crate::result::OptimizationResult;
use crate::statistics::PassStatistics;
use techscript_ast::LiteralVal;
use techscript_ir::instruction::Op;
use techscript_ir::value::Value;
use techscript_ir::Module;
use techscript_syntax::TokenKind;

/// Folds algebraic identities like x + 0, x * 1, and x * 0.
pub struct AlgebraicSimplification;

impl OptimizationPass for AlgebraicSimplification {
    fn name(&self) -> &'static str {
        "algebraic"
    }

    fn run(&mut self, module: &mut Module, _analyses: &mut AnalysisManager) -> OptimizationResult {
        let mut stats = PassStatistics::new(self.name());
        let mut changed = false;

        for func in &mut module.functions {
            for block in &mut func.blocks {
                for inst in &mut block.instructions {
                    if let Op::BinaryOp { op, left, right } = &inst.op {
                        // x + 0 -> x
                        if *op == TokenKind::Plus {
                            if let Value::Const(LiteralVal::Int(0)) = right {
                                inst.op = Op::Load(left.clone());
                                stats.changed = true;
                                stats.instructions_removed += 1;
                                changed = true;
                                continue;
                            }
                            if let Value::Const(LiteralVal::Int(0)) = left {
                                inst.op = Op::Load(right.clone());
                                stats.changed = true;
                                stats.instructions_removed += 1;
                                changed = true;
                                continue;
                            }
                        }

                        // x * 1 -> x
                        if *op == TokenKind::Star {
                            if let Value::Const(LiteralVal::Int(1)) = right {
                                inst.op = Op::Load(left.clone());
                                stats.changed = true;
                                stats.instructions_removed += 1;
                                changed = true;
                                continue;
                            }
                            if let Value::Const(LiteralVal::Int(1)) = left {
                                inst.op = Op::Load(right.clone());
                                stats.changed = true;
                                stats.instructions_removed += 1;
                                changed = true;
                                continue;
                            }

                            // x * 0 -> 0
                            if let Value::Const(LiteralVal::Int(0)) = right {
                                inst.op = Op::Constant(LiteralVal::Int(0));
                                stats.changed = true;
                                stats.instructions_removed += 1;
                                changed = true;
                                continue;
                            }
                            if let Value::Const(LiteralVal::Int(0)) = left {
                                inst.op = Op::Constant(LiteralVal::Int(0));
                                stats.changed = true;
                                stats.instructions_removed += 1;
                                changed = true;
                                continue;
                            }
                        }
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
