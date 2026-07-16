use crate::context::SemanticContext;
use techscript_ast::Program;

/// Common interface for configurable semantic verification stages.
pub trait Pass {
    fn run(&mut self, program: &Program, context: &mut SemanticContext);
}

/// Orchestrator running semantic compiler verification tasks sequentially.
pub struct PassPipeline {
    passes: Vec<Box<dyn Pass>>,
}

impl Default for PassPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl PassPipeline {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn add_pass(&mut self, pass: Box<dyn Pass>) {
        self.passes.push(pass);
    }

    pub fn execute(&mut self, program: &Program, context: &mut SemanticContext) {
        for pass in &mut self.passes {
            pass.run(program, context);
        }
    }
}
