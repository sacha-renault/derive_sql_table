use darling::{ast::Data, FromDeriveInput, FromField};
use syn::{Ident, Type};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(sql_table), supports(struct_named))]
pub struct SqlTable {
    pub ident: Ident,

    pub data: Data<(), TableColumn>,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub create_if_exists: bool,
}

#[derive(Debug, FromField)]
#[darling(attributes(primary_key, column_type))]
pub struct TableColumn {
    pub ident: Option<Ident>,

    pub ty: Type,

    #[darling(default)]
    pub primary_key: bool,

    #[darling(default, rename = "value")]
    pub column_type: Option<String>,
}
