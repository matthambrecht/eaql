use eaql::transpiler::engine;

// Database Query Tests (Validator)
// Normal
#[test]
fn transpile_integration_test_db_create_normal() {
    // Create keyword tests
    assert_eq!(
        engine("create database test;"),
        Ok("CREATE DATABASE test;".to_string())
    );
    assert_eq!(
        engine("make database test."),
        Ok("CREATE DATABASE test;".to_string())
    );
}

#[test]
fn transpile_integration_test_db_use_normal() {
    // Use keyword tests
    assert_eq!(
        engine("use database test;"),
        Ok("USE DATABASE test;".to_string())
    );
    assert_eq!(
        engine("enter database test;"),
        Ok("USE DATABASE test;".to_string())
    );
}

#[test]
fn transpile_integration_test_db_show_normal() {
    // Show keyword tests
    assert_eq!(engine("show database;"), Ok("SHOW DATABASES;".to_string()));
    assert_eq!(engine("list databases."), Ok("SHOW DATABASES;".to_string()));
}

#[test]
fn transpile_integration_test_db_destroy_normal() {
    // Destroy
    assert_eq!(
        engine("remove database db1!"),
        Ok("DROP DATABASE db1;".to_string())
    );
    assert_eq!(
        engine("destroy database db1!"),
        Ok("DROP DATABASE db1;".to_string())
    );
    assert_eq!(
        engine("delete database db1!"),
        Ok("DROP DATABASE db1;".to_string())
    );

    // Multiple databases tests
    assert_eq!(
        engine("delete databases db1, db2, db3;"),
        Ok("DROP DATABASE db1, db2, db3;".to_string())
    );
    assert_eq!(
        engine("Destroy the databases db1, db2 and db3."),
        Ok("DROP DATABASE db1, db2, db3;".to_string())
    );
}

// Error
#[test]
fn transpile_integration_test_db_create_error() {
    // Generic Error Test
    assert!(engine("create database test").is_err());
    assert!(engine("Create the test!").is_err());
    assert!(engine("Make the database \"test\".").is_err());
}

#[test]
fn transpile_integration_test_db_use_error() {
    // Generic Error Test
    assert!(engine("use database test").is_err());
    assert!(engine("Enter test!").is_err());
    assert!(engine("enter the database \"test\".").is_err());
}

#[test]
fn transpile_integration_test_db_show_error() {
    // Generic Error Test
    assert!(engine("show database").is_err());
    assert!(engine("show!").is_err());
    assert!(engine("List the \"databases\".").is_err());
}

#[test]
fn transpile_integration_test_db_destroy_error() {
    // Generic Error Test
    assert!(engine("delete databases db1, db2, db3").is_err());
    assert!(engine("Delete db1!").is_err());
    assert!(engine("Destroy the databases db1, \"db2\" and db3.").is_err());
}

// Table Accessor Query Tests (Validator)
// Normal
#[test]
fn transpile_integration_test_table_accessor_normal_get() {
    // Test "wildcard" keywords
    assert_eq!(
        engine("get all from test_table;"),
        Ok("SELECT * FROM test_table;".to_string())
    );
    assert_eq!(
        engine("get any from test_table;"),
        Ok("SELECT * FROM test_table;".to_string())
    );
    assert_eq!(
        engine("get everything from test_table;"),
        Ok("SELECT * FROM test_table;".to_string())
    );

    // Test "get" keywords
    assert_eq!(
        engine("get all from test_table;"),
        Ok("SELECT * FROM test_table;".to_string())
    );
    assert_eq!(
        engine("retrieve all from test_table;"),
        Ok("SELECT * FROM test_table;".to_string())
    );
    assert_eq!(
        engine("find all from test_table;"),
        Ok("SELECT * FROM test_table;".to_string())
    );

    // Test column listing
    assert_eq!(
        engine("get me id and value from test_table."),
        Ok("SELECT id, value FROM test_table;".to_string())
    );
    assert_eq!(
        engine("get me id, price, value from test_table!"),
        Ok("SELECT id, price, value FROM test_table;".to_string())
    );
}

