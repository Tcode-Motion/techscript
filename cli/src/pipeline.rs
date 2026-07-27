//! # TechScript Compiler Driver — Stage Compilation Pipeline
//!
//! Wires together Lexer → Parser → Semantic → IR Lowering → Optimization → Bytecode
//! Emits CompilationEvents to the EventBus at each stage boundary.

use colored::Colorize;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use techscript_bytecode::BytecodeModule;
use techscript_common::{FileId, SourceManager};
use techscript_runtime::RuntimeValue;

use crate::artifacts::{ArtifactManager, BuildManifest, BuildOutput};
use crate::cache::BuildCache;
use crate::config::EffectiveConfig;
use crate::diagnostics::{DiagnosticStats, RichDiagnostic};
use crate::events::EventBus;
use crate::logging::Logger;
use crate::profiler::TimingProfiler;

/// Build profiles configuration settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildProfile {
    Debug,
    Release,
    ReleaseFast,
    ReleaseSmall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionBackend {
    Interpreter,
    Vm,
    Native,
}

/// Helper to resolve target triple.
pub fn get_host_target_triple() -> String {
    #[cfg(feature = "llvm")]
    unsafe {
        let raw = llvm_sys::target_machine::LLVMGetDefaultTargetTriple();
        let cstr = std::ffi::CStr::from_ptr(raw);
        let s = cstr.to_string_lossy().into_owned();
        llvm_sys::core::LLVMDisposeMessage(raw);
        s
    }
    #[cfg(not(feature = "llvm"))]
    {
        "x86_64-pc-windows-msvc".to_string()
    }
}

/// Compilation pipeline options.
#[derive(Debug, Clone)]
pub struct PipelineOptions {
    pub profile: BuildProfile,
    pub backend: ExecutionBackend,
    pub emit_source_map: bool,
    pub emit_debug_symbols: bool,
    pub emit_compile_db: bool,
    pub emit_build_manifest: bool,
}

/// The result details of compiling a single unit.
#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub file_id: FileId,
    pub path: PathBuf,
    pub bytecode: Option<BytecodeModule>,
    pub diagnostics: Vec<RichDiagnostic>,
    pub duration: Duration,
    pub from_cache: bool,
}

pub struct CompilationPipeline {
    pub source_manager: SourceManager,
    pub config: EffectiveConfig,
    pub cache: BuildCache,
    pub artifacts: ArtifactManager,
    pub events: EventBus,
    pub logger: Logger,
    pub profiler: TimingProfiler,
}

impl CompilationPipeline {
    pub fn new(
        source_manager: SourceManager,
        config: EffectiveConfig,
        cache: BuildCache,
        artifacts: ArtifactManager,
        events: EventBus,
        logger: Logger,
        profiler: TimingProfiler,
    ) -> Self {
        Self {
            source_manager,
            config,
            cache,
            artifacts,
            events,
            logger,
            profiler,
        }
    }

