# Graph Report - app  (2026-09-18)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 3158 nodes · 6946 edges · 194 communities (123 shown, 71 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 35 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `ecf43dd7`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- LoweringContext
- Span
- .execute_statement
- .compile_instruction
- ResolveSymbols
- package-manager/src/lib.rs
- IRBuilder
- Interpreter
- RichDiagnostic
- cli/src/pipeline.rs
- packager/src/main.rs
- interpreter_tests.rs
- .parse_statement
- Parser<'a>
- web.rs
- BuildCache
- ProjectBuildGraph
- PassManager
- cli/src/config.rs
- CodegenContext
- migrate.rs
- DiagnosticReporter
- TimingProfiler
- VMDebugger
- CanvasFn
- Vec
- DoctorContext
- .get_word_at_offset
- .parse_expression
- ValueStack
- DocumentFormatter
- DSLSchema
- Backend
- Result
- Option
- graphics.rs
- .emit_to_file
- .new
- BuiltinRegistry
- RefCell
- json.rs
- parse
- compress.rs
- BytecodeBuilder
- file_copy
- .write_to_file
- register_common_functions
- cli/src/main.rs
- lsp_tests.rs
- RuntimeContext
- .register_csv
- bench.rs
- .subtypes
- .register_ftp
- .register_jwt
- .register_mongodb
- .register_notification
- .register_pdf
- bench_diagnostic.rs
- capability_validation_bench.rs
- native_runtime/src/lib.rs
- techscript_runtime
- common_tests.rs
- .register_defaults
- RuntimeValue
- Logger
- ExitCode
- Module
- OptimizationResult
- Diagnostic
- stdlib/src/lib.rs
- .register_module
- SemanticContext
- Value
- CFGAnalysis
- RuntimeError
- techscript_cli
- UserFunction
- SourceFile
- commands/build.rs
- LiteralVal
- parser_tests.rs
- run_src
- serde
- VM
- TypeInterner
- semantic_tests.rs
- techscript_ast
- Op
- Interpreter
- ResourceTable
- SymbolTable
- FileId
- Environment
- optimizer_tests.rs
- repl.rs
- BytecodeModule
- CompilationDatabase
- file_ext.rs
- DefaultModuleResolver
- VMHeap
- emit.rs
- .scan_files
- semantic/src/lib.rs
- vm_tests.rs
- IRVerifier
- diagnosticreporter
- ir_tests.rs
- lexer_tests.rs
- std_database_execute
- optimizer/src/passes/mod.rs
- NativeBridge
- bytecode/src/lib.rs
- .compile_parallel
- .scaffold
- BytecodeValidator
- make_source_file
- OptimizationContext
- VMError
- run_once
- sync.rs
- LivenessAnalysis
- ScriptMutex
- common/src/lib.rs
- disassembler.rs
- Commands
- self_cmd.rs
- find_test_files
- ProgressBar
- .load
- frame.rs
- clean.rs
- commands/docs.rs
- lsp/src/main.rs
- doc.rs
- Theme
- .incoming_calls
- formatter_tests.rs
- .register_database
- verify_all_examples.py
- .register_audio
- .register_binary
- .register_charts
- .register_datetime
- .register_dns
- .register_docs
- .register_email
- .register_encoding
- .register_excel
- .register_http
- .register_image
- .register_ini
- .register_io
- .register_localization
- .register_logging
- .register_mock
- .register_net
- .register_os
- .register_path
- .register_postgres
- .register_powerpoint
- .register_profiler
- .register_qrcode
- .register_regex
- .register_report
- .register_security
- .register_settings
- .register_socket
- .register_strings
- .register_sys
- .register_system
- .register_testing
- .register_theme
- .register_uuid
- .register_video
- .register_word
- techscript_packager

## God Nodes (most connected - your core abstractions)
1. `Span` - 131 edges
2. `NodeId` - 99 edges
3. `RuntimeValue` - 88 edges
4. `Expression` - 60 edges
5. `TsValue` - 53 edges
6. `ExitCode` - 52 edges
7. `RuntimeError` - 47 edges
8. `Backend` - 45 edges
9. `Module` - 45 edges
10. `Ident` - 40 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `NodeId`  [INFERRED]
  runtime/interpreter/examples/interpreter_example.rs → compiler/common/src/node_id.rs
