use darling::FromDeriveInput;
use data::SqlTable;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[allow(dead_code)]
mod data;
mod expand;
mod parse;

#[proc_macro_derive(SqlTable, attributes(sql_table, primary_key, column_type))]
pub fn sql_model_macro(input: TokenStream) -> TokenStream {
    // Parse input
    let input = parse_macro_input!(input as DeriveInput);

    // Use darling
    let receiver = match SqlTable::from_derive_input(&input) {
        Ok(receiver) => receiver,
        Err(err) => return err.write_errors().into(),
    };

    println!("{receiver:?}");

    proc_macro::TokenStream::new()
}
