use darling::FromDeriveInput;
use data::SqlTable;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod data;
mod expand;

#[proc_macro_derive(SqlTable, attributes(sql_table, sql_column))]
pub fn sql_model_macro(input: TokenStream) -> TokenStream {
    // Parse input
    let input = parse_macro_input!(input as DeriveInput);

    // Use darling
    let receiver = match SqlTable::from_derive_input(&input) {
        Ok(receiver) => receiver,
        Err(err) => return err.write_errors().into(),
    };

    let tokens = expand::expand(receiver);

    TokenStream::from(tokens)
}
