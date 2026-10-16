# Graph Report - app  (2026-10-16)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 3340 nodes · 7288 edges · 271 communities (112 shown, 159 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 35 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `62c22252`
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
- stdlib_tests.rs
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
- crypto.rs
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
- diagnosticreporter
- ir_tests.rs
- NativeBridge
- bytecode/src/lib.rs
- .compile_parallel
- .scaffold
- BytecodeValidator
- make_source_file
- OptimizationContext
- run_once
- sync.rs
- common/src/lib.rs
- disassembler.rs
- Commands
- self_cmd.rs
- find_test_files
- ProgressBar
- .load
- frame.rs
- commands/docs.rs
- doc.rs
- Theme
- .incoming_calls
- .register_database
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
- Token
- DummyGC
- database.rs
- env.rs
- net_handler.rs
- linter_tests.rs
- VMProfiler
- DebugSymbols
- DiagnosticStats
- .propagate_in_op
- CallFrame
- operations.rs
- LintRule
- SqlParam<'a>
- .register_http
- SqlParam<'a>
- clean.rs
- .register_database
- .register_notification
- .register_config
- .register_gui
- .register_math
- .register_mysql
- .register_net
- .register_oauth
- .register_random
- .register_redis
- .register_sqlite
- .register_strings
- .register_svg
- .register_sys
- .register_task
- .register_thread
- .register_time
- .register_url
- .register_web
- .register_websocket
- .register_xml
- techscript_packager

## God Nodes (most connected - your core abstractions)
1. `Span` - 128 edges
2. `NodeId` - 99 edges
3. `RuntimeValue` - 71 edges
4. `Expression` - 60 edges
5. `TsValue` - 54 edges
6. `ExitCode` - 50 edges
7. `LogosToken` - 50 edges
8. `Backend` - 45 edges
9. `Module` - 43 edges
10. `Ident` - 40 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `NodeId`  [INFERRED]
  runtime/interpreter/examples/interpreter_example.rs → compiler/common/src/node_id.rs
- `test_linter_engine()` --calls--> `NodeId`  [INFERRED]
  tools/linter/tests/linter_tests.rs → compiler/common/src/node_id.rs
- `execute_source()` --calls--> `compile()`  [INFERRED]
  runtime/vm/tests/vm_tests.rs → compiler/bytecode/src/lib.rs
- `FunctionBody` --references--> `Block`  [EXTRACTED]
  runtime/runtime/src/function.rs → compiler/ast/src/lib.rs
- `BridgedFunction` --references--> `Expression`  [EXTRACTED]
  runtime/interpreter/src/functions.rs → compiler/ast/src/lib.rs

## Import Cycles
- 1-file cycle: `compiler/ir/benches/builder_bench.rs -> compiler/ir/benches/builder_bench.rs`
- 1-file cycle: `compiler/ir/src/lowering.rs -> compiler/ir/src/lowering.rs`
- 1-file cycle: `tools/packager/src/main.rs -> tools/packager/src/main.rs`
- 1-file cycle: `runtime/vm/src/debugger.rs -> runtime/vm/src/debugger.rs`
- 1-file cycle: `compiler/bytecode/src/disassembler.rs -> compiler/bytecode/src/disassembler.rs`
- 1-file cycle: `compiler/parser/src/parser.rs -> compiler/parser/src/parser.rs`
- 1-file cycle: `cli/src/commands/doc.rs -> cli/src/commands/doc.rs`
- 1-file cycle: `cli/src/pipeline.rs -> cli/src/pipeline.rs`
- 1-file cycle: `cli/src/cache.rs -> cli/src/cache.rs`
- 1-file cycle: `compiler/optimizer/src/pass_manager.rs -> compiler/optimizer/src/pass_manager.rs`
- 1-file cycle: `compiler/optimizer/src/passes/dead_code.rs -> compiler/optimizer/src/passes/dead_code.rs`
- 1-file cycle: `compiler/llvm_backend/src/context.rs -> compiler/llvm_backend/src/context.rs`
- 1-file cycle: `cli/src/commands/migrate.rs -> cli/src/commands/migrate.rs`
- 1-file cycle: `tools/lsp/src/lib.rs -> tools/lsp/src/lib.rs`
- 1-file cycle: `compiler/llvm_backend/src/jit.rs -> compiler/llvm_backend/src/jit.rs`
- 1-file cycle: `compiler/llvm_backend/src/type_map.rs -> compiler/llvm_backend/src/type_map.rs`
- 1-file cycle: `runtime/vm/src/stack.rs -> runtime/vm/src/stack.rs`
- 1-file cycle: `compiler/semantic/src/dsl_schema.rs -> compiler/semantic/src/dsl_schema.rs`
- 1-file cycle: `compiler/llvm_backend/src/lib.rs -> compiler/llvm_backend/src/lib.rs`
- 1-file cycle: `compiler/semantic/src/passes/resolve.rs -> compiler/semantic/src/passes/resolve.rs`

