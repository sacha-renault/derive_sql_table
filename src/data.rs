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
#[darling(attributes(sql_table))]
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
    pub column_type: Option<String>,

    #[darling(default)]
    pub foreign_key: Option<String>,
}
