// ABOUTME: Replication utilities module
// ABOUTME: Handles PostgreSQL logical replication setup and monitoring

pub mod publication;

pub use publication::{create_publication, list_publications, drop_publication};