## Communities (271 total, 159 thin omitted)

### Community 0 - "LoweringContext"
Cohesion: 0.06
Nodes (43): criterion_benchmark(), Criterion, AssignmentExpr, BinaryExpr, Block, BlockId, ConstDecl, Default (+35 more)

### Community 1 - "Span"
Cohesion: 0.12
Nodes (30): AskExpr, AssignmentExpr, BinaryExpr, BreakStmt, CallExpr, DSLBlock, DSLChild, DSLProperty (+22 more)

### Community 2 - ".execute_statement"
Cohesion: 0.07
Nodes (30): FieldSpec, Block, Callable, ConstDecl, DSLBlock, EnumDecl, ExecResult, ForStmt (+22 more)

### Community 3 - ".compile_instruction"
Cohesion: 0.19
Nodes (10): Drop, HashMap, Module, Result, Self, String, LLVMBackendOptions, LLVMOrcLLJITRef (+2 more)

### Community 4 - "ResolveSymbols"
Cohesion: 0.10
Nodes (30): AssignmentExpr, BinaryExpr, Block, CallExpr, ConstDecl, DSLBlock, EnumDecl, Expression (+22 more)

### Community 5 - "package-manager/src/lib.rs"
Cohesion: 0.06
Nodes (42): CompilationStatus, Cached, Compiled, Compiling, Failed, Pending, CompilationUnit, load_package() (+34 more)

### Community 6 - "IRBuilder"
Cohesion: 0.08
Nodes (23): BasicBlock, BlockId, Default, DslBlockId, Function, GlobalId, IRType, LiteralVal (+15 more)

### Community 7 - "Interpreter"
Cohesion: 0.10
Nodes (20): AskExpr, FStringExpr, IndexExpr, LambdaExpr, ListExpr, LiteralExpr, MapExpr, MemberExpr (+12 more)

### Community 8 - "RichDiagnostic"
Cohesion: 0.07
Nodes (69): JmpBuf, longjmp(), setjmp(), ts_add(), ts_alloc_bool(), ts_alloc_enum(), ts_alloc_float(), ts_alloc_int() (+61 more)

### Community 9 - "cli/src/pipeline.rs"
Cohesion: 0.13
Nodes (26): ArtifactManager, BuildCache, execute(), BytecodeModule, Duration, FileId, Option, Path (+18 more)

### Community 10 - "packager/src/main.rs"
Cohesion: 0.18
Nodes (33): AsRef, build_and_copy_tools(), calculate_checksums_json(), compile_binary(), compile_inno_installer(), copy_dir_all(), copy_resources(), count_files() (+25 more)

### Community 11 - "interpreter_tests.rs"
Cohesion: 0.16
Nodes (10): Result, RuntimeError, RuntimeValue, run_source(), test_interpreter_basic_arithmetic(), test_interpreter_dsl_block_produces_value(), test_interpreter_dsl_block_with_code(), test_interpreter_empty() (+2 more)