- `test_linter_engine()` --calls--> `NodeId`  [INFERRED]
  tools/linter/tests/linter_tests.rs → compiler/common/src/node_id.rs
- `execute_source()` --calls--> `compile()`  [INFERRED]
  runtime/vm/tests/vm_tests.rs → compiler/bytecode/src/lib.rs
- `compile_to_ir_module()` --references--> `Module`  [EXTRACTED]
  cli/src/commands/emit.rs → compiler/ir/src/module.rs
- `run_src()` --references--> `RuntimeValue`  [EXTRACTED]
  cli/tests/e2e_tests.rs → runtime/runtime/src/value.rs

## Import Cycles
- 1-file cycle: `cli/src/cache.rs -> cli/src/cache.rs`
- 1-file cycle: `cli/src/commands/doc.rs -> cli/src/commands/doc.rs`
- 1-file cycle: `cli/src/pipeline.rs -> cli/src/pipeline.rs`
- 1-file cycle: `tools/packager/src/main.rs -> tools/packager/src/main.rs`
- 1-file cycle: `cli/src/commands/doctor.rs -> cli/src/commands/doctor.rs`
- 1-file cycle: `cli/src/commands/lint.rs -> cli/src/commands/lint.rs`
- 1-file cycle: `cli/src/commands/migrate.rs -> cli/src/commands/migrate.rs`
- 1-file cycle: `cli/src/commands/repl.rs -> cli/src/commands/repl.rs`
- 1-file cycle: `runtime/interpreter/src/expressions.rs -> runtime/interpreter/src/expressions.rs`
- 1-file cycle: `cli/src/config.rs -> cli/src/config.rs`
- 1-file cycle: `cli/src/crash.rs -> cli/src/crash.rs`
- 1-file cycle: `cli/src/diagnostics.rs -> cli/src/diagnostics.rs`
- 1-file cycle: `cli/src/project.rs -> cli/src/project.rs`
- 1-file cycle: `stdlib/src/json.rs -> stdlib/src/json.rs`
- 1-file cycle: `compiler/bytecode/src/disassembler.rs -> compiler/bytecode/src/disassembler.rs`
- 1-file cycle: `runtime/vm/src/debugger.rs -> runtime/vm/src/debugger.rs`
- 1-file cycle: `compiler/ir/benches/builder_bench.rs -> compiler/ir/benches/builder_bench.rs`
- 1-file cycle: `compiler/ir/src/lowering.rs -> compiler/ir/src/lowering.rs`
- 1-file cycle: `compiler/ir/src/builder.rs -> compiler/ir/src/builder.rs`
- 1-file cycle: `compiler/llvm_backend/src/codegen.rs -> compiler/llvm_backend/src/codegen.rs`

## Communities (194 total, 71 thin omitted)

### Community 0 - "LoweringContext"
Cohesion: 0.06
Nodes (43): criterion_benchmark(), Criterion, AssignmentExpr, BinaryExpr, Block, BlockId, ConstDecl, Default (+35 more)

### Community 1 - "Span"
Cohesion: 0.05
Nodes (87): AtomicU32, AskExpr, AssignmentExpr, BinaryExpr, Block, BreakStmt, CallExpr, ConstDecl (+79 more)

### Community 2 - ".execute_statement"
Cohesion: 0.07
Nodes (30): FieldSpec, Block, Callable, ConstDecl, DSLBlock, EnumDecl, ExecResult, ForStmt (+22 more)

### Community 3 - ".compile_instruction"
Cohesion: 0.07
Nodes (39): CodegenContext, BlockId, Function, GlobalId, HashMap, IRType, LiteralVal, LLVMTypeRef (+31 more)

### Community 4 - "ResolveSymbols"
Cohesion: 0.10
Nodes (30): AssignmentExpr, BinaryExpr, Block, CallExpr, ConstDecl, DSLBlock, EnumDecl, Expression (+22 more)

### Community 5 - "package-manager/src/lib.rs"
Cohesion: 0.10
Nodes (30): DependencyConfig, Display, Formatter, LockedPackage, PackageConfig, CapabilityValidator, DependencyConfig, DependencySolver (+22 more)

### Community 6 - "IRBuilder"
Cohesion: 0.08
Nodes (27): BasicBlock, BlockId, Default, DslBlockId, Function, GlobalId, IRType, LiteralVal (+19 more)

