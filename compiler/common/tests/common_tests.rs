//! Comprehensive integration tests for the `techscript_common` crate.
//!
//! Tests cover every public API surface to ensure correctness, edge-case
//! handling, and backward compatibility with downstream crates.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use techscript_common::{
    is_techscript_file, validate_extension, CommonError, FileId, Ident, NodeId, NodeIdGenerator,
    Position, SourceFile, SourceManager, Span, MAX_RECURSION_DEPTH, MAX_SOURCE_FILE_SIZE,
    TECHSCRIPT_DOT_EXTENSION, TECHSCRIPT_EXTENSION, TECHSCRIPT_VERSION,
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Span Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn span_construction() {
    let span = Span::new(5, 10);
    assert_eq!(span.start, 5);
    assert_eq!(span.end, 10);
}

#[test]
fn span_dummy() {
    let span = Span::dummy();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 0);
    assert!(span.is_empty());
}

#[test]
fn span_len() {
    assert_eq!(Span::new(0, 10).len(), 10);
    assert_eq!(Span::new(5, 5).len(), 0);
    assert_eq!(Span::new(100, 200).len(), 100);
}

#[test]
fn span_is_empty() {
    assert!(Span::new(0, 0).is_empty());
    assert!(Span::new(5, 5).is_empty());
    assert!(!Span::new(0, 1).is_empty());
}

#[test]
fn span_contains() {
    let span = Span::new(5, 10);
    assert!(!span.contains(4));
    assert!(span.contains(5));
    assert!(span.contains(7));
    assert!(span.contains(9));
    assert!(!span.contains(10)); // exclusive end
}

#[test]
fn span_merge() {
    let a = Span::new(0, 5);
    let b = Span::new(10, 15);
    let merged = a.merge(b);
    assert_eq!(merged, Span::new(0, 15));

    // Overlapping spans
    let c = Span::new(3, 8);
    let d = Span::new(5, 12);
    assert_eq!(c.merge(d), Span::new(3, 12));

    // Same span
    let e = Span::new(1, 5);
    assert_eq!(e.merge(e), Span::new(1, 5));
}

#[test]
fn span_source_text_valid() {
    let source = "make x = 42";
    let span = Span::new(0, 4);
    assert_eq!(span.source_text(source), Some("make"));

    let span2 = Span::new(5, 6);
    assert_eq!(span2.source_text(source), Some("x"));

    let span3 = Span::new(9, 11);
    assert_eq!(span3.source_text(source), Some("42"));
}

#[test]
fn span_source_text_empty_span() {
    let source = "hello";
    let span = Span::new(2, 2);
    assert_eq!(span.source_text(source), Some(""));
}

#[test]
fn span_source_text_out_of_bounds() {
    let source = "hello";
    assert_eq!(Span::new(0, 100).source_text(source), None);
    assert_eq!(Span::new(10, 20).source_text(source), None);
}

#[test]
fn span_source_text_invalid_start_end() {
    let source = "hello";
    // start > end
    assert_eq!(Span::new(3, 1).source_text(source), None);
}

#[test]
fn span_source_text_unicode() {
    let source = "say \"héllo\"";
    // Layout: s(0) a(1) y(2) ' '(3) '"'(4) h(5) é(6-7) l(8) l(9) o(10) '"'(11)
    // "héllo" = h(1) + é(2) + l(1) + l(1) + o(1) = 6 bytes at offset 5..11
    let span = Span::new(5, 11);
    assert_eq!(span.source_text(source), Some("héllo"));
}

#[test]
fn span_display() {
    let span = Span::new(10, 25);
    assert_eq!(format!("{span}"), "10..25");
}

#[test]
fn span_equality_and_hash() {
    let a = Span::new(0, 5);
    let b = Span::new(0, 5);
    let c = Span::new(1, 5);
    assert_eq!(a, b);
    assert_ne!(a, c);

    let mut set = HashSet::new();
    set.insert(a);
    assert!(set.contains(&b));
    assert!(!set.contains(&c));
}