### Community 12 - ".parse_statement"
Cohesion: 0.22
Nodes (14): Block, DiagnosticReporter, DSLBlock, ForStmt, IfStmt, ImportStmt, ParseResult, RepeatStmt (+6 more)

### Community 13 - "Parser<'a>"
Cohesion: 0.15
Nodes (12): DiagnosticReporter, ErrorCode, HashSet, ParseResult, Self, String, Token, TokenKind (+4 more)

### Community 14 - "web.rs"
Cohesion: 0.04
Nodes (48): LogosToken, Arrow, Bang, BangEqual, BangEqualEqual, BlockComment, Colon, Comma (+40 more)

### Community 15 - "BuildCache"
Cohesion: 0.17
Nodes (15): BytecodeModule, HashMap, Module, OptimizationLevel, Option, Path, PathBuf, Result (+7 more)

### Community 16 - "ProjectBuildGraph"
Cohesion: 0.11
Nodes (22): DslBlockValue, dsl_to_html(), dsl_to_html_inner(), is_safe_ip(), is_safe_url(), PAGE_CONTENT, render_card(), render_children() (+14 more)

### Community 17 - "PassManager"
Cohesion: 0.14
Nodes (16): Box, AnalysisManager, Default, Module, OptimizationPass, OptimizationResult, Self, Vec (+8 more)

### Community 18 - "cli/src/config.rs"
Cohesion: 0.11
Nodes (12): Block, ContinueStmt, ExportDecl, ForStmt, IfStmt, LambdaExpr, Program, RepeatStmt (+4 more)

### Community 19 - "CodegenContext"
Cohesion: 0.06
Nodes (23): CodegenEngine, CodegenEngine<'a>, TypeInfo, Value, BlockId, Drop, GlobalId, HashMap (+15 more)

### Community 20 - "migrate.rs"
Cohesion: 0.09
Nodes (19): DoctorContext, execute(), execute(), ExitCode, Option, String, Theme, execute() (+11 more)

### Community 22 - "TimingProfiler"
Cohesion: 0.15
Nodes (14): Default, Duration, HashMap, Into, Option, Self, String, Vec (+6 more)

### Community 23 - "VMDebugger"
Cohesion: 0.17
Nodes (7): Opcode, BytecodeFunction, Default, HashSet, RuntimeValue, Self, VMDebugger

### Community 24 - "stdlib_tests.rs"
Cohesion: 0.19
Nodes (15): ConstDecl, EnumDecl, EnumVariant, FieldSpec, FuncDecl, ImportStmt, MethodDecl, MethodKeyword (+7 more)

### Community 26 - "Vec"
Cohesion: 0.14
Nodes (16): CodeLens, CodeLensParams, CompletionParams, CompletionResponse, collect_local_decls(), collect_stmt_decls(), dsl_block_completions(), is_keyword_kind() (+8 more)

### Community 27 - "DoctorContext"
Cohesion: 0.34
Nodes (24): techscript_ast, techscript_builtins, techscript_bytecode, techscript_cli, techscript_common, techscript_errors, techscript_formatter, techscript_gc (+16 more)

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
Nodes (13): DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, DocumentSymbolParams, DocumentSymbolResponse, InlayHint, InlayHintParams, Range (+5 more)

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
Cohesion: 0.10
Nodes (9): AstVisitor, StdFunction, StdlibModule, StdlibRegistry, NEXT_ID, SqlParam, parse_toml_value(), to_toml_value() (+1 more)

### Community 42 - "json.rs"
Cohesion: 0.22
Nodes (6): block_comment(), is_comment_start(), lex(), lex_recovered(), Lexer, Lexer<'a>

### Community 43 - "parse"
Cohesion: 0.30
Nodes (9): Diagnostic, DiagnosticReporter, Program, Result, Token, Vec, parse(), parse_recovered() (+1 more)

