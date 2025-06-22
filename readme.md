# RSQL 🦀

A lightweight, in-memory SQL-like database engine built in Rust, with persistent storage via JSON files. Supports basic SQL-like commands such as `CREATE TABLE`, `INSERT`, `SELECT`, and `DROP`.

## 🛠 Features

- Create and manage multiple tables
- Insert rows into tables
- Query specific columns or all data with `SELECT`
- Drop tables
- Data is persisted to `db.json` on disk
- Human-readable and editable JSON storage

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## 🚀 Getting Started

Clone the repo and build the project:

```bash
git clone https://github.com/Shan-N/rustySQL.git
cd rustySQL
cargo build --release
./target/release/rsql
```

> The database will automatically load from `db.json` if it exists.

## 🧪 Example Usage

```text
Welcome to RSQL
rsql:>
CREATE TABLE users (id, name, email)
Table users created!

rsql:>
INSERT INTO users VALUES (1, Alice, alice@example.com)
Row inserted into table 'users'

rsql:>
SELECT * FROM users
Table: users
["id", "name", "email"]
["1", "Alice", "alice@example.com"]

rsql:>
SELECT name, email FROM users
Table: users
["name", "email"]
["Alice", "alice@example.com"]

rsql:>
DROP TABLE users
Table 'users' is dropped!

rsql:>
EXIT
db.json saved locally
Exiting...
```

## 💾 Data Persistence

All tables and data are saved to `db.json` automatically:
- On `EXIT`
- After every `CREATE`, `INSERT`, or `DROP`

The format is JSON and can be manually inspected or modified if needed.

Example `db.json`:
```json
{
  "tables": {
    "users": {
      "name": "users",
      "columns": ["id", "name", "email"],
      "rows": [
        {
          "id": "1",
          "name": "Alice",
          "email": "alice@example.com"
        }
      ]
    }
  }
}
```

## 🧰 Dependencies

This project uses:

- [`serde`](https://crates.io/crates/serde)
- [`serde_json`](https://crates.io/crates/serde_json)
- [`serde_derive`](https://crates.io/crates/serde_derive)

Add to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 📚 Future Work

- Add `UPDATE` and `DELETE` commands
- Query filtering (`WHERE` clause)
- Type checking (e.g., integers, strings)
- Better error messages
- API Support
- Tauri based GUI


## 📄 License

MIT © 2025 Shantanu
