use eaql::validator::engine;

// Database Query Tests (Validator)
// Normal
#[test]
fn integration_test_db_create_normal() {
    // Create keyword tests
    assert_eq!(engine("create database test;"), true);
    assert_eq!(engine("make database test."), true);
}

#[test]
fn integration_test_db_use_normal() {
    // Use keyword tests
    assert_eq!(engine("use database test;"), true);
    assert_eq!(engine("enter database test;"), true);
}

#[test]
fn integration_test_db_show_normal() {
    // Show keyword tests
    assert_eq!(engine("show database;"), true);
    assert_eq!(engine("list databases."), true);
}

#[test]
fn integration_test_db_destroy_normal() {
    // Destroy
    assert_eq!(engine("remove database db1!"), true);
    assert_eq!(engine("destroy database db1!"), true);
    assert_eq!(engine("delete database db1!"), true);
    
    // Multiple databases tests
    assert_eq!(engine("delete databases db1, db2, db3;"), true);
    assert_eq!(engine("Destroy the databases db1, db2 and db3."), true);
}

// Error
#[test]
fn integration_test_db_create_error() {
    // Generic Error Test
    assert_eq!(engine("create database test"), false);
    assert_eq!(engine("Create the test!"), false);
    assert_eq!(engine("Make the database \"test\"."), false);
}

#[test]
fn integration_test_db_use_error() {
    // Generic Error Test
    assert_eq!(engine("use database test"), false);
    assert_eq!(engine("Enter test!"), false);
    assert_eq!(engine("enter the database \"test\"."), false);
}

#[test]
fn integration_test_db_show_error() {
    // Generic Error Test
    assert_eq!(engine("show database"), false);
    assert_eq!(engine("show!"), false);
    assert_eq!(engine("List the \"databases\"."), false);
}

#[test]
fn integration_test_db_destroy_error() {
    // Generic Error Test
    assert_eq!(engine("delete databases db1, db2, db3"), false);
    assert_eq!(engine("Delete db1!"), false);
    assert_eq!(engine("Destroy the databases db1, \"db2\" and db3."), false);
}


// Table Accessor Query Tests (Validator)
// Normal
#[test]
fn integration_test_table_accessor_normal_get() {
    // Test "wildcard" keywords
    assert_eq!(engine("get all from test_table;"), true);
    assert_eq!(engine("get any from test_table;"), true);
    assert_eq!(engine("get everything from test_table;"), true);
    
    // Test "get" keywords
    assert_eq!(engine("get all from test_table;"), true);
    assert_eq!(engine("retrieve all from test_table;"), true);
    assert_eq!(engine("find all from test_table;"), true);

    // Test column listing
    assert_eq!(engine("get me id and value from test_table."), true);
    assert_eq!(engine("get me id, price, value from test_table!"), true);
}

#[test]
fn integration_test_table_accessor_normal_filter() {
    // Test "filter entrance" keywords
    assert_eq!(engine("get all from test_table where id = 3;"), true);
    assert_eq!(engine("get all from test_table wherever id = 3;"), true);
    assert_eq!(engine("get all from test_table whenever id = 3;"), true);
    
    // Test different conditionals
    assert_eq!(engine("get all from test_table where id = 3 and price = 2.0."), true);
    assert_eq!(engine("get all from test_table where id = 3 or (price <= 2 and name is \"3\")!"), true);
    assert_eq!(engine("get all from test_table where (price < 3 or name is \"test\" and (id = 3 or (value < 4 and time >= 5));"), true);
}


#[test]
fn integration_test_table_accessor_normal_postprocessor() {
    // Test "post-processor entrance" keywords
    assert_eq!(engine("get all from test_table then limit 5;"), true);
    assert_eq!(engine("get all from test_table afterwords limit 5;"), true);
    assert_eq!(engine("get all from test_table after limit 5;"), true);

    // Test post-processor w/ filter
    assert_eq!(engine("get all from test_table where id = 3 or (price <= 2 and name is \"3\") then limit 5."), true);
}


#[test]
fn integration_test_table_accessor_normal_postprocessor_limit() {
    // Test "post-processor limit" keywords
    assert_eq!(engine("get all from test_table then limit 5;"), true);
    assert_eq!(engine("get all from test_table then limit it to 5;"), true);
}

// Error
#[test]
fn integration_test_table_accessor_error_get() {
    // Test improper tokens
    assert_eq!(engine("get all from \"test_table\";"), false);
    assert_eq!(engine("get all from test_table"), false);
    assert_eq!(engine("get everything in test_table;"), false);
    
    // Test invalid column listing
    assert_eq!(engine("get me id and \"value\" from test_table."), false);
    assert_eq!(engine("get me id, price value from test_table!"), false);
}

#[test]
fn integration_test_table_accessor_error_filter() {
    // Test bad conditions
    assert_eq!(engine("get all from test_table where id = 3"), false);
    assert_eq!(engine("get all from test_table where id is equal to 3;"), false);
    assert_eq!(engine("get all from test_table where id <== 3;"), false);
    
    // Test different bad conditionals
    assert_eq!(engine("get all from test_table where id = 3 or (price <= 2 and name is id)!"), false);

    // Bug that needs to be fixed
    assert_eq!(engine("get all from test_table where (price < 3 or name is \"test\" and (id = 3 or (value < 4 and time >= 5)));"), false);
}

#[test]
fn integration_test_table_accessor_error_postprocessor() {
    // Generic tests
    assert_eq!(engine("get all from test_table then limit 5"), false);

    // Test bad postprocessors
    assert_eq!(engine("get all from test_table afterwords is 5;"), false);

    // Test post-processor befor filter
    assert_eq!(engine("get all from test_table then limit 5 where id = 3 or (price <= 2 and name is \"3\")."), false);
}

#[test]
fn integration_test_table_accessor_error_postprocessor_limit() {
    // Test bad limit
    assert_eq!(engine("get all from test_table then limit = 5;"), false);
    assert_eq!(engine("get all from test_table then limit id;"), false);
}