### Community 44 - "compress.rs"
Cohesion: 0.17
Nodes (5): CheckedProgram, DSLChild, interpret(), DslBlockLintRule, DslNamingConventionRule

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
Cohesion: 0.21
Nodes (16): compress_gunzip(), compress_gzip(), compress_tar(), compress_untar(), compress_unzip(), compress_zip(), gunzip_archive(), gzip_file() (+8 more)

### Community 51 - "RuntimeContext"
Cohesion: 0.06
Nodes (13): Callable, AskNative, AssertNative, ExitNative, LenNative, NativeRegistry, RangeNative, SayNative (+5 more)

### Community 52 - ".register_csv"
Cohesion: 0.47
Nodes (4): StdlibRegistry, test_csv_parse(), test_csv_register(), test_csv_stringify()

### Community 53 - "bench.rs"
Cohesion: 0.70
Nodes (4): criterion_benchmark(), find_boundary_fast(), find_boundary_slow(), Criterion

### Community 54 - ".subtypes"
Cohesion: 0.40
Nodes (3): TypeHierarchyItem, TypeHierarchySubtypesParams, TypeHierarchySupertypesParams

### Community 58 - ".register_notification"
Cohesion: 0.19
Nodes (7): build_canvas_api(), CanvasFn, dsl_to_svg(), parse_height(), parse_width(), render_dsl_to_svg(), StdlibRegistry

### Community 64 - "common_tests.rs"
Cohesion: 0.04
Nodes (6): backward_compat_ast_reexport_pattern(), backward_compat_original_api(), node_id_construction(), node_id_display(), node_id_equality_and_hash(), node_id_serde_roundtrip()

### Community 66 - ".register_defaults"
Cohesion: 0.08
Nodes (12): StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry (+4 more)

### Community 67 - "RuntimeValue"
Cohesion: 0.13
Nodes (3): DslProperty, RuntimeType, RuntimeValue

### Community 68 - "Logger"
Cohesion: 0.19
Nodes (4): format_duration(), LogFormat, Logger, StageStatus

### Community 69 - "ExitCode"
Cohesion: 0.08
Nodes (17): execute(), default_toml_config(), execute(), execute(), open_browser(), dump_ast(), dump_bytecode(), dump_ir() (+9 more)

### Community 70 - "Module"
Cohesion: 0.08
Nodes (14): Module, optimize(), OptimizationPass, AlgebraicSimplification, BranchSimplification, Canonicalize, CFGCleanup, ConstantFolding (+6 more)

### Community 71 - "OptimizationResult"
Cohesion: 0.13
Nodes (3): OptimizationContext, OptimizationLevel, OptimizationPipeline

### Community 72 - "Diagnostic"
Cohesion: 0.27
Nodes (3): Diagnostic, DiagnosticLevel, ErrorCode

### Community 73 - "stdlib/src/lib.rs"
Cohesion: 0.18
Nodes (4): AsyncTask, MockFunction, Scheduler, spawn_task()

### Community 74 - ".register_module"
Cohesion: 0.06
Nodes (15): StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry, StdlibRegistry (+7 more)

### Community 78 - "Value"
Cohesion: 0.11
Nodes (11): BytecodeLowerer, Instruction, InstructionMetadata, Op, Terminator, TerminatorKind, GlobalId, IRType (+3 more)

### Community 79 - "CFGAnalysis"
Cohesion: 0.09
Nodes (5): Function, FunctionId, DominatorAnalysis, AnalysisManager, UseDefAnalysis

### Community 80 - "crypto.rs"
Cohesion: 0.23
Nodes (4): DiagnosticRenderer<'a>, FixSuggestion, RichDiagnostic, SpanLabel

### Community 82 - "RuntimeError"
Cohesion: 0.13
Nodes (8): FlowSignal, list_get(), list_set(), map_get(), map_set(), RuntimeError, RuntimeErrorKind, test_list_and_map_mutations()