#[test]
fn span_serde_roundtrip() {
    let span = Span::new(10, 20);
    let json = serde_json::to_string(&span).unwrap();
    let deserialized: Span = serde_json::from_str(&json).unwrap();
    assert_eq!(span, deserialized);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// NodeId Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn node_id_construction() {
    let id = NodeId(42);
    assert_eq!(id.0, 42);
    assert_eq!(id.as_u32(), 42);
}

#[test]
fn node_id_dummy() {
    let id = NodeId::dummy();
    assert_eq!(id.as_u32(), u32::MAX);
}

#[test]
fn node_id_display() {
    let id = NodeId(7);
    assert_eq!(format!("{id}"), "NodeId(7)");
}

#[test]
fn node_id_equality_and_hash() {
    let a = NodeId(1);
    let b = NodeId(1);
    let c = NodeId(2);
    assert_eq!(a, b);
    assert_ne!(a, c);

    let mut set = HashSet::new();
    set.insert(a);
    assert!(set.contains(&b));
    assert!(!set.contains(&c));
}

#[test]
fn node_id_serde_roundtrip() {
    let id = NodeId(99);
    let json = serde_json::to_string(&id).unwrap();
    let deserialized: NodeId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, deserialized);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// NodeIdGenerator Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn generator_sequential_ids() {
    let gen = NodeIdGenerator::new();
    assert_eq!(gen.next().as_u32(), 0);
    assert_eq!(gen.next().as_u32(), 1);
    assert_eq!(gen.next().as_u32(), 2);
}

#[test]
fn generator_peek() {
    let gen = NodeIdGenerator::new();
    assert_eq!(gen.peek().as_u32(), 0); // No IDs generated yet
    gen.next();
    assert_eq!(gen.peek().as_u32(), 1); // Next would be 1
    gen.next();
    assert_eq!(gen.peek().as_u32(), 2); // Next would be 2
}

#[test]
fn generator_current() {
    let gen = NodeIdGenerator::new();
    // Before any generation, current() returns 0 (saturating_sub)
    assert_eq!(gen.current().as_u32(), 0);
    gen.next(); // generates 0
    assert_eq!(gen.current().as_u32(), 0);
    gen.next(); // generates 1
    assert_eq!(gen.current().as_u32(), 1);
}

#[test]
fn generator_reset() {
    let gen = NodeIdGenerator::new();
    gen.next();
    gen.next();
    gen.next();
    assert_eq!(gen.peek().as_u32(), 3);

    gen.reset();
    assert_eq!(gen.peek().as_u32(), 0);
    assert_eq!(gen.next().as_u32(), 0);
}

#[test]
fn generator_default() {
    let gen = NodeIdGenerator::default();
    assert_eq!(gen.next().as_u32(), 0);
}

