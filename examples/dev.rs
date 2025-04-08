use derive_sql_table::SqlTable;

struct NaiveDateTime {}

#[derive(SqlTable)]
struct ComplexTable {
    #[sql_column(primary_key, auto_increment)]
    id: i32,

    #[sql_column(foreign_key = "users.id", on_delete = "CASCADE")]
    user_id: i32,

    #[sql_column(unique, not_null)]
    email: String,

    #[sql_column(default = "NOW()")]
    created_at: String,
}

fn main() {
    println!("{}", ComplexTable::create_table_query());
}
