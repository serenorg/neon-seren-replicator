// ABOUTME: PostgreSQL utilities module
// ABOUTME: Exports connection management and common database operations

pub mod connection;
pub mod privileges;

pub use connection::connect;
pub use privileges::{check_source_privileges, check_target_privileges, PrivilegeCheck};
