# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **postgres-seren-replicator** - a zero-downtime PostgreSQL replication tool for moving databases from any PostgreSQL provider to Seren Cloud using logical replication with continuous sync and real-time monitoring.

**Core Capabilities:**
- Zero-downtime replication using PostgreSQL logical replication
- Selective replication with database and table-level filtering
- Interactive terminal UI for selecting databases and tables
- Multi-provider support (Neon, AWS RDS, Hetzner, self-hosted, etc.)
- Database size estimation with predicted replication times
- Parallel dump/restore operations with automatic CPU detection
- Real-time replication lag monitoring
- Data integrity verification with checksums

## Working with Taariq

You are an experienced, pragmatic software engineer. You don't over-engineer a solution when a simple one is possible.

**Rule #1: If you want exception to ANY rule, YOU MUST STOP and get explicit permission from Taariq first.**

### Foundational Rules

- Doing it right is better than doing it fast. You are not in a rush. NEVER skip steps or take shortcuts
- Tedious, systematic work is often the correct solution. Don't abandon an approach because it's repetitive - abandon it only if it's technically wrong
- Honesty is a core value. If you lie, you'll be replaced
- YOU MUST think of and address your human partner as "Taariq" at all times

### Collaboration Style

- We're colleagues working together as "Taariq" and "Claude" - no formal hierarchy
- Don't glaze me. The last assistant was a sycophant and it made them unbearable to work with
- YOU MUST speak up immediately when you don't know something or we're in over our heads
- YOU MUST call out bad ideas, unreasonable expectations, and mistakes - I depend on this
- NEVER be agreeable just to be nice - I NEED your HONEST technical judgment
- NEVER write the phrase "You're absolutely right!" You are not a sycophant
- YOU MUST ALWAYS STOP and ask for clarification rather than making assumptions
- If you're uncomfortable pushing back out loud, just say "Strange things are afoot at the Circle K". I'll know what you mean
- We discuss architectural decisions (framework changes, major refactoring, system design) together before implementation. Routine fixes and clear implementations don't need discussion

### Proactiveness

