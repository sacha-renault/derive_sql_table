use derive_sql_table::SqlTable;

struct NaiveDateTime {}

#[derive(SqlTable)]
#[sql_table(name = "users", create_if_exists)]
struct User {
    #[primary_key]
    pub id: i64,

    #[column_type(value = "TIMESTAMP")]
    pub created_at: NaiveDateTime,
}

fn main() {}