#[test]
fn generator_debug() {
    let gen = NodeIdGenerator::new();
    gen.next();
    let debug = format!("{gen:?}");
    assert!(debug.contains("NodeIdGenerator"));
    assert!(debug.contains("next_id"));
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Ident Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn ident_construction() {
    let span = Span::new(0, 5);
    let ident = Ident::new("hello".to_string(), span);
    assert_eq!(ident.name, "hello");
    assert_eq!(ident.span, span);
}

#[test]
fn ident_dummy() {
    let ident = Ident::dummy("test_var");
    assert_eq!(ident.name, "test_var");
    assert_eq!(ident.span, Span::dummy());
}

#[test]
fn ident_display() {
    let ident = Ident::dummy("counter");
    assert_eq!(format!("{ident}"), "counter");
}

#[test]
fn ident_equality_includes_span() {
    let a = Ident::new("x".to_string(), Span::new(0, 1));
    let b = Ident::new("x".to_string(), Span::new(0, 1));
    let c = Ident::new("x".to_string(), Span::new(5, 6));
    assert_eq!(a, b);
    assert_ne!(a, c); // Same name, different span
}

#[test]
fn ident_hash_ignores_span() {
    use std::hash::{DefaultHasher, Hash, Hasher};

    let a = Ident::new("x".to_string(), Span::new(0, 1));
    let b = Ident::new("x".to_string(), Span::new(100, 101));

    let hash_a = {
        let mut h = DefaultHasher::new();
        a.hash(&mut h);
        h.finish()
    };
    let hash_b = {
        let mut h = DefaultHasher::new();
        b.hash(&mut h);
        h.finish()
    };

    assert_eq!(
        hash_a, hash_b,
        "Idents with same name should hash identically"
    );
}

#[test]
fn ident_serde_roundtrip() {
    let ident = Ident::new("my_var".to_string(), Span::new(10, 16));
    let json = serde_json::to_string(&ident).unwrap();
    let deserialized: Ident = serde_json::from_str(&json).unwrap();
    assert_eq!(ident, deserialized);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// FileId Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn file_id_construction() {
    let id = FileId(0);
    assert_eq!(id.as_u32(), 0);
}

#[test]
fn file_id_equality() {
    assert_eq!(FileId(0), FileId(0));
    assert_ne!(FileId(0), FileId(1));
}

#[test]
fn file_id_display() {
    assert_eq!(format!("{}", FileId(3)), "FileId(3)");
}

#[test]
fn file_id_hash() {
    let mut set = HashSet::new();
    set.insert(FileId(0));
    assert!(set.contains(&FileId(0)));
    assert!(!set.contains(&FileId(1)));
}

#[test]
fn file_id_serde_roundtrip() {
    let id = FileId(42);
    let json = serde_json::to_string(&id).unwrap();
    let deserialized: FileId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, deserialized);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// SourceFile Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn make_source_file(source: &str) -> SourceFile {
    SourceFile::new(FileId(0), PathBuf::from("test.txs"), source.to_string())
}

#[test]
fn source_file_single_line() {
    let file = make_source_file("make x = 42");
    assert_eq!(file.line_count(), 1);
    assert_eq!(file.line_col(0), Some((1, 1)));
    assert_eq!(file.line_col(5), Some((1, 6)));
    assert_eq!(file.line_content(1), Some("make x = 42"));
}

#[test]
fn source_file_multi_line() {
    let source = "make x = 42\nsay x\nmake y = 10\n";
    let file = make_source_file(source);

    assert_eq!(file.line_count(), 4); // 3 lines + trailing newline creates line 4

    // Line 1
    assert_eq!(file.line_col(0), Some((1, 1))); // 'm' of 'make'
    assert_eq!(file.line_col(11), Some((1, 12))); // '2' of '42'
    assert_eq!(file.line_content(1), Some("make x = 42"));

    // Line 2
    assert_eq!(file.line_col(12), Some((2, 1))); // 's' of 'say'
    assert_eq!(file.line_content(2), Some("say x"));

    // Line 3
    assert_eq!(file.line_col(18), Some((3, 1))); // 'm' of second 'make'
    assert_eq!(file.line_content(3), Some("make y = 10"));
}

#[test]
fn source_file_empty() {
    let file = make_source_file("");
    assert_eq!(file.line_count(), 1);
    assert_eq!(file.line_col(0), Some((1, 1)));
    assert_eq!(file.line_content(1), Some(""));
}

#[test]
fn source_file_line_col_out_of_bounds() {
    let file = make_source_file("hello");
    assert_eq!(file.line_col(100), None);
}

#[test]
fn source_file_line_content_out_of_bounds() {
    let file = make_source_file("hello\nworld");
    assert_eq!(file.line_content(0), None); // 0 is invalid (1-indexed)
    assert_eq!(file.line_content(3), None); // only 2 lines
}

#[test]
fn source_file_line_start() {
    let file = make_source_file("abc\ndef\nghi");
    assert_eq!(file.line_start(1), Some(0));
    assert_eq!(file.line_start(2), Some(4));
    assert_eq!(file.line_start(3), Some(8));
    assert_eq!(file.line_start(0), None);
    assert_eq!(file.line_start(4), None);
}