    /// Compiles a single source file.
    pub fn compile_unit(
        &mut self,
        path: &Path,
        opts: &PipelineOptions,
    ) -> anyhow::Result<CompilationResult> {
        let start = Instant::now();
        let source = std::fs::read_to_string(path)?;
        let fid = self
            .source_manager
            .add_file(path.to_path_buf(), source.clone());

        let mut reporter = techscript_errors::DiagnosticReporter::new();

        // Stage 1: Lexer
        self.events
            .emit(&crate::events::CompilationEvent::BeforeLex { path });
        let lex_start = Instant::now();
        let tokens = match techscript_lexer::lex(&source, &mut reporter) {
            Ok(toks) => toks,
            Err(diags) => {
                let rich_diags = diags
                    .iter()
                    .map(|d| RichDiagnostic::from_legacy(d, fid))
                    .collect();
                return Ok(CompilationResult {
                    file_id: fid,
                    path: path.to_path_buf(),
                    bytecode: None,
                    diagnostics: rich_diags,
                    duration: start.elapsed(),
                    from_cache: false,
                });
            }
        };
        let lex_dur = lex_start.elapsed();
        self.events
            .emit(&crate::events::CompilationEvent::AfterLex {
                path,
                token_count: tokens.len(),
                duration: lex_dur,
            });

        // Stage 2: Parser
        self.events
            .emit(&crate::events::CompilationEvent::BeforeParse { path });
        let parse_start = Instant::now();
        let program = match techscript_parser::parse(&tokens, &mut reporter) {
            Ok(ast) => ast,
            Err(diags) => {
                let rich_diags = diags
                    .iter()
                    .map(|d| RichDiagnostic::from_legacy(d, fid))
                    .collect();
                return Ok(CompilationResult {
                    file_id: fid,
                    path: path.to_path_buf(),
                    bytecode: None,
                    diagnostics: rich_diags,
                    duration: start.elapsed(),
                    from_cache: false,
                });
            }
        };
        let parse_dur = parse_start.elapsed();
        self.events
            .emit(&crate::events::CompilationEvent::AfterParse {
                path,
                node_count: program.statements.len(),
                duration: parse_dur,
            });

        // Stage 3: Semantic Analysis
        self.events
            .emit(&crate::events::CompilationEvent::BeforeSemantic { path });
        let sem_start = Instant::now();
        let checked = match techscript_semantic::analyze(program, &mut reporter) {
            Ok(chk) => chk,
            Err(diags) => {
                let rich_diags = diags
                    .iter()
                    .map(|d| RichDiagnostic::from_legacy(d, fid))
                    .collect();
                return Ok(CompilationResult {
                    file_id: fid,
                    path: path.to_path_buf(),
                    bytecode: None,
                    diagnostics: rich_diags,
                    duration: start.elapsed(),
                    from_cache: false,
                });
            }
        };
        let sem_dur = sem_start.elapsed();
        self.events
            .emit(&crate::events::CompilationEvent::AfterSemantic {
                path,
                symbol_count: checked.symbols.scopes.len(),
                duration: sem_dur,
            });

        // Stage 4: IR Lowering
        self.events
            .emit(&crate::events::CompilationEvent::BeforeLowering { path });
        let lower_start = Instant::now();
        let lowered = techscript_ir::lower(&checked.program, "main");
        for diag in &lowered.diagnostics {
            reporter.report(diag.clone());
        }
        if reporter.has_errors() {
            let rich_diags = reporter
                .get_diagnostics()
                .iter()
                .map(|d| RichDiagnostic::from_legacy(d, fid))
                .collect();
            return Ok(CompilationResult {
                file_id: fid,
                path: path.to_path_buf(),
                bytecode: None,
                diagnostics: rich_diags,
                duration: start.elapsed(),
                from_cache: false,
            });
        }
        let lower_dur = lower_start.elapsed();
        self.events
            .emit(&crate::events::CompilationEvent::AfterLowering {
                path,
                function_count: lowered.module.functions.len(),
                duration: lower_dur,
            });

        // Stage 5: Optimization
        self.events
            .emit(&crate::events::CompilationEvent::BeforeOptimize { path });
        let opt_start = Instant::now();
        let mut module = lowered.module;
        let opt_ctx = techscript_optimizer::OptimizationContext::new();
        let opt_res = techscript_optimizer::optimize(&mut module, &opt_ctx);
        let opt_dur = opt_start.elapsed();
        self.events
            .emit(&crate::events::CompilationEvent::AfterOptimize {
                path,
                passes_run: if opt_res.changed { 1 } else { 0 },
                duration: opt_dur,
            });

        // Stage 6: Bytecode Compilation
        self.events
            .emit(&crate::events::CompilationEvent::BeforeBytecode { path });
        let bc_start = Instant::now();
        let bytecode_module = techscript_bytecode::compile(&module);
        let bc_dur = bc_start.elapsed();
        self.events
            .emit(&crate::events::CompilationEvent::AfterBytecode {
                path,
                instruction_count: bytecode_module
                    .functions
                    .iter()
                    .map(|f| f.chunk.instructions.len())
                    .sum(),
                duration: bc_dur,
            });

        // Stage 7: LLVM Native Compilation (if requested)
        if opts.backend == ExecutionBackend::Native {
            #[cfg(not(feature = "llvm"))]
            {
                eprintln!("{}", "Warning: Native compilation requires system LLVM toolchain dependencies (clang/LLVM). This compiler driver was compiled without LLVM features. Fallback VM bytecode execution is recommended.".yellow().bold());
                return Err(anyhow::anyhow!("Native compilation is unavailable because compiler LLVM features are disabled."));
            }

            #[cfg(feature = "llvm")]
            {
                let opt_level = match opts.profile {
                    BuildProfile::Debug => {
                        techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelNone
                    }
                    BuildProfile::Release => {
                        techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault
                    }
                    BuildProfile::ReleaseFast => {
                        techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive
                    }
                    BuildProfile::ReleaseSmall => {
                        techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault
                    }
                };

                let llvm_opts = techscript_llvm_backend::LLVMBackendOptions {
                    target_triple: get_host_target_triple(),
                    opt_level,
                    debug_symbols: opts.emit_debug_symbols,
                };

                let name = path.file_stem().unwrap_or_default().to_string_lossy();
                let out_dir = path.parent().unwrap_or(path).join("build");
                std::fs::create_dir_all(&out_dir)?;
                let obj_ext = if cfg!(windows) { "obj" } else { "o" };
                let obj_path = out_dir.join(format!("{}.{}", name, obj_ext));

                techscript_llvm_backend::LLVMBackend::compile(&module, &llvm_opts, &obj_path)
                    .map_err(|e| anyhow::anyhow!("LLVM Native Compilation failed: {}", e))?;
            }
        }

        let duration = start.elapsed();
        let rich_diags = reporter
            .get_diagnostics()
            .iter()
            .map(|d| RichDiagnostic::from_legacy(d, fid))
            .collect();

        Ok(CompilationResult {
            file_id: fid,
            path: path.to_path_buf(),
            bytecode: Some(bytecode_module),
            diagnostics: rich_diags,
            duration,
            from_cache: false,
        })
    }

