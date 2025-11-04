// ABOUTME: Privilege checking utilities for migration prerequisites
// ABOUTME: Validates source and target databases have required permissions

use anyhow::{Context, Result};
use tokio_postgres::Client;

pub struct PrivilegeCheck {
    pub has_replication: bool,
    pub has_create_db: bool,
    pub has_create_role: bool,
    pub is_superuser: bool,
}

/// Check if connected user has replication privileges (needed for source)
pub async fn check_source_privileges(client: &Client) -> Result<PrivilegeCheck> {
    let row = client
        .query_one(
            "SELECT rolreplication, rolcreatedb, rolcreaterole, rolsuper
             FROM pg_roles
             WHERE rolname = current_user",
            &[],
        )
        .await
        .context("Failed to query user privileges")?;

    Ok(PrivilegeCheck {
        has_replication: row.get(0),
        has_create_db: row.get(1),
        has_create_role: row.get(2),
        is_superuser: row.get(3),
    })
}

/// Check if connected user has sufficient privileges for target
pub async fn check_target_privileges(client: &Client) -> Result<PrivilegeCheck> {
    // Same query as source
    check_source_privileges(client).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postgres::connect;

    #[tokio::test]
    #[ignore]
    async fn test_check_source_privileges() {
        let url = std::env::var("TEST_SOURCE_URL").unwrap();
        let client = connect(&url).await.unwrap();

        let privileges = check_source_privileges(&client).await.unwrap();

        // Should have at least one privilege
        assert!(
            privileges.has_replication || privileges.is_superuser,
            "Source user should have REPLICATION privilege or be superuser"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_target_privileges() {
        let url = std::env::var("TEST_TARGET_URL").unwrap();
        let client = connect(&url).await.unwrap();

        let privileges = check_target_privileges(&client).await.unwrap();

        // Should have create privileges or be superuser
        assert!(
            privileges.has_create_db || privileges.is_superuser,
            "Target user should have CREATE DATABASE privilege or be superuser"
        );
    }
}
