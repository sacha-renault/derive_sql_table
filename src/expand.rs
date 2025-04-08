use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::data::{ForeignKeyConstraint, SqlTable, TableColumn};

/// Converts a Rust type to the corresponding SQL type
///
/// Maps common Rust types to their SQL equivalents:
/// - String types to TEXT
/// - Integer types (i8, i16, i32) to INTEGER
/// - Larger integers (i64, u64) to BIGINT
/// - Floating point types to REAL
/// - Boolean to BOOLEAN
/// - UUID to UUID
/// - All other types default to TEXT
fn rust_type_to_sql_type(ty: &Type) -> String {
    let type_str = quote! { #ty }.to_string();

    match type_str.as_str() {
        "String" | "& str" | "& String" => "TEXT".to_string(),
        "i32" | "i16" | "i8" => "INTEGER".to_string(),
        "i64" => "BIGINT".to_string(),
        "u32" | "u16" | "u8" => "INTEGER".to_string(),
        "u64" => "BIGINT".to_string(),
        "f32" | "f64" => "REAL".to_string(),
        "bool" => "BOOLEAN".to_string(),
        "uuid :: Uuid" => "UUID".to_string(),
        _ => "TEXT".to_string(),
    }
}

/// Processes foreign key definitions from table column attributes
///
/// Takes a TableColumn that may have a foreign key reference and converts it
/// into a ForeignKeyConstraint with proper validation.
///
/// # Arguments
///
/// * `field` - A TableColumn that may contain a foreign key reference
///
/// # Returns
///
/// * `Option<ForeignKeyConstraint>` - A structured foreign key constraint if one exists
///
/// # Panics
///
/// Panics if the foreign key reference format is invalid. Expected format is "table.column".
fn handle_foreign_key(field: &TableColumn) -> Option<ForeignKeyConstraint> {
    // Only process if field has a foreign key reference
    if let Some(foreign_ref) = &field.foreign_key {
        // Get the field name
        let field_name = field.ident.as_ref()?.to_string();

        // Split by dot to get table and column name
        let parts: Vec<&str> = foreign_ref.split('.').collect();

        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            // Invalid foreign key format - should be "table.column"
            panic!(
                "Invalid foreign key format '{}' : '{}'. Expected format: 'table.column'",
                field_name, foreign_ref
            );
        }

        Some(ForeignKeyConstraint {
            field_name,
            referenced_table: parts[0].to_string(),
            referenced_column: parts[1].to_string(),
            on_delete: field.on_delete.clone(),
            on_update: field.on_update.clone(),
        })
    } else {
        None
    }
}

/// Converts a TableColumn to an SQL column definition string
///
/// Generates the SQL column definition including name, type, and constraints
/// such as NOT NULL, UNIQUE, AUTOINCREMENT, and DEFAULT values.
///
/// # Arguments
///
/// * `field` - The TableColumn to convert
///
/// # Returns
///
/// A string containing the SQL column definition
fn field_to_sql_column(field: &TableColumn) -> String {
    // Get the field name from ident
    let field_name = &field.ident.clone().unwrap().to_string();

    // Sql type
    let sql_type = field
        .column_type
        .clone()
        .unwrap_or_else(|| rust_type_to_sql_type(&field.ty));

    // Start building the column definition
    let mut column_def = format!("{} {}", field_name, sql_type);

    // Add constraints
    if field.not_null {
        column_def.push_str(" NOT NULL");
    }

    if field.unique {
        column_def.push_str(" UNIQUE");
    }

    if field.auto_increment {
        column_def.push_str(" AUTOINCREMENT");
    }

    if let Some(default_val) = &field.default {
        column_def.push_str(&format!(" DEFAULT {}", default_val));
    }

    column_def
}

/// Generates a complete CREATE TABLE SQL statement
///
/// Combines column definitions, primary keys, and foreign key constraints
/// into a properly formatted CREATE TABLE statement.
///
/// # Arguments
///
/// * `create_if_exists` - Whether to include "IF NOT EXISTS" in the statement
/// * `table_name` - Name of the table to create
/// * `columns` - Array of column definition strings
/// * `primary_keys` - Array of column names to use as primary keys
/// * `foreign_keys` - Array of foreign key constraints
///
/// # Returns
///
/// A string containing the complete CREATE TABLE SQL statement
fn generate_create_table_sql(
    create_if_exists: bool,
    table_name: &str,
    columns: &[String],
    primary_keys: &[String],
    foreign_keys: &[ForeignKeyConstraint],
) -> String {
    // Init the create table statement
    let mut sql = "CREATE TABLE".to_string();

    // add the if exists logic
    if create_if_exists {
        sql.push_str(" IF NOT EXISTS");
    }

    // Add columns
    sql.push_str(&format!(" {} (\n ", table_name));
    sql.push_str(&columns.join(",\n"));

    // Add primary key constraint if exists
    if !primary_keys.is_empty() {
        sql.push_str(&format!(",\nPRIMARY KEY ({})", primary_keys.join(", ")));
    }

    // Add foreign key constraints
    for fk in foreign_keys {
        let mut constraint = format!(
            ",\nFOREIGN KEY ({}) REFERENCES {}({})",
            fk.field_name, fk.referenced_table, fk.referenced_column
        );

        if let Some(on_delete) = &fk.on_delete {
            constraint.push_str(&format!(" ON DELETE {}", on_delete));
        }

        if let Some(on_update) = &fk.on_update {
            constraint.push_str(&format!(" ON UPDATE {}", on_update));
        }

        sql.push_str(&constraint);
    }

    // Close the create table statement
    sql.push_str("\n);");

    sql
}

/// Main entry point for the SqlTable derive macro
///
/// Processes a struct annotated with #[derive(SqlTable)] and generates
/// the impl block with table_name and create_table_query methods.
///
/// # Arguments
///
/// * `table_def` - The parsed SqlTable struct with field information
///
/// # Returns
///
/// A TokenStream containing the implementation for the struct
pub fn expand(table_def: SqlTable) -> TokenStream {
    // Struct name
    let struct_name = table_def.ident;

    // Get the table name
    let table_name = table_def
        .name
        .clone()
        .unwrap_or_else(|| struct_name.to_string().to_case(Case::Snake));

    // Process fields
    let fields = match &table_def.data {
        darling::ast::Data::Struct(fields) => fields,
        _ => panic!("SqlTable only supports structs with named fields"),
    };

    // init a column vec
    let mut columns = Vec::new();

    // Iterate over the fields
    for field in fields.iter() {
        // Column def
        let column_def = field_to_sql_column(field);
        columns.push(column_def);
    }

    // Get the prim keys
    let primary_keys = fields
        .iter()
        .filter(|f| f.primary_key)
        .map(|f| f.ident.clone().unwrap().to_string())
        .collect::<Vec<_>>();
    let foreign_keys = fields
        .iter()
        .filter_map(|f| handle_foreign_key(f))
        .collect::<Vec<_>>();

    // Generate the CREATE TABLE SQL
    let create_table_sql = generate_create_table_sql(
        table_def.if_not_exists,
        &table_name,
        &columns,
        &primary_keys,
        &foreign_keys,
    );

    // Generate the impl block
    quote! {
        impl #struct_name {
            pub fn create_table_query() -> String {
                #create_table_sql.to_string()
            }

            pub fn table_name() -> &'static str {
                #table_name
            }
        }
    }
}