### Community 7 - "Interpreter"
Cohesion: 0.10
Nodes (22): AskExpr, EvalResult, ExecResult, FStringExpr, IndexExpr, LambdaExpr, ListExpr, LiteralExpr (+14 more)

### Community 8 - "RichDiagnostic"
Cohesion: 0.11
Nodes (20): Diagnostic, Duration, ErrorCode, FileId, Into, Option, Self, SourceManager (+12 more)

### Community 9 - "cli/src/pipeline.rs"
Cohesion: 0.16
Nodes (21): BytecodeModule, Duration, FileId, Option, Path, PathBuf, Result, RuntimeValue (+13 more)

### Community 10 - "packager/src/main.rs"
Cohesion: 0.18
Nodes (33): AsRef, build_and_copy_tools(), calculate_checksums_json(), compile_binary(), compile_inno_installer(), copy_dir_all(), copy_resources(), count_files() (+25 more)

### Community 11 - "interpreter_tests.rs"
Cohesion: 0.16
Nodes (10): Result, RuntimeError, RuntimeValue, run_source(), test_interpreter_basic_arithmetic(), test_interpreter_dsl_block_produces_value(), test_interpreter_dsl_block_with_code(), test_interpreter_empty() (+2 more)

### Community 12 - ".parse_statement"
Cohesion: 0.21
Nodes (15): Block, DiagnosticReporter, DSLBlock, ForStmt, IfStmt, ImportStmt, ParseResult, RepeatStmt (+7 more)

### Community 13 - "Parser<'a>"
Cohesion: 0.15
Nodes (12): DiagnosticReporter, ErrorCode, HashSet, ParseResult, Self, String, Token, TokenKind (+4 more)

### Community 14 - "web.rs"
Cohesion: 0.15
Nodes (19): DslBlockValue, IpAddr, Resolver, SocketAddr, dsl_to_html(), is_safe_ip(), is_safe_url(), render_card() (+11 more)

### Community 15 - "BuildCache"
Cohesion: 0.17
Nodes (15): BytecodeModule, HashMap, Module, OptimizationLevel, Option, Path, PathBuf, Result (+7 more)

### Community 16 - "ProjectBuildGraph"
Cohesion: 0.19
Nodes (18): FileId, HashMap, HashSet, Option, Path, PathBuf, Result, Self (+10 more)

### Community 17 - "PassManager"
Cohesion: 0.14
Nodes (16): Box, AnalysisManager, Default, Module, OptimizationPass, OptimizationResult, Self, Vec (+8 more)

### Community 18 - "cli/src/config.rs"
Cohesion: 0.17
Nodes (22): Capability, Default, HashSet, OptimizationLevel, Option, Path, PathBuf, Result (+14 more)

### Community 19 - "CodegenContext"
Cohesion: 0.15
Nodes (14): BlockId, Drop, GlobalId, HashMap, LLVMContextRef, LLVMValueRef, LocalId, Option (+6 more)

### Community 20 - "migrate.rs"
Cohesion: 0.14
Nodes (19): ExitCode, Option, ExitCode, Option, String, execute(), execute(), line_transform() (+11 more)

### Community 21 - "DiagnosticReporter"
Cohesion: 0.05
Nodes (38): execute(), Option, DiagnosticReporter, Vec, block_comment(), is_comment_start(), lex(), lex_recovered() (+30 more)

### Community 22 - "TimingProfiler"
Cohesion: 0.15
Nodes (14): Default, Duration, HashMap, Into, Option, Self, String, Vec (+6 more)

### Community 23 - "VMDebugger"
Cohesion: 0.17
Nodes (7): Opcode, BytecodeFunction, Default, HashSet, RuntimeValue, Self, VMDebugger

### Community 25 - "CanvasFn"
Cohesion: 0.12
Nodes (15): CanvasOp, CanvasFn, CanvasOp, dsl_to_svg(), parse_height(), parse_width(), StdlibRegistry, Callable (+7 more)

### Community 26 - "Vec"
Cohesion: 0.14
Nodes (17): CompletionParams, CompletionResponse, InlayHint, InlayHintParams, Mutex, collect_local_decls(), collect_stmt_decls(), dsl_block_completions() (+9 more)

### Community 27 - "DoctorContext"
Cohesion: 0.23
Nodes (8): ExitCode, Option, PathBuf, Self, DoctorContext, execute(), home_dir(), Theme

