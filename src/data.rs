use darling::{ast::Data, FromDeriveInput, FromField};
use syn::{Ident, Type};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(sql_table), supports(struct_named))]
pub struct SqlTable {
    /// Name of of column
    pub ident: Ident,

    pub data: Data<(), TableColumn>,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub create_if_exists: bool,
}

#[derive(Debug, FromField)]
#[darling(attributes(sql_column))]
pub struct TableColumn {
    /// Name of of column
    pub ident: Option<Ident>,

    /// Rust type
    pub ty: Type,

    ///
    /// OTHER OPTIONS
    ///

    #[darling(default)]
    pub primary_key: bool,

    #[darling(default)]
    pub not_null: bool,

    #[darling(default)]
    pub unique: bool,

    #[darling(default)]
    pub auto_increment: bool,

    #[darling(default)]
    pub column_type: Option<String>,

    #[darling(default)]
    pub foreign_key: Option<String>,

    #[darling(default)]
    pub default: Option<String>,

    #[darling(default)]
    pub on_update: Option<String>,

    #[darling(default)]
    pub on_delete: Option<String>,

    ///
    /// OPTION FOR GENERATING GETTER AND EVERYTHING
    ///

    #[darling(default)]
    pub getter: bool,

    #[darling(default)]
    pub exclude_insert: bool,
}

pub struct ForeignKeyConstraint {
    pub field_name: String,
    pub referenced_table: String,
    pub referenced_column: String,
    pub on_delete: Option<String>,
    pub on_update: Option<String>,
}
