use crate::analysis::AnalysisManager;
use crate::pass::OptimizationPass;
use crate::result::OptimizationResult;
use crate::statistics::PassStatistics;
use techscript_ast::LiteralVal;
use techscript_ir::instruction::Op;
use techscript_ir::value::Value;
use techscript_ir::Module;
use techscript_syntax::TokenKind;

/// Folds unary/binary operations with constant literal operands.
pub struct ConstantFolding;

impl OptimizationPass for ConstantFolding {
    fn name(&self) -> &'static str {
        "constant_folding"
    }

    fn run(&mut self, module: &mut Module, _analyses: &mut AnalysisManager) -> OptimizationResult {
        let mut stats = PassStatistics::new(self.name());
        let mut changed = false;

        for func in &mut module.functions {
            for block in &mut func.blocks {
                for inst in &mut block.instructions {
                    if let Op::BinaryOp { op, left, right } = &inst.op {
                        if let (Value::Const(l_lit), Value::Const(r_lit)) = (left, right) {
                            if let Some(folded) = self.fold_binary(op, l_lit, r_lit) {
                                inst.op = Op::Constant(folded);
                                stats.changed = true;
                                stats.instructions_removed += 1;
                                stats.constants_folded += 1;
                                changed = true;
                            }
                        }
                    } else if let Op::UnaryOp {
                        op,
                        right: Value::Const(r_lit),
                    } = &inst.op
                    {
                        if let Some(folded) = self.fold_unary(op, r_lit) {
                            inst.op = Op::Constant(folded);
                            stats.changed = true;
                            stats.instructions_removed += 1;
                            stats.constants_folded += 1;
                            changed = true;
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

impl ConstantFolding {
    fn fold_binary(
        &self,
        op: &TokenKind,
        left: &LiteralVal,
        right: &LiteralVal,
    ) -> Option<LiteralVal> {
        match (left, right) {
            (LiteralVal::Int(l), LiteralVal::Int(r)) => match op {
                TokenKind::Plus => Some(LiteralVal::Int(l + r)),
                TokenKind::Minus => Some(LiteralVal::Int(l - r)),
                TokenKind::Star => Some(LiteralVal::Int(l * r)),
                TokenKind::Slash => {
                    if *r != 0 {
                        Some(LiteralVal::Float(*l as f64 / *r as f64))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            (LiteralVal::Float(l), LiteralVal::Float(r)) => match op {
                TokenKind::Plus => Some(LiteralVal::Float(l + r)),
                TokenKind::Minus => Some(LiteralVal::Float(l - r)),
                TokenKind::Star => Some(LiteralVal::Float(l * r)),
                TokenKind::Slash => {
                    if *r != 0.0 {
                        Some(LiteralVal::Float(l / r))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            (LiteralVal::Bool(l), LiteralVal::Bool(r)) => match op {
                TokenKind::And => Some(LiteralVal::Bool(*l && *r)),
                TokenKind::Or => Some(LiteralVal::Bool(*l || *r)),
                _ => None,
            },
            _ => None,
        }
    }

    fn fold_unary(&self, op: &TokenKind, right: &LiteralVal) -> Option<LiteralVal> {
        match right {
            LiteralVal::Int(r) => match op {
                TokenKind::Minus => Some(LiteralVal::Int(-r)),
                _ => None,
            },
            LiteralVal::Float(f) => match op {
                TokenKind::Minus => Some(LiteralVal::Float(-f)),
                _ => None,
            },
            LiteralVal::Bool(b) => match op {
                TokenKind::Not => Some(LiteralVal::Bool(!b)),
                _ => None,
            },
            _ => None,
        }
    }
}