### Community 28 - ".get_word_at_offset"
Cohesion: 0.13
Nodes (12): CallHierarchyItem, CallHierarchyPrepareParams, GotoTypeDefinitionParams, GotoTypeDefinitionResponse, Hover, HoverParams, Location, Position (+4 more)

### Community 29 - ".parse_expression"
Cohesion: 0.20
Nodes (10): DiagnosticReporter, Expression, ParseResult, Result, String, Token, TokenKind, Precedence (+2 more)

### Community 31 - "ValueStack"
Cohesion: 0.24
Nodes (6): Result, RuntimeValue, Self, Vec, VMError, ValueStack

### Community 32 - "DocumentFormatter"
Cohesion: 0.22
Nodes (9): DocumentFormatter, Formatter, DSLBlock, Expression, LiteralVal, Program, Self, Statement (+1 more)

### Community 33 - "DSLSchema"
Cohesion: 0.38
Nodes (12): HashMap, Self, String, Vec, build_dsl_registry(), DSLSchema, register_canvas_misc_schemas(), register_canvas_schemas() (+4 more)

### Community 34 - "Backend"
Cohesion: 0.15
Nodes (13): CodeLens, CodeLensParams, DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentSymbolParams, DocumentSymbolResponse, Range (+5 more)

### Community 35 - "Result"
Cohesion: 0.21
Nodes (7): DocumentFormattingParams, DocumentOnTypeFormattingParams, DocumentRangeFormattingParams, InitializeParams, InitializeResult, TextEdit, Result

### Community 36 - "Option"
Cohesion: 0.14
Nodes (11): CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, GotoDeclarationParams, GotoDeclarationResponse, GotoDefinitionParams, GotoDefinitionResponse, GotoImplementationParams, GotoImplementationResponse (+3 more)

### Community 37 - "graphics.rs"
Cohesion: 0.42
Nodes (15): ImageFormat, Rgba, create_canvas(), draw_circle(), draw_line(), draw_rect(), parse_color(), save_image_with_format() (+7 more)

### Community 38 - ".emit_to_file"
Cohesion: 0.34
Nodes (10): Module, Path, Result, String, LLVMCodeGenFileType, get_host_target_triple(), LLVMBackend, LLVMBackendOptions (+2 more)

### Community 39 - ".new"
Cohesion: 0.13
Nodes (10): Client, CodeActionParams, CodeActionResponse, FoldingRange, FoldingRangeParams, SelectionRange, SelectionRangeParams, SemanticTokensParams (+2 more)

### Community 40 - "BuiltinRegistry"
Cohesion: 0.20
Nodes (8): NativeCallback, HashMap, Result, RuntimeError, Self, String, Value, BuiltinRegistry

### Community 41 - "RefCell"
Cohesion: 0.12
Nodes (9): AstVisitor, AtomicI64, flowsignal, IndexMap, RefCell, runtimeerrorkind, NEXT_ID, tosocketaddrs (+1 more)

### Community 42 - "json.rs"
Cohesion: 0.21
Nodes (9): parse_json_value(), StdlibRegistry, stringify_value(), test_stringify_error(), Result, RuntimeError, RuntimeValue, String (+1 more)

### Community 43 - "parse"
Cohesion: 0.30
Nodes (9): Diagnostic, DiagnosticReporter, Program, Result, Token, Vec, parse(), parse_recovered() (+1 more)

### Community 44 - "compress.rs"
Cohesion: 0.35
Nodes (10): gunzip_archive(), gzip_file(), StdlibRegistry, tar_dir(), test_unzip_archive_corrupted_zip(), test_unzip_archive_invalid_path(), untar_archive(), unzip_archive() (+2 more)

### Community 45 - "BytecodeBuilder"
Cohesion: 0.05
Nodes (37): ArtifactCategory, ArtifactManager, BuildManifest, BuildOutput, Path, PathBuf, Result, Self (+29 more)

### Community 46 - "file_copy"
Cohesion: 0.55
Nodes (10): file_copy(), file_exists(), file_read(), file_remove(), file_write(), Result, RuntimeContext, RuntimeError (+2 more)

### Community 47 - ".write_to_file"
Cohesion: 0.31
Nodes (7): Option, Path, PathBuf, Result, String, CrashReport, install_panic_hook()

