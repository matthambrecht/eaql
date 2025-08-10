use eaql::validator::engine;

// Database Query Tests (Validator)
// Normal
#[test]
fn integration_test_db_create_normal() {
    assert_eq!(engine("create database test;"), true);
    assert_eq!(engine("Create the database test!"), true);
    assert_eq!(engine("Make the database test."), true);
}

#[test]
fn integration_test_db_use_normal() {
    assert_eq!(engine("use database test;"), true);
    assert_eq!(engine("Enter the database test!"), true);
    assert_eq!(engine("enter the database test."), true);
}

#[test]
fn integration_test_db_show_normal() {
    assert_eq!(engine("show database;"), true);
    assert_eq!(engine("List databases!"), true);
    assert_eq!(engine("List the databases."), true);
}

#[test]
fn integration_test_db_destroy_normal() {
    assert_eq!(engine("delete databases db1, db2, db3;"), true);
    assert_eq!(engine("Delete database db1!"), true);
    assert_eq!(engine("Destroy the databases db1, db2 and db3."), true);
}

// Error
#[test]
fn integration_test_db_create_error() {
    assert_eq!(engine("create database test"), false);
    assert_eq!(engine("Create the test!"), false);
    assert_eq!(engine("Make the database \"test\"."), false);
}

#[test]
fn integration_test_db_use_error() {
    assert_eq!(engine("use database test"), false);
    assert_eq!(engine("Enter test!"), false);
    assert_eq!(engine("enter the database \"test\"."), false);
}

#[test]
fn integration_test_db_show_error() {
    assert_eq!(engine("show database"), false);
    assert_eq!(engine("show!"), false);
    assert_eq!(engine("List the \"databases\"."), false);
}

#[test]
fn integration_test_db_destroy_error() {
    assert_eq!(engine("delete databases db1, db2, db3"), false);
    assert_eq!(engine("Delete db1!"), false);
    assert_eq!(engine("Destroy the databases db1, \"db2\" and db3."), false);
}