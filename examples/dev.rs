use derive_sql_table::SqlTable;

struct NaiveDateTime {}

#[derive(SqlTable)]
#[sql_table(name = "users", create_if_exists)]
struct User {
    #[sql_table(primary_key)]
    pub id: i64,

    #[sql_table(column_type = "TIMESTAMP")]
    pub created_at: NaiveDateTime,

    #[sql_table(foreign_key = "role.id")]
    pub role_id: i64,
}

fn main() {}
