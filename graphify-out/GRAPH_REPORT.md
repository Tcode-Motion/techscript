# Graph Report - .  (2026-09-15)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 1377 nodes · 2859 edges · 82 communities (61 shown, 21 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 3 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `35fd6533`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 23|Community 23]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]
- [[_COMMUNITY_Community 28|Community 28]]
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 32|Community 32]]
- [[_COMMUNITY_Community 33|Community 33]]
- [[_COMMUNITY_Community 34|Community 34]]
- [[_COMMUNITY_Community 35|Community 35]]
- [[_COMMUNITY_Community 36|Community 36]]
- [[_COMMUNITY_Community 37|Community 37]]
- [[_COMMUNITY_Community 38|Community 38]]
- [[_COMMUNITY_Community 39|Community 39]]
- [[_COMMUNITY_Community 40|Community 40]]
- [[_COMMUNITY_Community 41|Community 41]]
- [[_COMMUNITY_Community 42|Community 42]]
- [[_COMMUNITY_Community 43|Community 43]]
- [[_COMMUNITY_Community 44|Community 44]]
- [[_COMMUNITY_Community 45|Community 45]]
- [[_COMMUNITY_Community 46|Community 46]]
- [[_COMMUNITY_Community 47|Community 47]]
- [[_COMMUNITY_Community 48|Community 48]]
- [[_COMMUNITY_Community 49|Community 49]]
- [[_COMMUNITY_Community 50|Community 50]]
- [[_COMMUNITY_Community 51|Community 51]]
- [[_COMMUNITY_Community 52|Community 52]]
- [[_COMMUNITY_Community 53|Community 53]]
- [[_COMMUNITY_Community 54|Community 54]]
- [[_COMMUNITY_Community 55|Community 55]]
- [[_COMMUNITY_Community 56|Community 56]]
- [[_COMMUNITY_Community 57|Community 57]]
- [[_COMMUNITY_Community 58|Community 58]]
- [[_COMMUNITY_Community 59|Community 59]]
- [[_COMMUNITY_Community 60|Community 60]]
- [[_COMMUNITY_Community 61|Community 61]]
- [[_COMMUNITY_Community 62|Community 62]]
- [[_COMMUNITY_Community 63|Community 63]]
- [[_COMMUNITY_Community 64|Community 64]]
- [[_COMMUNITY_Community 66|Community 66]]
- [[_COMMUNITY_Community 67|Community 67]]
- [[_COMMUNITY_Community 68|Community 68]]
- [[_COMMUNITY_Community 69|Community 69]]
- [[_COMMUNITY_Community 70|Community 70]]
- [[_COMMUNITY_Community 71|Community 71]]
- [[_COMMUNITY_Community 72|Community 72]]
- [[_COMMUNITY_Community 73|Community 73]]
- [[_COMMUNITY_Community 74|Community 74]]
- [[_COMMUNITY_Community 77|Community 77]]

## God Nodes (most connected - your core abstractions)
1. `TsValue` - 53 edges
2. `Backend` - 45 edges
3. `IRBuilder` - 34 edges
4. `LoweringContext` - 34 edges
5. `Result` - 29 edges
6. `Option` - 27 edges
7. `ResolveSymbols` - 24 edges
8. `SemanticContext` - 24 edges
9. `Result` - 23 edges
10. `CodegenContext` - 22 edges

## Surprising Connections (you probably didn't know these)
- `techscript_cli` --crate_depends_on--> `techscript_native_runtime`  [EXTRACTED]
  cli/Cargo.toml → runtime/native_runtime/Cargo.toml
- `techscript_cli` --crate_depends_on--> `techscript_package_manager`  [EXTRACTED]
  cli/Cargo.toml → tools/package-manager/Cargo.toml
- `techscript_cli` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  cli/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_formatter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  tools/formatter/Cargo.toml → compiler/ast/Cargo.toml
- `techscript_interpreter` --crate_depends_on--> `techscript_ast`  [EXTRACTED]
  runtime/interpreter/Cargo.toml → compiler/ast/Cargo.toml

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

