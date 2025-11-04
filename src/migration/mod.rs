// ABOUTME: Migration utilities module
// ABOUTME: Handles schema introspection, dump/restore, and data migration

pub mod schema;

pub use schema::{list_databases, list_tables, DatabaseInfo, TableInfo};