### Community 48 - "register_common_functions"
Cohesion: 0.47
Nodes (8): register_common_functions(), register_list_functions(), register_map_functions(), StdlibRegistry, Callable, HashMap, Rc, String

### Community 49 - "cli/src/main.rs"
Cohesion: 0.52
Nodes (6): main(), levenshtein(), perform_first_run_check(), show_custom_help(), show_welcome_screen(), suggest_subcommand()

### Community 51 - "RuntimeContext"
Cohesion: 0.06
Nodes (26): Item, Iterator, Rc, RefCell, RuntimeContext, Callable, AskNative, AssertNative (+18 more)

### Community 52 - ".register_csv"
Cohesion: 0.47
Nodes (4): StdlibRegistry, test_csv_parse(), test_csv_register(), test_csv_stringify()

### Community 53 - "bench.rs"
Cohesion: 0.70
Nodes (4): criterion_benchmark(), find_boundary_fast(), find_boundary_slow(), Criterion

### Community 54 - ".subtypes"
Cohesion: 0.40
Nodes (3): TypeHierarchyItem, TypeHierarchySubtypesParams, TypeHierarchySupertypesParams

### Community 62 - "native_runtime/src/lib.rs"
Cohesion: 0.09
Nodes (65): c_char, c_int, ffi, raw, JmpBuf, longjmp(), HashMap, String (+57 more)

### Community 63 - "techscript_runtime"
Cohesion: 0.14
Nodes (5): crate, hashmap, rc, tcpstream, techscript_runtime

### Community 64 - "common_tests.rs"
Cohesion: 0.04
Nodes (6): backward_compat_ast_reexport_pattern(), backward_compat_original_api(), node_id_construction(), node_id_display(), node_id_equality_and_hash(), node_id_serde_roundtrip()

### Community 66 - ".register_defaults"
Cohesion: 0.07
Nodes (14): StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry (+6 more)

### Community 67 - "RuntimeValue"
Cohesion: 0.08
Nodes (29): AtomicU64, PartialEq, ModelInstance, NEXT_OBJECT_ID, ObjectId, IndexMap, Self, String (+21 more)

### Community 68 - "Logger"
Cohesion: 0.07
Nodes (25): CompilationEvent, EventBus, EventListener, Box, Default, Duration, Path, Self (+17 more)

### Community 69 - "ExitCode"
Cohesion: 0.07
Nodes (28): execute(), execute(), default_toml_config(), execute(), Option, dump_ast(), dump_bytecode(), dump_ir() (+20 more)

### Community 70 - "Module"
Cohesion: 0.09
Nodes (18): DslBlockIR, Module, Option, Self, String, Vec, DslBlockId, FunctionId (+10 more)

### Community 71 - "OptimizationResult"
Cohesion: 0.08
Nodes (21): optimize(), BranchSimplification, OptimizationPipeline, OptimizationResult, Self, PassStatistics, Self, String (+13 more)

### Community 72 - "Diagnostic"
Cohesion: 0.12
Nodes (15): Diagnostic, DiagnosticLevel, Option, Self, String, CheckedProgram, DslBlockLintRule, DslNamingConventionRule (+7 more)

### Community 73 - "stdlib/src/lib.rs"
Cohesion: 0.09
Nodes (21): FnOnce, StdFnCallback, AsyncTask, MockFunction, Box, Default, F, HashMap (+13 more)

### Community 74 - ".register_module"
Cohesion: 0.06
Nodes (17): StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry (+9 more)

### Community 77 - "SemanticContext"
Cohesion: 0.10
Nodes (20): build_dsl_registry, Default, HashMap, Option, Self, String, Vec, SemanticContext (+12 more)

### Community 78 - "Value"
Cohesion: 0.12
Nodes (15): builder, BytecodeLowerer, HashMap, Self, String, GlobalId, LocalId, ValueId (+7 more)

### Community 79 - "CFGAnalysis"
Cohesion: 0.09
Nodes (17): collections, CFGAnalysis, HashMap, HashSet, Self, Vec, DominatorAnalysis, HashMap (+9 more)

### Community 82 - "RuntimeError"
Cohesion: 0.13
Nodes (21): ErrorCode, FlowSignal, eval_binary(), eval_unary(), Result, list_get(), list_set(), map_get() (+13 more)