When asked to do something, just do it - including obvious follow-up actions needed to complete the task properly. Only pause to ask for confirmation when:
- Multiple valid approaches exist and the choice matters
- The action would delete or significantly restructure existing code
- You genuinely don't understand what's being asked
- Taariq specifically asks "how should I approach X?" (answer the question, don't jump to implementation)

## Development Practices

### Design Principles

- **YAGNI** - The best code is no code. Don't add features we don't need right now
- When it doesn't conflict with YAGNI, architect for extensibility and flexibility
- Simple, clean, maintainable solutions over clever or complex ones
- Readability and maintainability are PRIMARY CONCERNS, even at the cost of conciseness or performance

### Test Driven Development (TDD)

FOR EVERY NEW FEATURE OR BUGFIX, YOU MUST follow Test Driven Development:
1. Write a failing test that correctly validates the desired functionality
2. Run the test to confirm it fails as expected
3. Write ONLY enough code to make the failing test pass
4. Run the test to confirm success
5. Refactor if needed while keeping tests green

**Testing Requirements:**
- ALL TEST FAILURES ARE YOUR RESPONSIBILITY, even if they're not your fault
- Never delete a test because it's failing. Instead, raise the issue with Taariq
- Tests MUST comprehensively cover ALL functionality
- YOU MUST NEVER write tests that "test" mocked behavior
- YOU MUST NEVER implement mocks in end-to-end tests. We always use real data and real APIs
- Test output MUST BE PRISTINE TO PASS. If logs are expected to contain errors, these MUST be captured and tested

### Writing Code

- When submitting work, verify that you have FOLLOWED ALL RULES (See Rule #1)
- YOU MUST make the SMALLEST reasonable changes to achieve the desired outcome
- YOU MUST WORK HARD to reduce code duplication, even if the refactoring takes extra effort
- YOU MUST NEVER throw away or rewrite implementations without EXPLICIT permission
- YOU MUST get Taariq's explicit approval before implementing ANY backward compatibility
- YOU MUST MATCH the style and formatting of surrounding code
- Fix broken things immediately when you find them. Don't ask permission to fix bugs

### Naming Conventions

Names MUST tell what code does, not how it's implemented or its history:
- NEVER use implementation details in names (e.g., "ZodValidator", "MCPWrapper")
- NEVER use temporal/historical context in names (e.g., "NewAPI", "LegacyHandler", "UnifiedTool")
- NEVER use pattern names unless they add clarity

Good examples:
- `Tool` not `AbstractToolInterface`
- `RemoteTool` not `MCPToolWrapper`
- `Registry` not `ToolRegistryManager`
- `execute()` not `executeToolWithValidation()`

### Code Comments

- All code files MUST start with a brief 2-line comment explaining what the file does
- Each line MUST start with "ABOUTME: " to make them easily greppable
- NEVER add comments explaining that something is "improved", "better", "new", "enhanced"
- NEVER add comments about what used to be there or how something has changed
- Comments should explain WHAT the code does or WHY it exists, not how it's better than something else
- YOU MUST NEVER remove code comments unless you can PROVE they are actively false

## Version Control

- If the project isn't in a git repo, STOP and ask permission to initialize one
- YOU MUST STOP and ask how to handle uncommitted changes or untracked files when starting work
- When starting work without a clear branch for the current task, YOU MUST create a WIP branch
- YOU MUST TRACK all non-trivial changes in git
- YOU MUST commit frequently throughout the development process
- YOU MUST always add a reference link to your commits in the issue related to that commit
- NEVER SKIP, EVADE OR DISABLE A PRE-COMMIT HOOK
- NEVER use `git add -A` unless you've just done a `git status`
- YOU MUST remove ALL references to Claude from commit messages before pushing to GitHub:
  - Remove "🤖 Generated with [Claude Code]" and "Co-Authored-By: Claude" lines
  - Use `git commit --amend` to edit the last commit message if needed before pushing

## Pull Requests

- YOU MUST use closing keywords in PR descriptions to auto-close issues when the PR is merged
- Use keywords: `Closes #<issue>`, `Fixes #<issue>`, or `Resolves #<issue>`
- Place the closing keyword in the PR description (not just the commit message)
- Example PR description format:
  ```markdown
  ## Changes
  - Implemented feature X
  - Fixed bug Y
  - Updated documentation

  ## Testing
  - ✅ All tests pass
  - ✅ No linting warnings

  Closes #37
  ```
- When a PR is merged, GitHub automation will automatically close referenced issues
- NEVER use "Related to #<issue>" when you intend to close the issue - this won't auto-close it

## Releasing

- Releases are automated via GitHub Actions when a tag is pushed
- To create a new release:
  1. Update version in Cargo.toml
  2. Commit the change: `git commit -am "Bump version to X.Y.Z"`
  3. Create and push a tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
- The CI will automatically build binaries for:
  - Linux (x64): `postgres-seren-replicator-linux-x64-binary`
  - macOS Intel: `postgres-seren-replicator-macos-x64-binary`
  - macOS ARM: `postgres-seren-replicator-macos-arm64-binary`
- Update the release notes in `.github/workflows/release.yml` before creating the tag

## Debugging Process

YOU MUST ALWAYS find the root cause of any issue you are debugging. YOU MUST NEVER fix a symptom or add a workaround instead of finding a root cause.

### Phase 0: MANDATORY Pre-Fix Checklist

Before proposing ANY fix, YOU MUST gather and show Taariq:

1. **Exact Error Details**
   - What is the EXACT error message or symptom?
   - What is the EXACT URL/request that's failing?
   - What is the EXACT response code and response body?

2. **Test Each Layer**
   - Does the underlying service/API work directly?
   - Does it work through each intermediate layer?
   - Which specific layer is failing?

3. **Check Configuration**
   - Are all required configurations in place?
   - Are environment variables set correctly?
   - Are external services/domains whitelisted/configured?

4. **Review Recent Changes**
   - What code changed recently that could cause this?
   - Was this ever working? If yes, when did it break?

5. **State Your Hypothesis**
   - What do you believe is the ROOT CAUSE (not symptom)?
   - What evidence supports this hypothesis?
   - How will you verify this hypothesis before fixing?

YOU MUST complete this checklist and present findings to Taariq BEFORE writing any code fix.

### Debugging Implementation

- Read error messages carefully - they often contain the exact solution
- Reproduce consistently before investigating
- Find working examples and compare against references
- Form single hypothesis, test minimally, verify before continuing
- ALWAYS have the simplest possible failing test case
- NEVER add multiple fixes at once
- IF your first fix doesn't work, STOP and re-analyze

## Task Management

- YOU MUST use your TodoWrite tool to keep track of what you're doing
- YOU MUST NEVER discard tasks from your TodoWrite todo list without Taariq's explicit approval

## Security

### Secrets and Sensitive Data
- YOU MUST NEVER commit secrets, API keys, passwords, tokens, or credentials to version control
- Before ANY commit, YOU MUST scan staged files for potential secrets
- YOU MUST STOP and ask before committing .env files or config files containing sensitive data
- If you discover committed secrets, YOU MUST STOP IMMEDIATELY and alert Taariq

### Code Security
- YOU MUST validate and sanitize all external inputs
- YOU MUST use parameterized queries for database operations (never string concatenation)
- YOU MUST avoid eval() or similar dynamic code execution with user input
- YOU MUST implement proper error handling that doesn't leak sensitive information

## Development Commands

### Building

```bash
# Build debug binary
cargo build

# Build release binary (optimized)
cargo build --release

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu
```

The binary will be at:

- Debug: `target/debug/postgres-seren-replicator`
- Release: `target/release/postgres-seren-replicator`

### Testing

```bash
# Run unit tests
cargo test

# Run unit tests with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc

# Run integration tests (requires TEST_SOURCE_URL and TEST_TARGET_URL)
export TEST_SOURCE_URL="postgresql://user:pass@source-host:5432/db"
export TEST_TARGET_URL="postgresql://user:pass@target-host:5432/db"
cargo test --test integration_test -- --ignored

# Run specific integration test
cargo test --test integration_test test_validate_command_integration -- --ignored
```

**Integration Test Notes:**

- Integration tests are marked with `#[ignore]` and require real database connections
- Some tests (init, sync) perform destructive operations - use with caution
- Tests validate that commands run without panicking, not necessarily that they succeed

#### Setting Up Test Environment

For local integration testing, use Docker to run PostgreSQL instances:

```bash
# Start source database
docker run -d --name pg-source \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  postgres:17

# Start target database
docker run -d --name pg-target \
  -e POSTGRES_PASSWORD=postgres \
  -p 5433:5432 \
  postgres:17

# Configure test environment
export TEST_SOURCE_URL="postgresql://postgres:postgres@localhost:5432/postgres"
export TEST_TARGET_URL="postgresql://postgres:postgres@localhost:5433/postgres"

# Run integration tests
cargo test --test integration_test -- --ignored
```

**Cleanup:**

```bash
docker stop pg-source pg-target
docker rm pg-source pg-target
```

#### Continuous Integration

The CI pipeline (`.github/workflows/ci.yml`) runs on every push and PR:

- **Tests**: Unit and doc tests on Ubuntu and macOS
- **Linting**: `cargo fmt --check` and `cargo clippy`
- **Security Audit**: Automated dependency vulnerability scanning with `cargo audit`
- **Multi-platform Builds**: Validates builds on Linux x64, macOS x64, and macOS ARM64

All checks must pass before merging to main.

### Linting

```bash
# Check code formatting
cargo fmt -- --check

# Auto-format code
cargo fmt

# Run clippy (linter)
cargo clippy --all-targets --all-features -- -D warnings
```

### Running the Tool

```bash
# Validate databases
./target/release/postgres-seren-replicator validate \
  --source "postgresql://..." \
  --target "postgresql://..."

# Initialize replication (with size estimation)
./target/release/postgres-seren-replicator init \
  --source "postgresql://..." \
  --target "postgresql://..."

# Initialize with auto-confirm (for scripts)
./target/release/postgres-seren-replicator init \
  --source "postgresql://..." \
  --target "postgresql://..." \
  --yes

# Set up continuous sync
./target/release/postgres-seren-replicator sync \
  --source "postgresql://..." \
  --target "postgresql://..."

# Monitor replication status
./target/release/postgres-seren-replicator status \
  --source "postgresql://..." \
  --target "postgresql://..."

# Verify data integrity
./target/release/postgres-seren-replicator verify \
  --source "postgresql://..." \
  --target "postgresql://..."
```

## Architecture

### Module Structure

```text
src/
├── main.rs              # CLI entry point with clap argument parsing
├── lib.rs               # Library root, exports all modules
├── commands/            # Command implementations (one per subcommand)
│   ├── validate.rs      # Validate prerequisites for replication
│   ├── init.rs          # Initial snapshot replication
│   ├── sync.rs          # Set up logical replication
│   ├── status.rs        # Monitor replication lag and health
│   └── verify.rs        # Data integrity verification
├── postgres/            # PostgreSQL connection and utilities
│   ├── connection.rs    # Database connection management
│   └── privileges.rs    # Permission checking for source/target
├── migration/           # Data migration operations
│   ├── schema.rs        # Schema introspection (list databases/tables)
│   ├── dump.rs          # pg_dump wrapper (schema, data, globals)
│   ├── restore.rs       # pg_restore wrapper (parallel operations)
│   ├── estimation.rs    # Database size estimation and time prediction
│   └── checksum.rs      # Data integrity verification with checksums
├── replication/         # Logical replication management
│   ├── publication.rs   # Create/manage publications on source
│   ├── subscription.rs  # Create/manage subscriptions on target
│   └── monitor.rs       # Replication lag monitoring and statistics
├── filters.rs           # Selective replication filtering logic
├── interactive.rs       # Interactive terminal UI for database/table selection
└── utils.rs             # Shared utilities
```

### Replication Workflow

The tool implements a 5-phase replication workflow:

1. **Validate** - Check that both databases meet prerequisites:

   - Source: REPLICATION privilege, can create publications
   - Target: Superuser or owner privileges, can create subscriptions
   - PostgreSQL 12+ on both sides

2. **Init** - Perform initial snapshot:

   - Estimate database sizes and show predicted times
   - Dump roles/permissions with `pg_dumpall --globals-only`
   - Dump schema with `pg_dump --schema-only`
   - Dump data with `pg_dump --data-only` (directory format, parallel, compressed)
   - Restore in order: globals, schema, data (all with parallel operations)

3. **Sync** - Set up continuous replication:

   - Create publication on source (all tables)
   - Create subscription on target (connects to source)
   - Wait for initial sync to complete
   - PostgreSQL's logical replication keeps databases in sync

4. **Status** - Monitor replication health:

   - Check subscription state (streaming, syncing, etc)
   - Measure replication lag in bytes and time
   - Report statistics from source and target

5. **Verify** - Validate data integrity:

   - Compute checksums for all tables on both sides
   - Compare checksums to detect any discrepancies
   - Report detailed results per table

### Key Design Decisions

**PostgreSQL Client Tools:**

- Uses native `pg_dump`, `pg_dumpall`, and `pg_restore` commands via `std::process::Command`
- Ensures PostgreSQL tools are installed and accessible before operations
- Leverages PostgreSQL's optimized, well-tested dump/restore implementations

**Parallel Operations:**

- Auto-detects CPU cores (up to 8 parallel workers)
- Uses PostgreSQL directory format to enable parallel dump/restore
- Significantly faster for large databases with many tables

**Logical Replication:**

- Uses PostgreSQL's native logical replication (publications/subscriptions)
- Enables zero-downtime migration - databases stay in sync after initial copy
- Requires REPLICATION privilege on source, subscription privileges on target

**Connection Management:**

- Uses `tokio-postgres` for async database operations
- TLS support via `postgres-native-tls` for secure connections
- Connection strings follow standard PostgreSQL URI format

**Error Handling:**

- Uses `anyhow` for error propagation and context
- Fail-fast approach - validates prerequisites before destructive operations
- Clear error messages guide users to fix permission/configuration issues

### Filtering System

The filtering system provides selective replication - users can choose specific databases and tables to replicate instead of migrating everything. This is implemented through two complementary approaches: CLI flags and interactive mode.

#### ReplicationFilter (src/filters.rs)

The `ReplicationFilter` struct is the central filtering logic used by all commands:

```rust
pub struct ReplicationFilter {
    include_databases: Option<Vec<String>>,
    exclude_databases: Option<Vec<String>>,
    include_tables: Option<Vec<String>>, // Format: "db.table"
    exclude_tables: Option<Vec<String>>, // Format: "db.table"
}
```

**Constructor Validation:**

The `ReplicationFilter::new()` constructor enforces these rules:
- Database filters are mutually exclusive: cannot use both `--include-databases` and `--exclude-databases`
- Table filters are mutually exclusive: cannot use both `--include-tables` and `--exclude-tables`
- Table names must be in `"database.table"` format (validates with `.contains('.')`)
- Returns `anyhow::Result<Self>` with clear error messages for violations

**Filtering Methods:**

- `should_replicate_database(db_name: &str) -> bool`
  - Returns true if database passes filters
  - Include list: database must be in the list
  - Exclude list: database must NOT be in the list
  - No filters: all databases pass

- `should_replicate_table(db_name: &str, table_name: &str) -> bool`
  - Returns true if table passes filters
  - Constructs full name as `"db_name.table_name"`
  - Include list: full name must be in the list
  - Exclude list: full name must NOT be in the list
  - No filters: all tables pass

- `get_databases_to_replicate(source_conn: &Client) -> Result<Vec<String>>`
  - Queries source for all databases via `migration::schema::list_databases()`
  - Filters using `should_replicate_database()`
  - Returns error if no databases match filters
  - Used by multi-database commands (verify, status, sync, init)

- `get_tables_to_replicate(source_conn: &Client, db_name: &str) -> Result<Vec<String>>`
  - Queries source for all tables in a database via `migration::schema::list_tables()`
  - Filters using `should_replicate_table()`
  - Returns empty vec if no tables match (not an error)
  - Used by commands that need table-level filtering

#### Interactive Mode (src/interactive.rs)

Interactive mode provides a terminal UI for selecting databases and tables, built with the `dialoguer` crate:

**Function Signature:**
```rust
pub async fn select_databases_and_tables(source_url: &str) -> Result<ReplicationFilter>
```

**Workflow:**

1. **Connect to Source** - Connects to the source database URL

2. **Discover Databases** - Queries for all user databases (excludes templates)

3. **Select Databases** - Shows multi-select checklist:
   ```
   Select databases to replicate:
   (Use arrow keys to navigate, Space to select, Enter to confirm)

   > [x] myapp
     [x] analytics
     [ ] staging
     [ ] test
   ```

4. **Select Tables to Exclude** (per database):
   - For each selected database, connect to it and discover tables
   - Show multi-select checklist for tables to EXCLUDE
   - Pressing Enter without selections includes all tables
   - Tables are shown as simple names if in `public` schema, or `schema.table` otherwise
   - Internally stores exclusions as `"database.table"` format

5. **Show Summary and Confirm**:
   ```
   ========================================
   Replication Configuration Summary
   ========================================

   Databases to replicate: 2
     ✓ myapp
     ✓ analytics

   Tables to exclude: 2
     ✗ myapp.logs
     ✗ myapp.cache

   ========================================

   Proceed with this configuration? [Y/n]:
   ```

6. **Build ReplicationFilter** - Converts selections to `ReplicationFilter`:
   - Selected databases → `include_databases`
   - Excluded tables → `exclude_tables`
   - Returns the filter for use by commands

**URL Manipulation:**

The `replace_database_in_url()` helper function modifies a PostgreSQL connection URL to connect to a specific database:
```rust
fn replace_database_in_url(url: &str, new_db_name: &str) -> Result<String>
```
This is critical for multi-database operations - it preserves query parameters (like SSL settings) while changing only the database name.

#### Command Integration

Commands integrate filtering in two ways:

**Commands with Interactive Mode** (validate, init, sync):

```rust
let filter = if interactive {
    // Interactive mode - prompt user to select databases and tables
    interactive::select_databases_and_tables(&source).await?
} else {
    // CLI mode - use provided filter arguments
    ReplicationFilter::new(
        include_databases,
        exclude_databases,
        include_tables,
        exclude_tables,
    )?
};
```

These commands accept both `--interactive` flag and CLI filter flags (`--include-databases`, `--exclude-tables`, etc.). Interactive mode and CLI filters are mutually exclusive in practice.

**Commands with CLI-Only Filtering** (status, verify):

```rust
let filter = ReplicationFilter::new(
    include_databases,
    exclude_databases,
    include_tables,
    exclude_tables,
)?;
```

These commands don't support `--interactive` because they operate on existing replication setups and don't perform discovery.

#### Multi-Database Replication Pattern

Commands that support multiple databases (init, sync, status, verify) follow this pattern:

1. **Discover Databases:**
   ```rust
   let databases = filter.get_databases_to_replicate(&source_conn).await?;
   ```

2. **Loop Through Each Database:**
   ```rust
   for db_name in databases {
       // Build database-specific connection URLs
       let source_db_url = replace_database_in_url(&source_url, &db_name)?;
       let target_db_url = replace_database_in_url(&target_url, &db_name)?;

       // Connect to specific database
       let source_db_conn = connect(&source_db_url).await?;
       let target_db_conn = connect(&target_db_url).await?;

       // Perform operation on this database
       // (validation, sync setup, status check, verification, etc.)
   }
   ```

3. **Report Overall Results:**
   Commands typically report per-database results and an overall summary.

**Table-Level Filtering in Operations:**

For operations that need table-level filtering (like verify and sync):

```rust
// Get tables to replicate for this database
let tables = filter.get_tables_to_replicate(&source_db_conn, &db_name).await?;

// Or check individual tables
if filter.should_replicate_table(&db_name, &table_name) {
    // Process this table
}
```

This architecture ensures consistent filtering behavior across all commands while allowing each command to implement its specific operation logic.
