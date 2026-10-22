//! # WebForge SQLite Database Persistence Engine
//!
//! Provides zero-config, embedded SQLite database management, safe schema
//! initialization, and strictly parameterized queries to prevent SQL injection.

use techscript_runtime::value::DslBlockValue;

pub const DB_FILENAME: &str = "webforge.db";

/// Sanitize an identifier (table or column name) to allow only alphanumeric ASCII and underscores.
pub fn sanitize_identifier(raw: &str) -> String {
    let clean: String = raw
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    if clean.is_empty() {
        "records".to_string()
    } else {
        clean
    }
}

/// Open an SQLite connection to the standard webforge database.
pub fn open_connection() -> Result<rusqlite::Connection, rusqlite::Error> {
    rusqlite::Connection::open(DB_FILENAME)
}

/// Initialize SQLite schema from a declared TechScript `database` DSL block.
pub fn init_sqlite_database(dsl: &DslBlockValue) {
    if let Ok(conn) = open_connection() {
        for child in &dsl.children {
            if child.kind == "table" {
                let mut table_name = child.args.first().map(|a| a.to_string());
                if table_name.is_none() {
                    for p in &child.properties {
                        if p.name == "name" || p.name == "id" {
                            table_name = p.value.as_ref().map(|v| v.to_string());
                            break;
                        }
                    }
                }
                let table_name = table_name.unwrap_or_else(|| "records".to_string());

                let clean_name = sanitize_identifier(&table_name);
                let create_sql = format!(
                    "CREATE TABLE IF NOT EXISTS {} (\
                        id INTEGER PRIMARY KEY AUTOINCREMENT, \
                        title TEXT, \
                        content TEXT, \
                        created_at DATETIME DEFAULT CURRENT_TIMESTAMP\
                    )",
                    clean_name
                );
                let _ = conn.execute(&create_sql, []);
            }
        }
    }
}

/// Ensure a table exists before read/write operations.
pub fn ensure_table_exists(
    conn: &rusqlite::Connection,
    table_name: &str,
) -> Result<(), rusqlite::Error> {
    let clean_name = sanitize_identifier(table_name);
    let create_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (\
            id INTEGER PRIMARY KEY AUTOINCREMENT, \
            title TEXT, \
            content TEXT, \
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP\
        )",
        clean_name
    );
    conn.execute(&create_sql, [])?;
    Ok(())
}

/// Insert a record safely using parameterized SQL statements.
pub fn insert_record(table_name: &str, title: &str, content: &str) -> Result<i64, rusqlite::Error> {
    let conn = open_connection()?;
    let clean_name = sanitize_identifier(table_name);
    ensure_table_exists(&conn, &clean_name)?;

    let insert_sql = format!(
        "INSERT INTO {} (title, content) VALUES (?1, ?2)",
        clean_name
    );
    conn.execute(&insert_sql, rusqlite::params![title, content])?;
    Ok(conn.last_insert_rowid())
}

/// Query the most recent records from a table safely using parameterized SQL statements.
pub fn query_records(
    table_name: &str,
    limit: usize,
) -> Result<Vec<serde_json::Value>, rusqlite::Error> {
    let conn = open_connection()?;
    let clean_name = sanitize_identifier(table_name);
    ensure_table_exists(&conn, &clean_name)?;

    let query_sql = format!(
        "SELECT id, title, content, created_at FROM {} ORDER BY id DESC LIMIT ?1",
        clean_name
    );
    let mut stmt = conn.prepare(&query_sql)?;
    let rows = stmt.query_map(rusqlite::params![limit as i64], |row| {
        let id: i64 = row.get(0)?;
        let title: String = row.get(1).unwrap_or_default();
        let content: String = row.get(2).unwrap_or_default();
        let created_at: String = row.get(3).unwrap_or_default();
        Ok(serde_json::json!({
            "id": id,
            "title": title,
            "content": content,
            "created_at": created_at,
        }))
    })?;

    let mut result = Vec::new();
    for r in rows.flatten() {
        result.push(r);
    }
    Ok(result)
}