## Communities (82 total, 21 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.06
Nodes (44): criterion_benchmark(), Criterion, AssignmentExpr, BinaryExpr, Block, BlockId, ConstDecl, Default (+36 more)

### Community 1 - "Community 1"
Cohesion: 0.10
Nodes (60): c_char, HashMap, String, JmpBuf, ts_add(), ts_alloc_bool(), ts_alloc_enum(), ts_alloc_float() (+52 more)

### Community 2 - "Community 2"
Cohesion: 0.07
Nodes (30): FieldSpec, Block, Callable, ConstDecl, DSLBlock, EnumDecl, ExecResult, ForStmt (+22 more)

### Community 3 - "Community 3"
Cohesion: 0.08
Nodes (35): CodegenContext, BlockId, Function, GlobalId, HashMap, IRType, LiteralVal, LLVMTypeRef (+27 more)

### Community 4 - "Community 4"
Cohesion: 0.10
Nodes (30): AssignmentExpr, BinaryExpr, Block, CallExpr, ConstDecl, DSLBlock, EnumDecl, Expression (+22 more)

### Community 5 - "Community 5"
Cohesion: 0.10
Nodes (30): DependencyConfig, Display, Formatter, LockedPackage, PackageConfig, CapabilityValidator, DependencyConfig, DependencySolver (+22 more)

### Community 6 - "Community 6"
Cohesion: 0.09
Nodes (23): BasicBlock, BlockId, Default, DslBlockId, Function, GlobalId, IRType, LiteralVal (+15 more)

### Community 7 - "Community 7"
Cohesion: 0.10
Nodes (22): AskExpr, EvalResult, FStringExpr, IndexExpr, LambdaExpr, ListExpr, LiteralExpr, MapExpr (+14 more)

### Community 8 - "Community 8"
Cohesion: 0.11
Nodes (20): Diagnostic, Duration, ErrorCode, FileId, Into, Option, Self, SourceManager (+12 more)

### Community 9 - "Community 9"
Cohesion: 0.11
Nodes (29): ArtifactManager, BuildCache, ExitCode, Option, BytecodeModule, Duration, FileId, Option (+21 more)

### Community 10 - "Community 10"
Cohesion: 0.18
Nodes (33): AsRef, build_and_copy_tools(), calculate_checksums_json(), compile_binary(), compile_inno_installer(), copy_dir_all(), copy_resources(), count_files() (+25 more)

### Community 11 - "Community 11"
Cohesion: 0.10
Nodes (24): ExitCode, Option, PathBuf, String, dump_repl_ast(), dump_repl_bytecode(), dump_repl_ir(), eval_code() (+16 more)

### Community 12 - "Community 12"
Cohesion: 0.21
Nodes (15): Block, DiagnosticReporter, DSLBlock, ForStmt, IfStmt, ImportStmt, ParseResult, RepeatStmt (+7 more)

### Community 13 - "Community 13"
Cohesion: 0.15
Nodes (12): DiagnosticReporter, ErrorCode, HashSet, ParseResult, Self, String, Token, TokenKind (+4 more)

### Community 14 - "Community 14"
Cohesion: 0.15
Nodes (19): DslBlockValue, IpAddr, Resolver, SocketAddr, dsl_to_html(), is_safe_ip(), is_safe_url(), render_card() (+11 more)

### Community 15 - "Community 15"
Cohesion: 0.17
Nodes (15): BytecodeModule, HashMap, Module, OptimizationLevel, Option, Path, PathBuf, Result (+7 more)

### Community 16 - "Community 16"
Cohesion: 0.18
Nodes (19): FileId, HashMap, HashSet, Option, Path, PathBuf, Result, Self (+11 more)

### Community 17 - "Community 17"
Cohesion: 0.14
Nodes (16): Box, AnalysisManager, Default, Module, OptimizationPass, OptimizationResult, Self, Vec (+8 more)

### Community 18 - "Community 18"
Cohesion: 0.17
Nodes (22): Capability, Default, HashSet, OptimizationLevel, Option, Path, PathBuf, Result (+14 more)

### Community 19 - "Community 19"
Cohesion: 0.15
Nodes (14): BlockId, Drop, GlobalId, HashMap, LLVMContextRef, LLVMValueRef, LocalId, Option (+6 more)