#[test]
fn transpile_integration_test_table_accessor_normal_filter() {
    // Test "filter entrance" keywords
    assert_eq!(
        engine("get all from test_table where id = 3;"),
        Ok("SELECT * FROM test_table WHERE id = 3;".to_string())
    );
    assert_eq!(
        engine("get all from test_table wherever id = 3;"),
        Ok("SELECT * FROM test_table WHERE id = 3;".to_string())
    );
    assert_eq!(
        engine("get all from test_table whenever id = 3;"),
        Ok("SELECT * FROM test_table WHERE id = 3;".to_string())
    );

    // Test different conditionals
    assert_eq!(
        engine("get all from test_table where id = 3 and price = 2.0."),
        Ok("SELECT * FROM test_table WHERE id = 3 and price = 2.0;".to_string())
    );
    assert_eq!(
        engine("get all from test_table where id = 3 or (price <= 2 and name is \"3\")!"),
        Ok("SELECT * FROM test_table WHERE id = 3 or (price <= 2 and name = \"3\");".to_string())
    );
    assert_eq!(engine("get all from test_table where (price < 3 or name is \"test\" and (id = 3 or (value < 4 and time >= 5)));"),
    Ok("SELECT * FROM test_table WHERE (price < 3 or name = \"test\" and (id = 3 or (value < 4 and time >= 5)));".to_string()));
}

#[test]
fn transpile_integration_test_table_accessor_normal_postprocessor() {
    // Test "post-processor entrance" keywords
    assert_eq!(
        engine("get all from test_table then limit 5;"),
        Ok("SELECT * FROM test_table LIMIT 5;".to_string())
    );
    assert_eq!(
        engine("get all from test_table afterwords limit 5;"),
        Ok("SELECT * FROM test_table LIMIT 5;".to_string())
    );
    assert_eq!(
        engine("get all from test_table after limit 5;"),
        Ok("SELECT * FROM test_table LIMIT 5;".to_string())
    );

    // Test post-processor w/ filter
    assert_eq!(
        engine(
            "get all from test_table where id = 3 or (price <= 2 and name is \"3\") then limit 5."
        ),
        Ok(
            "SELECT * FROM test_table WHERE id = 3 or (price <= 2 and name = \"3\") LIMIT 5;"
                .to_string()
        )
    );
}

#[test]
fn transpile_integration_test_table_accessor_normal_postprocessor_limit() {
    // Test "post-processor limit" keywords
    assert_eq!(
        engine("get all from test_table then limit 5;"),
        Ok("SELECT * FROM test_table LIMIT 5;".to_string())
    );
    assert_eq!(
        engine("get all from test_table then limit it to 5;"),
        Ok("SELECT * FROM test_table LIMIT 5;".to_string())
    );
}

// Error
#[test]
fn transpile_integration_test_table_accessor_error_get() {
    // Test improper tokens
    assert!(engine("get all from \"test_table\";").is_err());
    assert!(engine("get all from test_table").is_err());
    assert!(engine("get everything in test_table;").is_err());

    // Test invalid column listing
    assert!(engine("get me id and \"value\" from test_table.").is_err());
    assert!(engine("get me id, price value from test_table!").is_err());
}

#[test]
fn transpile_integration_test_table_accessor_error_filter() {
    // Test bad conditions
    assert!(engine("get all from test_table where id = 3").is_err());
    assert!(engine("get all from test_table where id is equal to 3;").is_err());
    assert!(engine("get all from test_table where id <== 3;").is_err());

    // Test different bad conditionals
    assert!(
        engine("get all from test_table where id = 3 or (price <= 2 and name is id)!").is_err()
    );
    assert!(engine("get all from test_table where (price < 3 or name is \"test\" and (id = 3 or (value < 4 and time >= 5)))));").is_err());
    assert!(engine("get all from test_table where price < 3 or name is \"test\" and (id = 3 or (value < 4 and time >= 5)))));").is_err());
}

#[test]
fn transpile_integration_test_table_accessor_error_postprocessor() {
    // Generic tests
    assert!(engine("get all from test_table then limit 5").is_err());

    // Test bad postprocessors
    assert!(engine("get all from test_table afterwords is 5;").is_err());

    // Test post-processor befor filter
    assert!(
        engine(
            "get all from test_table then limit 5 where id = 3 or (price <= 2 and name is \"3\")."
        )
        .is_err()
    );
}

#[test]
fn transpile_integration_test_table_accessor_error_postprocessor_limit() {
    // Test bad limit
    assert!(engine("get all from test_table then limit = 5;").is_err());
    assert!(engine("get all from test_table then limit id;").is_err());
}
