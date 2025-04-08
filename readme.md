# derive_sql_table

A simple Rust macro for generating SQL table definitions from structs.

## What it does

This crate provides a `#[derive(SqlTable)]` macro that lets you define database tables directly in your Rust code. It generates CREATE TABLE statements from your struct definitions.

## Usage

```rust
use derive_sql_table::SqlTable;

#[derive(SqlTable)]
#[sql_table(name = "users", if_not_exists)]
struct User {
    #[sql_column(primary_key, auto_increment)]
    id: i32,
    
    #[sql_column(not_null, unique)]
    username: String,
    
    #[sql_column(not_null)]
    email: String,
    
    #[sql_column(foreign_key = "roles.id", on_delete = "CASCADE")]
    role_id: i32,
}
```

## Features

- **Table attributes**:
  - `name` - Custom table name (defaults to snake_case of struct name)
  - `if_not_exists` - Add IF NOT EXISTS to CREATE TABLE statement

- **Column attributes**:
  - `primary_key` - Mark field as primary key
  - `not_null` - Add NOT NULL constraint
  - `unique` - Add UNIQUE constraint
  - `auto_increment` - Add AUTOINCREMENT
  - `column_type` - Override the SQL column type
  - `foreign_key` - Reference another table in "table.column" format
  - `default` - Set DEFAULT value
  - `on_update` - ON UPDATE action for foreign keys
  - `on_delete` - ON DELETE action for foreign keys

## Why use this?

- No ORM overhead
- Works well with SQLx or other SQL libraries
- Single source of truth for your schema
- No duplication of model definitions

## License

MIT
