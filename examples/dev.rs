use derive_sql_table::SqlTable;

struct NaiveDateTime {}

#[derive(SqlTable)]
#[sql_table(name = "users", create_if_exists)]
struct User {
    #[sql_column(primary_key, auto_increment)]
    pub id: i64,

    #[sql_column(column_type = "TIMESTAMP")]
    pub created_at: NaiveDateTime,

    #[sql_column(foreign_key = "role.id")]
    pub role_id: i64,
}

fn main() {
    println!("{}", User::create_table_query());
}