### Community 20 - "Community 20"
Cohesion: 0.14
Nodes (19): ExitCode, Option, ExitCode, Option, String, execute(), execute(), line_transform() (+11 more)

### Community 21 - "Community 21"
Cohesion: 0.34
Nodes (24): techscript_ast, techscript_builtins, techscript_bytecode, techscript_cli, techscript_common, techscript_errors, techscript_formatter, techscript_gc (+16 more)

### Community 22 - "Community 22"
Cohesion: 0.14
Nodes (15): Default, Duration, HashMap, Into, Option, Self, String, Vec (+7 more)

### Community 23 - "Community 23"
Cohesion: 0.12
Nodes (12): BytecodeFunction, BytecodeModule, String, Opcode, Operand, BytecodeFunction, Default, HashSet (+4 more)

### Community 25 - "Community 25"
Cohesion: 0.12
Nodes (15): CanvasOp, CanvasFn, CanvasOp, dsl_to_svg(), parse_height(), parse_width(), StdlibRegistry, Callable (+7 more)

### Community 26 - "Community 26"
Cohesion: 0.14
Nodes (16): CheckedProgram, CodeLens, CodeLensParams, CompletionParams, CompletionResponse, InlayHint, InlayHintParams, collect_local_decls() (+8 more)

### Community 27 - "Community 27"
Cohesion: 0.23
Nodes (8): ExitCode, Option, PathBuf, Self, DoctorContext, execute(), home_dir(), Theme

### Community 28 - "Community 28"
Cohesion: 0.13
Nodes (12): CallHierarchyItem, CallHierarchyPrepareParams, GotoTypeDefinitionParams, GotoTypeDefinitionResponse, Hover, HoverParams, Location, Position (+4 more)

### Community 29 - "Community 29"
Cohesion: 0.20
Nodes (10): DiagnosticReporter, Expression, ParseResult, Result, String, Token, TokenKind, Precedence (+2 more)

### Community 31 - "Community 31"
Cohesion: 0.24
Nodes (6): Result, RuntimeValue, Self, Vec, VMError, ValueStack

### Community 32 - "Community 32"
Cohesion: 0.22
Nodes (9): DocumentFormatter, Formatter, DSLBlock, Expression, LiteralVal, Program, Self, Statement (+1 more)

### Community 33 - "Community 33"
Cohesion: 0.38
Nodes (12): HashMap, Self, String, Vec, build_dsl_registry(), DSLSchema, register_canvas_misc_schemas(), register_canvas_schemas() (+4 more)

### Community 34 - "Community 34"
Cohesion: 0.18
Nodes (12): DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, Mutex, Backend, is_keyword_kind(), LocalDecl, HashMap (+4 more)

### Community 35 - "Community 35"
Cohesion: 0.16
Nodes (9): CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, DocumentFormattingParams, DocumentOnTypeFormattingParams, DocumentRangeFormattingParams, InitializeParams, InitializeResult, TextEdit (+1 more)

### Community 36 - "Community 36"
Cohesion: 0.14
Nodes (11): CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, GotoDeclarationParams, GotoDeclarationResponse, GotoDefinitionParams, GotoDefinitionResponse, GotoImplementationParams, GotoImplementationResponse (+3 more)

### Community 37 - "Community 37"
Cohesion: 0.42
Nodes (15): ImageFormat, Rgba, create_canvas(), draw_circle(), draw_line(), draw_rect(), parse_color(), save_image_with_format() (+7 more)

### Community 38 - "Community 38"
Cohesion: 0.34
Nodes (10): Module, Path, Result, String, LLVMCodeGenFileType, get_host_target_triple(), LLVMBackend, LLVMBackendOptions (+2 more)

### Community 39 - "Community 39"
Cohesion: 0.16
Nodes (9): Client, CodeActionParams, CodeActionResponse, DocumentSymbolParams, DocumentSymbolResponse, Range, SemanticTokensParams, SemanticTokensResult (+1 more)

### Community 40 - "Community 40"
Cohesion: 0.20
Nodes (8): NativeCallback, HashMap, Result, RuntimeError, Self, String, Value, BuiltinRegistry