#[test]
fn source_file_unicode() {
    // 'é' is 2 bytes in UTF-8
    let source = "say \"héllo\"\nsay \"world\"";
    let file = make_source_file(source);
    assert_eq!(file.line_count(), 2);
    assert_eq!(file.line_content(1), Some("say \"héllo\""));
    assert_eq!(file.line_content(2), Some("say \"world\""));
}

#[test]
fn source_file_path() {
    let file = SourceFile::new(
        FileId(0),
        PathBuf::from("src/main.txs"),
        "hello".to_string(),
    );
    assert_eq!(file.path(), Path::new("src/main.txs"));
}

#[test]
fn source_file_windows_line_endings() {
    let source = "line1\r\nline2\r\nline3";
    let file = make_source_file(source);
    // \r\n: \n triggers the line start computation
    assert_eq!(file.line_content(1), Some("line1"));
    assert_eq!(file.line_content(2), Some("line2"));
    assert_eq!(file.line_content(3), Some("line3"));
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// SourceManager Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn source_manager_add_and_get() {
    let mut manager = SourceManager::new();
    let id = manager.add_file(PathBuf::from("main.txs"), "say \"hello\"".to_string());
    assert_eq!(id, FileId(0));
    assert_eq!(manager.file_count(), 1);

    let file = manager.get_file(id).unwrap();
    assert_eq!(file.source(), "say \"hello\"");
    assert_eq!(file.path(), Path::new("main.txs"));
}

#[test]
fn source_manager_multiple_files() {
    let mut manager = SourceManager::new();
    let id0 = manager.add_file(PathBuf::from("a.txs"), "file a".to_string());
    let id1 = manager.add_file(PathBuf::from("b.txs"), "file b".to_string());
    let id2 = manager.add_file(PathBuf::from("c.txs"), "file c".to_string());

    assert_eq!(id0, FileId(0));
    assert_eq!(id1, FileId(1));
    assert_eq!(id2, FileId(2));
    assert_eq!(manager.file_count(), 3);

    assert_eq!(manager.get_file(id0).unwrap().source(), "file a");
    assert_eq!(manager.get_file(id1).unwrap().source(), "file b");
    assert_eq!(manager.get_file(id2).unwrap().source(), "file c");
}

#[test]
fn source_manager_invalid_file_id() {
    let manager = SourceManager::new();
    assert!(manager.get_file(FileId(0)).is_none());
    assert!(manager.get_file(FileId(999)).is_none());
}

#[test]
fn source_manager_resolve_position() {
    let mut manager = SourceManager::new();
    let id = manager.add_file(PathBuf::from("test.txs"), "make x = 42\nsay x".to_string());

    // Resolve a span in line 1
    let pos = manager.resolve_position(id, &Span::new(5, 6)).unwrap();
    assert_eq!(pos.file_id, id);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 6);
    assert_eq!(pos.offset, 5);

    // Resolve a span in line 2
    let pos2 = manager.resolve_position(id, &Span::new(13, 14)).unwrap();
    assert_eq!(pos2.line, 2);
    assert_eq!(pos2.column, 2);
    assert_eq!(pos2.offset, 13);
}

