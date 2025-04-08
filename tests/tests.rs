use derive_sql_table::SqlTable;

#[test]
fn test_table_name() {
    #[derive(SqlTable)]
    struct User {}

    assert_eq!(User::table_name(), "user");
}

#[test]
fn test_custom_table_name() {
    #[derive(SqlTable)]
    #[sql_table(name = "custom_users")]
    struct User {}

    assert_eq!(User::table_name(), "custom_users");
}

#[test]
fn test_generated_code_syntax() {
    struct NaiveDateTime {}

    // This test checks if the generated code compiles without errors
    #[derive(SqlTable)]
    #[sql_table(if_not_exists)]
    #[allow(unused)]
    struct ComplexTable {
        #[sql_column(primary_key, auto_increment)]
        id: i32,

        #[sql_column(foreign_key = "users.id", on_delete = "CASCADE")]
        user_id: i32,

        #[sql_column(unique, not_null)]
        email: String,

        #[sql_column(default = "NOW()", column_type = "TIMESTAMP")]
        created_at: NaiveDateTime,
    }

    // Just calling these functions to verify they compile correctly
    let create_query = ComplexTable::create_table_query();

    // Assert
    assert!(create_query.contains("CREATE TABLE IF NOT EXISTS complex_table (\n"));
    assert!(create_query.contains("id INTEGER AUTOINCREMENT,\n"));
    assert!(create_query.contains("email TEXT NOT NULL UNIQUE,\n"));
    assert!(create_query.contains("created_at TIMESTAMP DEFAULT NOW(),\n"));
    assert!(create_query.contains("PRIMARY KEY (id),\n"));
    assert!(create_query.contains("FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE"));
}
