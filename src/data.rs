use darling::{ast::Data, FromDeriveInput, FromField};
use syn::{Ident, Type};

/// Represents a struct that will be mapped to an SQL table
///
/// This struct is used by the proc macro system to parse attributes from
/// a struct marked with #[derive(SqlTable)]. It captures table-level
/// configuration like the table name and if_not_exists option.
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(sql_table), supports(struct_named))]
pub struct SqlTable {
    /// Identifier (name) of the struct being processed
    pub ident: Ident,

    /// Data from the struct, containing its fields as TableColumn instances
    pub data: Data<(), TableColumn>,

    /// Optional custom name for the SQL table
    /// If not provided, will default to the snake_case version of the struct name
    #[darling(default)]
    pub name: Option<String>,

    /// Whether to include "IF NOT EXISTS" in the CREATE TABLE statement
    /// Defaults to false
    #[darling(default)]
    pub if_not_exists: bool,
}

/// Represents a field within a struct that will be mapped to a column in an SQL table
///
/// This struct is used by the darling library to parse field-level attributes.
/// It captures various SQL column constraints and properties like primary key,
/// nullability, uniqueness, etc.
#[derive(Debug, FromField)]
#[darling(attributes(sql_column))]
pub struct TableColumn {
    /// Optional identifier (name) of the field
    /// This will be used as the column name in the SQL table
    pub ident: Option<Ident>,

    /// Rust type of the field
    /// Used to determine the SQL type for the column
    pub ty: Type,

    /// Whether this column is a primary key
    /// Defaults to false
    #[darling(default)]
    pub primary_key: bool,

    /// Whether this column has a NOT NULL constraint
    /// Defaults to false
    #[darling(default)]
    pub not_null: bool,

    /// Whether this column has a UNIQUE constraint
    /// Defaults to false
    #[darling(default)]
    pub unique: bool,

    /// Whether this column is auto-incrementing
    /// Defaults to false
    #[darling(default)]
    pub auto_increment: bool,

    /// Optional override for the SQL column type
    /// If not provided, will be derived from the Rust type
    #[darling(default)]
    pub column_type: Option<String>,

    /// Optional foreign key reference in the format "table.column"
    /// Used to generate FOREIGN KEY constraints
    #[darling(default)]
    pub foreign_key: Option<String>,

    /// Optional DEFAULT value for the column
    #[darling(default)]
    pub default: Option<String>,

    /// Optional ON UPDATE action for foreign keys
    /// Common values: "CASCADE", "RESTRICT", "SET NULL"
    #[darling(default)]
    pub on_update: Option<String>,

    /// Optional ON DELETE action for foreign keys
    /// Common values: "CASCADE", "RESTRICT", "SET NULL"
    #[darling(default)]
    pub on_delete: Option<String>,
}

/// Represents a foreign key constraint in an SQL table
///
/// This struct is used to store information about foreign key relationships
/// between tables. It includes the referencing field, the referenced table and column,
/// and the ON DELETE and ON UPDATE actions.
pub struct ForeignKeyConstraint {
    /// Name of the field in the current table that references another table
    pub field_name: String,

    /// Name of the table being referenced
    pub referenced_table: String,

    /// Name of the column being referenced in the referenced table
    pub referenced_column: String,

    /// Optional ON DELETE action (CASCADE, RESTRICT, SET NULL, etc.)
    pub on_delete: Option<String>,

    /// Optional ON UPDATE action (CASCADE, RESTRICT, SET NULL, etc.)
    pub on_update: Option<String>,
}