### Community 84 - "UserFunction"
Cohesion: 0.13
Nodes (4): BridgedFunction, Interpreter, FunctionBody, UserFunction

### Community 86 - "commands/build.rs"
Cohesion: 0.11
Nodes (7): main(), execute(), execute(), invoke_linker(), execute(), find_test_files(), Instant

### Community 87 - "LiteralVal"
Cohesion: 0.13
Nodes (4): LiteralVal, ConstantPool, DslBlockIR, DslBlockId

### Community 88 - "parser_tests.rs"
Cohesion: 0.17
Nodes (20): parse_source(), test_dsl_block_deeply_nested(), test_dsl_block_empty(), test_dsl_block_no_args(), test_dsl_block_with_inline_code(), test_dsl_block_with_nested_sub_blocks(), test_dsl_block_with_properties(), test_dsl_block_with_property_no_value() (+12 more)

### Community 89 - "run_src"
Cohesion: 0.15
Nodes (13): run_src(), test_e2e_calculator(), test_e2e_collections_lists_and_maps(), test_e2e_fs_sandboxing_denied(), test_e2e_fs_sandboxing_granted(), test_e2e_hello_world(), test_e2e_json_parsing(), test_e2e_recursive_fibonacci() (+5 more)

### Community 90 - "serde"
Cohesion: 0.22
Nodes (5): helper_aes_decrypt(), helper_aes_encrypt(), helper_bcrypt_hash(), helper_bcrypt_verify(), StdlibRegistry

### Community 91 - "VM"
Cohesion: 0.17
Nodes (3): VMError, run(), VM

### Community 92 - "TypeInterner"
Cohesion: 0.21
Nodes (3): Type, TypeId, TypeInterner

### Community 93 - "semantic_tests.rs"
Cohesion: 0.18
Nodes (17): check_source(), test_dsl_semantic_duplicate_property_error(), test_dsl_semantic_full_web_page(), test_dsl_semantic_invalid_nested_block_warning(), test_dsl_semantic_missing_required_property(), test_dsl_semantic_nested_valid_block(), test_dsl_semantic_unknown_property_warning(), test_dsl_semantic_valid_block() (+9 more)

### Community 94 - "techscript_ast"
Cohesion: 0.12
Nodes (6): CheckedProgram, execute(), MAX_RECURSION_DEPTH, MAX_SOURCE_FILE_SIZE, TECHSCRIPT_VERSION, main()

### Community 95 - "Op"
Cohesion: 0.21
Nodes (3): BasicBlock, BlockId, CFGAnalysis

### Community 96 - "Interpreter"
Cohesion: 0.16
Nodes (3): ExecResult, Statement, Interpreter

### Community 97 - "ResourceTable"
Cohesion: 0.18
Nodes (3): ResourceTable, RuntimeConfig, RuntimeContext

### Community 98 - "SymbolTable"
Cohesion: 0.20
Nodes (3): Scope, Symbol, SymbolTable

### Community 99 - "FileId"
Cohesion: 0.15
Nodes (10): FileId, Position, file_id_construction(), file_id_hash(), file_id_serde_roundtrip(), position_construction(), position_display(), position_equality() (+2 more)

### Community 101 - "optimizer_tests.rs"
Cohesion: 0.22
Nodes (10): compile_source(), test_bytecode_compilation_and_validation(), test_bytecode_constant_deduplication(), test_bytecode_disassembler(), test_bytecode_serialization_roundtrip(), optimize_source(), test_optimizer_algebraic_simplification(), test_optimizer_branch_simplification() (+2 more)

### Community 105 - "file_ext.rs"
Cohesion: 0.20
Nodes (6): CommonError, is_techscript_file(), TECHSCRIPT_DOT_EXTENSION, TECHSCRIPT_EXTENSION, validate_extension(), validate_extension_invalid()

