# Database Example

This example shows how to open a database file, execute SQL commands to insert records, and query table rows in TechScript using the `sql` standard library module.

## Code (`db.txs`)
```txs
use sql

say "Opening local SQLite database..."
db = sql.open("local.db")

# Create table
sql.execute(db, "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)")

# Insert records
sql.execute(db, "INSERT INTO users (name) VALUES ('Alice')")
sql.execute(db, "INSERT INTO users (name) VALUES ('Bob')")

say "Querying users..."
rows = sql.query(db, "SELECT * FROM users ORDER BY id ASC")
for row in rows
    say $"User #{row['id']}: {row['name']}"
end

# Clean up
sql.close(db)
```

## Running the Example
```bash
tech run db.txs
```

## Expected Output
```
Opening local SQLite database...
Querying users...
User #1: Alice
User #2: Bob
```
