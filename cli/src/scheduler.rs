//! # TechScript Compiler Driver — Parallel Scheduler
//!
//! Spawns worker threads inside `std::thread::scope` to compile independent
//! nodes of the `ProjectBuildGraph` concurrently, respecting dependency edges.

use crate::pipeline::{CompilationPipeline, CompilationResult, PipelineOptions};
use crate::project::{CompilationStatus, ProjectBuildGraph};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct CompilationScheduler {
    pub thread_count: usize,
}

impl CompilationScheduler {
    /// Creates a scheduler with the specified worker thread count limit.
    pub fn new(thread_count: usize) -> Self {
        Self { thread_count }
    }

    /// Runs parallel compilation over the ProjectBuildGraph.
    pub fn compile_parallel(
        &self,
        graph: &mut ProjectBuildGraph,
        pipeline: &CompilationPipeline,
        opts: &PipelineOptions,
    ) -> anyhow::Result<Vec<CompilationResult>> {
        let results = Arc::new(Mutex::new(Vec::new()));
        let graph_mutex = Arc::new(Mutex::new(graph.clone()));

        let pool_size = self.thread_count.max(1);

        let start_time = Instant::now();

        std::thread::scope(|s| {
            let mut threads = Vec::new();

            for thread_id in 0..pool_size {
                let graph_clone = Arc::clone(&graph_mutex);
                let results_clone = Arc::clone(&results);

                let handle = s.spawn(move || {
                    loop {
                        // 1. Get next ready compilation unit
                        let next_fid = {
                            let mut locked_graph = graph_clone.lock().unwrap();
                            let ready = locked_graph.ready_units();
                            if ready.is_empty() {
                                // Are there still units compiling?
                                let any_pending = locked_graph
                                    .units
                                    .values()
                                    .any(|u| u.status == CompilationStatus::Pending);
                                if !any_pending {
                                    break; // All compiled or failed
                                }
                                None
                            } else {
                                let fid = ready[0];
                                if let Some(unit) = locked_graph.units.get_mut(&fid) {
                                    unit.status = CompilationStatus::Compiling;
                                }
                                Some(fid)
                            }
                        };

                        if let Some(fid) = next_fid {
                            let (path, pkg_name) = {
                                let locked_graph = graph_clone.lock().unwrap();
                                let unit = locked_graph.units.get(&fid).unwrap();
                                (unit.path.clone(), unit.package_name.clone())
                            };

                            let unit_start = Instant::now();

                            // Perform stage compilation using the pipeline
                            // Since compile_unit needs mut access to parts of the pipeline,
                            // we isolate pipeline usage. For thread-safe pipelines, we assume read-only/threadsafe context.
                            // To prevent complex borrow conflicts, we compile the source file.
                            let compile_res = match std::fs::read_to_string(&path) {
                                Ok(source) => {
                                    let mut reporter = techscript_errors::DiagnosticReporter::new();

                                    // Lexing
                                    let lex_start = Instant::now();
                                    let tokens = techscript_lexer::lex(&source, &mut reporter).ok();
                                    let lex_dur = lex_start.elapsed();

                                    // Parsing
                                    let mut ast = None;
                                    if let Some(toks) = &tokens {
                                        if let Ok(prog) =
                                            techscript_parser::parse(toks, &mut reporter)
                                        {
                                            ast = Some(prog);
                                        }
                                    }

                                    // Semantic
                                    let mut checked = None;
                                    if let Some(prog) = ast {
                                        if let Ok(chk) =
                                            techscript_semantic::analyze(prog, &mut reporter)
                                        {
                                            checked = Some(chk);
                                        }
                                    }

                                    // Lowering & Code Gen
                                    let mut bytecode = None;
                                    if let Some(chk) = checked {
                                        let lowered = techscript_ir::lower(&chk.program, &pkg_name);
                                        for diag in &lowered.diagnostics {
                                            reporter.report(diag.clone());
                                        }
                                        if !reporter.has_errors() {
                                            let optimized = if opts.profile
                                                == crate::pipeline::BuildProfile::Debug
                                            {
                                                lowered.module
                                            } else {
                                                let opt_ctx =
                                                    techscript_optimizer::OptimizationContext::new(
                                                    );
                                                let mut m = lowered.module;
                                                techscript_optimizer::optimize(&mut m, &opt_ctx);
                                                m
                                            };
                                            bytecode =
                                                Some(techscript_bytecode::compile(&optimized));
                                        }
                                    }

                                    let duration = unit_start.elapsed();
                                    let diags = reporter
                                        .get_diagnostics()
                                        .iter()
                                        .map(|d| {
                                            crate::diagnostics::RichDiagnostic::from_legacy(d, fid)
                                        })
                                        .collect();

                                    CompilationResult {
                                        file_id: fid,
                                        path: path.clone(),
                                        bytecode,
                                        diagnostics: diags,
                                        duration,
                                        from_cache: false,
                                    }
                                }
                                Err(e) => CompilationResult {
                                    file_id: fid,
                                    path: path.clone(),
                                    bytecode: None,
                                    diagnostics: vec![crate::diagnostics::RichDiagnostic::error(
                                        format!("Failed to read file: {}", e),
                                    )],
                                    duration: unit_start.elapsed(),
                                    from_cache: false,
                                },
                            };

                            let is_success = compile_res.bytecode.is_some();

                            // Store result
                            results_clone.lock().unwrap().push(compile_res);

                            // Update unit compilation status
                            {
                                let mut locked_graph = graph_clone.lock().unwrap();
                                if let Some(unit) = locked_graph.units.get_mut(&fid) {
                                    unit.status = if is_success {
                                        CompilationStatus::Compiled
                                    } else {
                                        CompilationStatus::Failed
                                    };
                                }
                            }
                        } else {
                            // Sleep briefly to yield CPU while waiting for dependencies
                            std::thread::sleep(Duration::from_millis(5));
                        }
                    }
                });

                threads.push(handle);
            }
        });

        // 3. Update the passed-in graph with final statuses
        let final_graph = graph_mutex.lock().unwrap();
        *graph = final_graph.clone();

        let final_results = results.lock().unwrap().clone();
        Ok(final_results)
    }
}