### Community 83 - "techscript_cli"
Cohesion: 0.34
Nodes (24): techscript_ast, techscript_builtins, techscript_bytecode, techscript_cli, techscript_common, techscript_errors, techscript_formatter, techscript_gc (+16 more)

### Community 84 - "UserFunction"
Cohesion: 0.13
Nodes (12): BridgedFunction, Interpreter, Option, Result, Vec, FunctionBody, Rc, RefCell (+4 more)

### Community 85 - "SourceFile"
Cohesion: 0.17
Nodes (9): Arc, Option, Path, PathBuf, Self, String, Vec, SourceFile (+1 more)

### Community 86 - "commands/build.rs"
Cohesion: 0.19
Nodes (18): ArtifactManager, BuildCache, execute(), invoke_linker(), Error, Option, Path, Result (+10 more)

### Community 87 - "LiteralVal"
Cohesion: 0.15
Nodes (9): LiteralVal, ConstantPool, Option, Self, Vec, ConstantFolding, Option, ConstantPropagation (+1 more)

### Community 88 - "parser_tests.rs"
Cohesion: 0.17
Nodes (20): parse_source(), test_dsl_block_deeply_nested(), test_dsl_block_empty(), test_dsl_block_no_args(), test_dsl_block_with_inline_code(), test_dsl_block_with_nested_sub_blocks(), test_dsl_block_with_properties(), test_dsl_block_with_property_no_value() (+12 more)

### Community 89 - "run_src"
Cohesion: 0.15
Nodes (19): Result, String, Vec, run_src(), test_e2e_calculator(), test_e2e_collections_lists_and_maps(), test_e2e_fs_sandboxing_denied(), test_e2e_fs_sandboxing_granted() (+11 more)

### Community 90 - "serde"
Cohesion: 0.13
Nodes (13): Function, HashMap, Self, String, Vec, HashMap, IRType, Display (+5 more)

### Community 91 - "VM"
Cohesion: 0.15
Nodes (11): bytecodeloader, Self, VMProfiler, CallFrame, HashMap, Self, String, Vec (+3 more)

### Community 92 - "TypeInterner"
Cohesion: 0.21
Nodes (8): Default, HashMap, Self, String, Vec, Type, TypeId, TypeInterner

### Community 93 - "semantic_tests.rs"
Cohesion: 0.18
Nodes (19): check_source(), Result, Vec, test_dsl_semantic_duplicate_property_error(), test_dsl_semantic_full_web_page(), test_dsl_semantic_invalid_nested_block_warning(), test_dsl_semantic_missing_required_property(), test_dsl_semantic_nested_valid_block() (+11 more)

### Community 94 - "techscript_ast"
Cohesion: 0.12
Nodes (12): CheckedProgram, main(), symbol, symboltable, techscript_ast, techscript_common, techscript_errors, techscript_linter (+4 more)

### Community 95 - "Op"
Cohesion: 0.19
Nodes (15): BasicBlock, Option, Self, String, Vec, Instruction, InstructionMetadata, Op (+7 more)

### Community 96 - "Interpreter"
Cohesion: 0.16
Nodes (12): ExecResult, Statement, Interpreter, CallFrame, Default, F, Rc, RefCell (+4 more)

### Community 97 - "ResourceTable"
Cohesion: 0.20
Nodes (10): Any, ResourceTable, Box, Default, HashMap, HashSet, Option, Self (+2 more)

### Community 98 - "SymbolTable"
Cohesion: 0.20
Nodes (8): HashMap, Option, Self, String, Vec, Scope, Symbol, SymbolTable

### Community 99 - "FileId"
Cohesion: 0.15
Nodes (13): FileId, Position, Display, Formatter, Result, file_id_construction(), file_id_hash(), file_id_serde_roundtrip() (+5 more)

### Community 100 - "Environment"
Cohesion: 0.24
Nodes (10): error, Binding, Environment, HashMap, Option, Rc, RefCell, Result (+2 more)

### Community 101 - "optimizer_tests.rs"
Cohesion: 0.22
Nodes (14): analyze, compile_source(), test_bytecode_compilation_and_validation(), test_bytecode_constant_deduplication(), test_bytecode_disassembler(), test_bytecode_serialization_roundtrip(), optimize_source(), test_optimizer_algebraic_simplification() (+6 more)