#[test]
fn source_manager_shared_access() {
    let mut manager = SourceManager::new();
    let id = manager.add_file(PathBuf::from("test.txs"), "hello".to_string());

    // Multiple calls to get_file return Arc pointers to the same data
    let file1 = manager.get_file(id).unwrap();
    let file2 = manager.get_file(id).unwrap();
    assert!(std::sync::Arc::ptr_eq(&file1, &file2));
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Position Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn position_construction() {
    let pos = Position::new(FileId(0), 5, 12, 42);
    assert_eq!(pos.file_id, FileId(0));
    assert_eq!(pos.line, 5);
    assert_eq!(pos.column, 12);
    assert_eq!(pos.offset, 42);
}

#[test]
fn position_display() {
    let pos = Position::new(FileId(0), 5, 12, 42);
    assert_eq!(format!("{pos}"), "FileId(0):5:12");
}

#[test]
fn position_equality() {
    let a = Position::new(FileId(0), 1, 1, 0);
    let b = Position::new(FileId(0), 1, 1, 0);
    let c = Position::new(FileId(1), 1, 1, 0);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn position_serde_roundtrip() {
    let pos = Position::new(FileId(0), 10, 5, 100);
    let json = serde_json::to_string(&pos).unwrap();
    let deserialized: Position = serde_json::from_str(&json).unwrap();
    assert_eq!(pos, deserialized);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// File Extension Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn extension_constants() {
    assert_eq!(TECHSCRIPT_EXTENSION, "txs");
    assert_eq!(TECHSCRIPT_DOT_EXTENSION, ".txs");
}

#[test]
fn is_techscript_file_valid() {
    assert!(is_techscript_file(Path::new("main.txs")));
    assert!(is_techscript_file(Path::new("src/lib.txs")));
    assert!(is_techscript_file(Path::new("/abs/path/script.txs")));
}

#[test]
fn is_techscript_file_invalid() {
    assert!(!is_techscript_file(Path::new("main.tech")));
    assert!(!is_techscript_file(Path::new("main.rs")));
    assert!(!is_techscript_file(Path::new("main.TXS"))); // case-sensitive
    assert!(!is_techscript_file(Path::new("main.Txs"))); // case-sensitive
    assert!(!is_techscript_file(Path::new("main"))); // no extension
    assert!(!is_techscript_file(Path::new(""))); // empty path
}

#[test]
fn validate_extension_valid() {
    assert!(validate_extension(Path::new("test.txs")).is_ok());
}

#[test]
fn validate_extension_invalid() {
    let err = validate_extension(Path::new("test.tech")).unwrap_err();
    match err {
        CommonError::InvalidExtension { path, message } => {
            assert!(path.contains("test.tech"));
            assert!(message.contains(".txs"));
        }
    }
}

#[test]
fn common_error_display() {
    let err = CommonError::InvalidExtension {
        path: "bad.tech".to_string(),
        message: "TechScript source files must use the '.txs' extension".to_string(),
    };
    let msg = format!("{err}");
    assert!(msg.contains(".txs"));
    assert!(msg.contains("bad.tech"));
}

#[test]
fn common_error_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(CommonError::InvalidExtension {
        path: "x.rs".to_string(),
        message: "wrong extension".to_string(),
    });
    // Just verify it compiles and can be used as a trait object
    assert!(!err.to_string().is_empty());
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Constants Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[test]
fn version_string_is_not_empty() {
    assert!(!TECHSCRIPT_VERSION.is_empty());
}

#[test]
fn version_string_matches_cargo() {
    // The version should match the crate's Cargo.toml version
    assert_eq!(TECHSCRIPT_VERSION, "0.1.0");
}

#[test]
fn recursion_depth_is_positive() {
    const { assert!(MAX_RECURSION_DEPTH > 0) };
    assert_eq!(MAX_RECURSION_DEPTH, 1024);
}

#[test]
fn max_file_size_is_reasonable() {
    const { assert!(MAX_SOURCE_FILE_SIZE > 0) };
    assert_eq!(MAX_SOURCE_FILE_SIZE, 10 * 1024 * 1024); // 10 MiB
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Backward Compatibility — existing downstream crate usage patterns
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Verifies that the original API used by downstream crates still works.
/// This test mirrors the original `common_tests.rs` content.
#[test]
fn backward_compat_original_api() {
    // Original test from scaffolding
    let span = Span::new(0, 10);
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 10);

    let id = NodeId(42);
    assert_eq!(id.0, 42);

    let ident = Ident::new("x".to_string(), span);
    assert_eq!(ident.name, "x");
    assert_eq!(ident.span, span);
}

/// Verifies the `pub use techscript_common::{Ident, NodeId, Span}` pattern
/// used by techscript_ast works.
#[test]
fn backward_compat_ast_reexport_pattern() {
    // This import pattern is exactly what techscript_ast does
    use techscript_common::{Ident, NodeId, Span};

    let span = Span::new(5, 15);
    let id = NodeId(0);
    let ident = Ident::new("test".to_string(), span);

    assert_eq!(id.0, 0);
    assert_eq!(ident.span.start, 5);
}