### Community 106 - "DefaultModuleResolver"
Cohesion: 0.25
Nodes (3): DefaultModuleResolver, ModuleResolver, ModuleSource

### Community 107 - "VMHeap"
Cohesion: 0.21
Nodes (3): GarbageCollector, HeapObject, VMHeap

### Community 108 - "emit.rs"
Cohesion: 0.39
Nodes (5): compile_to_ir_module(), emit_asm(), emit_ir(), emit_llvm(), emit_obj()

### Community 111 - "vm_tests.rs"
Cohesion: 0.27
Nodes (8): execute_source(), test_vm_arithmetic(), test_vm_conditions(), test_vm_division_by_zero(), test_vm_functions(), test_vm_native_call(), test_vm_recursion(), test_vm_variables()

### Community 113 - "diagnosticreporter"
Cohesion: 0.15
Nodes (6): levenshtein(), main(), perform_first_run_check(), show_custom_help(), show_welcome_screen(), suggest_subcommand()

### Community 114 - "ir_tests.rs"
Cohesion: 0.35
Nodes (10): lower_source(), test_ir_basic_arithmetic(), test_ir_dsl_block_empty(), test_ir_dsl_block_nested(), test_ir_dsl_block_with_properties(), test_ir_empty(), test_ir_if_conditional_branching(), test_ir_null_coalescing_lower() (+2 more)

### Community 119 - "bytecode/src/lib.rs"
Cohesion: 0.13
Nodes (3): compile(), BytecodeModule, Operand

### Community 123 - "make_source_file"
Cohesion: 0.22
Nodes (9): make_source_file(), source_file_empty(), source_file_line_col_out_of_bounds(), source_file_line_content_out_of_bounds(), source_file_line_start(), source_file_multi_line(), source_file_single_line(), source_file_unicode() (+1 more)

### Community 128 - "sync.rs"
Cohesion: 0.28
Nodes (4): ScriptMutex, StdlibRegistry, test_script_mutex_lock_unlock(), test_script_mutex_new()

### Community 132 - "disassembler.rs"
Cohesion: 0.48
Nodes (4): BytecodeFunction, BytecodeModule, String, BytecodeDisassembler

### Community 136 - "find_test_files"
Cohesion: 0.21
Nodes (3): EventListener, CompilerPlugin, PluginRegistry

### Community 139 - "frame.rs"
Cohesion: 0.18
Nodes (3): VM, CallFrame, ExceptionHandler

### Community 141 - "commands/docs.rs"
Cohesion: 0.25
Nodes (5): Chunk, BytecodeInstruction, Opcode, Operand, InstructionId

### Community 143 - "doc.rs"
Cohesion: 0.67
Nodes (3): ExitCode, Option, execute()

### Community 144 - "Theme"
Cohesion: 0.23
Nodes (4): LiteralKind, lookup_keyword(), numeric_literal_kind(), NumericLiteralKind

### Community 148 - ".register_database"
Cohesion: 0.32
Nodes (9): apply_cli(), apply_env(), apply_toml(), CliConfig, ConfigManager, EffectiveConfig, num_cpus(), TomlConfig (+1 more)

### Community 160 - ".register_excel"
Cohesion: 0.14
Nodes (14): CanvasOp, Append, Background, Circle, Close, Content, Create, Line (+6 more)

### Community 161 - ".register_http"
Cohesion: 0.47
Nodes (10): contains(), from_bool(), from_float(), from_int(), join(), replace(), split(), to_lower() (+2 more)

### Community 168 - ".register_net"
Cohesion: 0.29
Nodes (8): dump_repl_ast(), dump_repl_bytecode(), dump_repl_ir(), eval_code(), evaluate_type(), execute(), handle_metacommand(), print_help()

### Community 171 - ".register_postgres"
Cohesion: 0.21
Nodes (5): DiagnosticOutput, Colored, Json, Plain, DiagnosticRenderer