### Community 41 - "Community 41"
Cohesion: 0.24
Nodes (7): AstVisitor, IndexMap, RefCell, runtime_to_sql_value(), RuntimeValue, Value, Write

### Community 42 - "Community 42"
Cohesion: 0.21
Nodes (9): parse_json_value(), StdlibRegistry, stringify_value(), test_stringify_error(), Result, RuntimeError, RuntimeValue, String (+1 more)

### Community 43 - "Community 43"
Cohesion: 0.30
Nodes (9): Diagnostic, DiagnosticReporter, Program, Result, Token, Vec, parse(), parse_recovered() (+1 more)

### Community 44 - "Community 44"
Cohesion: 0.35
Nodes (10): gunzip_archive(), gzip_file(), StdlibRegistry, tar_dir(), test_unzip_archive_corrupted_zip(), test_unzip_archive_invalid_path(), untar_archive(), unzip_archive() (+2 more)

### Community 45 - "Community 45"
Cohesion: 0.45
Nodes (11): extract_sqlite_params(), std_database_close(), std_database_connect(), std_database_execute(), std_database_query(), Result, RuntimeContext, RuntimeError (+3 more)

### Community 46 - "Community 46"
Cohesion: 0.55
Nodes (10): file_copy(), file_exists(), file_read(), file_remove(), file_write(), Result, RuntimeContext, RuntimeError (+2 more)

### Community 47 - "Community 47"
Cohesion: 0.31
Nodes (7): Option, Path, PathBuf, Result, String, CrashReport, install_panic_hook()

### Community 48 - "Community 48"
Cohesion: 0.47
Nodes (8): register_common_functions(), register_list_functions(), register_map_functions(), StdlibRegistry, Callable, HashMap, Rc, String

### Community 49 - "Community 49"
Cohesion: 0.52
Nodes (6): main(), levenshtein(), perform_first_run_check(), show_custom_help(), show_welcome_screen(), suggest_subcommand()

### Community 51 - "Community 51"
Cohesion: 0.33
Nodes (4): Result, RuntimeValue, VMError, VM

### Community 52 - "Community 52"
Cohesion: 0.47
Nodes (4): StdlibRegistry, test_csv_parse(), test_csv_register(), test_csv_stringify()

### Community 53 - "Community 53"
Cohesion: 0.70
Nodes (4): criterion_benchmark(), find_boundary_fast(), find_boundary_slow(), Criterion

### Community 54 - "Community 54"
Cohesion: 0.40
Nodes (3): TypeHierarchyItem, TypeHierarchySubtypesParams, TypeHierarchySupertypesParams

## Knowledge Gaps
- **307 isolated node(s):** `PathBuf`, `Self`, `Option`, `Theme`, `Self` (+302 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **21 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Write` connect `Community 41` to `Community 32`, `Community 40`, `Community 9`, `Community 8`, `Community 10`, `Community 14`, `Community 47`, `Community 20`, `Community 22`, `Community 23`, `Community 25`?**
  _High betweenness centrality (0.199) - this node is a cross-community bridge._
- **Why does `RefCell` connect `Community 41` to `Community 1`, `Community 37`, `Community 42`, `Community 45`, `Community 51`, `Community 52`, `Community 24`, `Community 25`?**
  _High betweenness centrality (0.118) - this node is a cross-community bridge._
- **Why does `Backend` connect `Community 34` to `Community 35`, `Community 36`, `Community 39`, `Community 50`, `Community 54`, `Community 26`, `Community 28`, `Community 62`, `Community 63`?**
  _High betweenness centrality (0.057) - this node is a cross-community bridge._
- **What connects `PathBuf`, `Self`, `Option` to the rest of the system?**
  _307 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 0` be split into smaller, more focused modules?**
  _Cohesion score 0.05674044265593561 - nodes in this community are weakly interconnected._
- **Should `Community 1` be split into smaller, more focused modules?**
  _Cohesion score 0.09730301427815971 - nodes in this community are weakly interconnected._
- **Should `Community 2` be split into smaller, more focused modules?**
  _Cohesion score 0.07402597402597402 - nodes in this community are weakly interconnected._