### Community 102 - "repl.rs"
Cohesion: 0.26
Nodes (14): ExitCode, Option, PathBuf, String, dump_repl_ast(), dump_repl_bytecode(), dump_repl_ir(), eval_code() (+6 more)

### Community 103 - "BytecodeModule"
Cohesion: 0.19
Nodes (10): BytecodeModule, Self, String, Vec, BytecodeSerializer, BytecodeVersion, Result, Self (+2 more)

### Community 104 - "CompilationDatabase"
Cohesion: 0.21
Nodes (9): CompilationDatabase, CompileCommand, Default, Path, PathBuf, Result, Self, String (+1 more)

### Community 105 - "file_ext.rs"
Cohesion: 0.20
Nodes (12): CommonError, is_techscript_file(), Display, Error, Formatter, Path, Result, String (+4 more)

### Community 106 - "DefaultModuleResolver"
Cohesion: 0.25
Nodes (9): DefaultModuleResolver, ModuleResolver, ModuleSource, Default, HashSet, PathBuf, Result, Self (+1 more)

### Community 107 - "VMHeap"
Cohesion: 0.21
Nodes (6): GarbageCollector, HeapObject, Rc, Self, Vec, VMHeap

### Community 108 - "emit.rs"
Cohesion: 0.21
Nodes (11): main(), Box, Error, Result, compile_to_ir_module(), emit_asm(), emit_ir(), emit_llvm() (+3 more)

### Community 109 - ".scan_files"
Cohesion: 0.26
Nodes (8): FileWatcher, F, HashMap, Path, PathBuf, Result, Self, SystemTime

### Community 110 - "semantic/src/lib.rs"
Cohesion: 0.23
Nodes (9): collectdecls, analyze(), Result, Self, Vec, SemanticAnalyzer, passpipeline, resolvesymbols (+1 more)

### Community 111 - "vm_tests.rs"
Cohesion: 0.27
Nodes (11): compile, execute_source(), Result, test_vm_arithmetic(), test_vm_conditions(), test_vm_division_by_zero(), test_vm_functions(), test_vm_native_call() (+3 more)

### Community 112 - "IRVerifier"
Cohesion: 0.30
Nodes (6): IRVerifier, Default, HashSet, Result, Self, String

### Community 113 - "diagnosticreporter"
Cohesion: 0.18
Nodes (3): diagnosticreporter, parser, techscript_cli

### Community 114 - "ir_tests.rs"
Cohesion: 0.35
Nodes (10): lower_source(), test_ir_basic_arithmetic(), test_ir_dsl_block_empty(), test_ir_dsl_block_nested(), test_ir_dsl_block_with_properties(), test_ir_empty(), test_ir_if_conditional_branching(), test_ir_null_coalescing_lower() (+2 more)

### Community 116 - "std_database_execute"
Cohesion: 0.25
Nodes (10): Result, Vec, SqlParam, SqlParam<'a>, std_database_close(), std_database_connect(), std_database_execute(), std_database_query() (+2 more)

### Community 117 - "optimizer/src/passes/mod.rs"
Cohesion: 0.20
Nodes (9): algebraicsimplification, branchsimplification, cfgcleanup, constantfolding, constantpropagation, copypropagation, deadcode, deadstore (+1 more)

### Community 118 - "NativeBridge"
Cohesion: 0.27
Nodes (5): BuiltinRegistry, NativeBridge, Default, Result, Self

### Community 119 - "bytecode/src/lib.rs"
Cohesion: 0.20
Nodes (9): bytecodedisassembler, bytecodefunction, bytecodeinstruction, bytecodelowerer, bytecodevalidator, compile(), constantpool, debugsymbols (+1 more)

### Community 120 - ".compile_parallel"
Cohesion: 0.22
Nodes (8): CompilationScheduler, Result, Self, Vec, CompilationPipeline, CompilationResult, PipelineOptions, ProjectBuildGraph

### Community 121 - ".scaffold"
Cohesion: 0.22
Nodes (7): ProjectTemplate, Option, Path, PathBuf, Result, Self, Vec

### Community 122 - "BytecodeValidator"
Cohesion: 0.24
Nodes (6): BytecodeValidator, Default, Result, Self, String, Operand