### Community 172 - ".register_powerpoint"
Cohesion: 0.27
Nodes (6): parse_json_value(), StdlibRegistry, stringify_value(), stringify_value_inner(), test_parse_json_value(), test_stringify_error()

### Community 173 - ".register_profiler"
Cohesion: 0.33
Nodes (4): ModelInstance, NEXT_OBJECT_ID, ObjectId, StructInstance

### Community 181 - ".register_sys"
Cohesion: 0.64
Nodes (6): check_fs_capability(), sys_exists(), sys_now(), sys_read_file(), sys_sleep(), sys_write_file()

### Community 182 - ".register_system"
Cohesion: 0.28
Nodes (3): ArtifactCategory, BuildManifest, BuildOutput

### Community 186 - ".register_video"
Cohesion: 0.22
Nodes (6): Severity, Error, Help, Hint, Note, Warning

### Community 190 - "techscript_packager"
Cohesion: 0.56
Nodes (4): process_exit(), process_pid(), process_run(), process_spawn()

### Community 196 - "database.rs"
Cohesion: 0.64
Nodes (5): SqlParam, std_database_close(), std_database_connect(), std_database_execute(), std_database_query()

### Community 197 - "env.rs"
Cohesion: 0.61
Nodes (5): env_all(), env_args(), env_current_dir(), env_get(), env_set()

### Community 198 - "net_handler.rs"
Cohesion: 0.64
Nodes (4): tcp_connect_fn(), tcp_listen_fn(), tcp_recv_fn(), tcp_send_fn()

### Community 209 - ".register_http"
Cohesion: 0.50
Nodes (3): execute_request!, StdlibRegistry, test_http_listen_capability_denied()

## Knowledge Gaps
- **387 isolated node(s):** `AssignmentExpr`, `BinaryExpr`, `Block`, `BlockId`, `ConstDecl` (+382 more)
  These have ≤1 connection - possible missing edges. (Counts symbols only; 994 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **159 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Write` connect `RefCell` to `DocumentFormatter`, `disassembler.rs`, `BuiltinRegistry`, `ProgressBar`, `RichDiagnostic`, `.register_postgres`, `packager/src/main.rs`, `doc.rs`, `.write_to_file`, `cli/src/main.rs`, `ProjectBuildGraph`, `migrate.rs`, `TimingProfiler`, `VMDebugger`, `.register_notification`?**
  _High betweenness centrality (0.078) - this node is a cross-community bridge._
- **Are the 13 inferred relationships involving `NodeId` (e.g. with `test_ast_assignment_expression()` and `test_ast_construction_and_equality()`) actually correct?**
  _`NodeId` has 13 INFERRED edges - model-reasoned connections that need verification._
- **What connects `AssignmentExpr`, `BinaryExpr`, `Block` to the rest of the system?**
  _387 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `LoweringContext` be split into smaller, more focused modules?**
  _Cohesion score 0.057971014492753624 - nodes in this community are weakly interconnected._
- **Why does `Span` connect `cli/src/config.rs` to `Span`, `Token`, `Diagnostic`, `RefCell`, `.dummy`, `commands/docs.rs`, `Value`, `BytecodeBuilder`, `CallFrame`, `RuntimeError`, `SourceFile`, `stdlib_tests.rs`, `.register_audio`, `techscript_ast`?**
  _High betweenness centrality (0.074) - this node is a cross-community bridge._
- **Should `Span` be split into smaller, more focused modules?**
  _Cohesion score 0.11779448621553884 - nodes in this community are weakly interconnected._
- **Why does `RuntimeValue` connect `RuntimeValue` to `Interpreter`, `Environment`, `.register_profiler`, `vm_tests.rs`, `ProjectBuildGraph`, `RuntimeError`, `RuntimeContext`, `UserFunction`, `NativeBridge`, `run_src`, `VM`?**
  _High betweenness centrality (0.062) - this node is a cross-community bridge._