    /// Compiles an entire project using topological sorting.
    pub fn compile_project(
        &mut self,
        graph: &mut crate::project::ProjectBuildGraph,
        opts: &PipelineOptions,
    ) -> anyhow::Result<Vec<CompilationResult>> {
        self.events
            .emit(&crate::events::CompilationEvent::BuildStarted {
                unit_count: graph.units.len(),
            });

        // For simple project compilation, we delegate to the scheduler
        let scheduler = crate::scheduler::CompilationScheduler::new(self.config.parallel_jobs);
        let results = scheduler.compile_parallel(graph, self, opts)?;

        let mut stats = DiagnosticStats {
            files_compiled: results.len(),
            elapsed: results.iter().map(|r| r.duration).sum(),
            ..Default::default()
        };

        for res in &results {
            for diag in &res.diagnostics {
                stats.record(diag);
            }
        }

        self.events
            .emit(&crate::events::CompilationEvent::BuildFinished { stats: &stats });

        Ok(results)
    }

    /// Executes compiled result via interpreter, VM, or LLVM JIT.
    pub fn execute(
        &self,
        result: &CompilationResult,
        opts: &PipelineOptions,
    ) -> anyhow::Result<RuntimeValue> {
        match opts.backend {
            ExecutionBackend::Vm => {
                let bytecode = result.bytecode.as_ref().ok_or_else(|| {
                    anyhow::anyhow!("Cannot execute compiled unit: Compilation failed.")
                })?;
                // Run using VM
                techscript_vm::run(bytecode.clone())
                    .map_err(|e| anyhow::anyhow!("VM Runtime Error: {:?}", e))
            }
            ExecutionBackend::Interpreter => {
                // Bridge to tree-walking interpreter
                let content = std::fs::read_to_string(&result.path)?;
                let mut rep = techscript_errors::DiagnosticReporter::new();
                let tokens = techscript_lexer::lex(&content, &mut rep)
                    .map_err(|_| anyhow::anyhow!("Interpreter lexer error"))?;
                let program = techscript_parser::parse(&tokens, &mut rep)
                    .map_err(|_| anyhow::anyhow!("Interpreter parsing error"))?;
                let checked = techscript_semantic::analyze(program, &mut rep)
                    .map_err(|_| anyhow::anyhow!("Interpreter semantic analysis error"))?;
                let res = techscript_interpreter::interpret(checked)
                    .map_err(|e| anyhow::anyhow!("Interpreter Runtime Error: {:?}", e))?;
                Ok(res)
            }
            ExecutionBackend::Native => {
                #[cfg(feature = "llvm")]
                {
                    let content = std::fs::read_to_string(&result.path)?;
                    let mut reporter = techscript_errors::DiagnosticReporter::new();
                    let tokens = techscript_lexer::lex(&content, &mut reporter)
                        .map_err(|e| anyhow::anyhow!("Lexer error: {:?}", e))?;
                    let program = techscript_parser::parse(&tokens, &mut reporter)
                        .map_err(|e| anyhow::anyhow!("Parser error: {:?}", e))?;
                    let checked = techscript_semantic::analyze(program, &mut reporter)
                        .map_err(|e| anyhow::anyhow!("Semantic error: {:?}", e))?;
                    let lowered = techscript_ir::lower(&checked.program, "main");
                    let mut module = lowered.module;

                    let opt_level = match opts.profile {
                        BuildProfile::Debug => {
                            techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelNone
                        }
                        BuildProfile::Release => {
                            techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault
                        }
                        BuildProfile::ReleaseFast => {
                            techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive
                        }
                        BuildProfile::ReleaseSmall => {
                            techscript_llvm_backend::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault
                        }
                    };

                    let llvm_opts = techscript_llvm_backend::LLVMBackendOptions {
                        target_triple: get_host_target_triple(),
                        opt_level,
                        debug_symbols: false,
                    };

                    unsafe {
                        let mut jit = techscript_llvm_backend::jit::LLVMJitEngine::new()
                            .map_err(|e| anyhow::anyhow!("Failed to init JIT: {}", e))?;
                        jit.compile(&module, &llvm_opts)
                            .map_err(|e| anyhow::anyhow!("JIT Compile Error: {}", e))?;
                        let res_code = jit
                            .execute("main")
                            .map_err(|e| anyhow::anyhow!("JIT Execution Error: {}", e))?;
                        Ok(RuntimeValue::Int(res_code))
                    }
                }
                #[cfg(not(feature = "llvm"))]
                {
                    Err(anyhow::anyhow!(
                        "JIT execution requires LLVM feature enabled."
                    ))
                }
            }
        }
    }

    /// Fast semantic analysis only check.
    pub fn check_unit(&mut self, path: &Path) -> anyhow::Result<Vec<RichDiagnostic>> {
        let source = std::fs::read_to_string(path)?;
        let fid = self
            .source_manager
            .add_file(path.to_path_buf(), source.clone());
        let mut reporter = techscript_errors::DiagnosticReporter::new();

        let tokens = match techscript_lexer::lex(&source, &mut reporter) {
            Ok(t) => t,
            Err(diags) => {
                return Ok(diags
                    .iter()
                    .map(|d| RichDiagnostic::from_legacy(d, fid))
                    .collect())
            }
        };

        let program = match techscript_parser::parse(&tokens, &mut reporter) {
            Ok(p) => p,
            Err(diags) => {
                return Ok(diags
                    .iter()
                    .map(|d| RichDiagnostic::from_legacy(d, fid))
                    .collect())
            }
        };

        let _ = techscript_semantic::analyze(program, &mut reporter);

        let diags = reporter
            .get_diagnostics()
            .iter()
            .map(|d| RichDiagnostic::from_legacy(d, fid))
            .collect();
        Ok(diags)
    }
}