### Community 123 - "make_source_file"
Cohesion: 0.22
Nodes (9): make_source_file(), source_file_empty(), source_file_line_col_out_of_bounds(), source_file_line_content_out_of_bounds(), source_file_line_start(), source_file_multi_line(), source_file_single_line(), source_file_unicode() (+1 more)

### Community 124 - "OptimizationContext"
Cohesion: 0.31
Nodes (5): OptimizationContext, OptimizationLevel, Default, Self, Self

### Community 125 - "VMError"
Cohesion: 0.25
Nodes (7): String, VMError, Result, VM, Result, run(), Result

### Community 127 - "run_once"
Cohesion: 0.25
Nodes (8): BuildProfile, execute(), Error, Option, Path, Result, run_once(), ExecutionBackend

### Community 129 - "LivenessAnalysis"
Cohesion: 0.29
Nodes (5): LivenessAnalysis, HashMap, HashSet, Self, Vec

### Community 130 - "ScriptMutex"
Cohesion: 0.36
Nodes (4): Condvar, Self, ScriptMutex, StdlibRegistry

### Community 131 - "common/src/lib.rs"
Cohesion: 0.29
Nodes (4): atomic, MAX_RECURSION_DEPTH, MAX_SOURCE_FILE_SIZE, TECHSCRIPT_VERSION

### Community 132 - "disassembler.rs"
Cohesion: 0.48
Nodes (4): BytecodeFunction, BytecodeModule, String, BytecodeDisassembler

### Community 134 - "Commands"
Cohesion: 0.47
Nodes (5): clap, Cli, Commands, Option, String

### Community 135 - "self_cmd.rs"
Cohesion: 0.33
Nodes (5): execute(), Option, String, VersionCheckResponse, deserialize

### Community 136 - "find_test_files"
Cohesion: 0.33
Nodes (6): execute(), find_test_files(), Option, Path, PathBuf, Vec

### Community 138 - ".load"
Cohesion: 0.33
Nodes (4): BytecodeLoader, Result, String, techscript_bytecode

### Community 139 - "frame.rs"
Cohesion: 0.40
Nodes (4): CallFrame, ExceptionHandler, Self, Vec

### Community 140 - "clean.rs"
Cohesion: 0.50
Nodes (4): execute(), home_dir(), Option, PathBuf

### Community 141 - "commands/docs.rs"
Cohesion: 0.50
Nodes (4): execute(), open_browser(), Option, Result

### Community 143 - "doc.rs"
Cohesion: 0.67
Nodes (3): ExitCode, Option, execute()

## Knowledge Gaps
- **359 isolated node(s):** `PathBuf`, `Self`, `Option`, `Theme`, `Self` (+354 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 911 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **71 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `RefCell` connect `RefCell` to `sync.rs`, `runtime_tests.rs`, `graphics.rs`, `stdlib/src/lib.rs`, `json.rs`, `.register_csv`, `stdlib_tests.rs`, `CanvasFn`, `native_runtime/src/lib.rs`, `techscript_runtime`?**
  _High betweenness centrality (0.101) - this node is a cross-community bridge._
- **Why does `Write` connect `RefCell` to `DocumentFormatter`, `disassembler.rs`, `BuiltinRegistry`, `ProgressBar`, `RichDiagnostic`, `packager/src/main.rs`, `web.rs`, `doc.rs`, `.write_to_file`, `migrate.rs`, `TimingProfiler`, `VMDebugger`, `CanvasFn`?**
  _High betweenness centrality (0.081) - this node is a cross-community bridge._
- **Why does `Span` connect `Span` to `Diagnostic`, `RefCell`, `BytecodeBuilder`, `Value`, `RuntimeError`, `SourceFile`, `DiagnosticReporter`, `techscript_ast`, `Op`?**
  _High betweenness centrality (0.072) - this node is a cross-community bridge._
- **Are the 13 inferred relationships involving `NodeId` (e.g. with `test_ast_assignment_expression()` and `test_ast_construction_and_equality()`) actually correct?**
  _`NodeId` has 13 INFERRED edges - model-reasoned connections that need verification._
- **What connects `PathBuf`, `Self`, `Option` to the rest of the system?**
  _359 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `LoweringContext` be split into smaller, more focused modules?**
  _Cohesion score 0.057971014492753624 - nodes in this community are weakly interconnected._
- **Should `Span` be split into smaller, more focused modules?**
  _Cohesion score 0.0514453699167075 - nodes in this community are weakly interconnected._