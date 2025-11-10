# Repository Guidelines

## Project Structure & Module Organization
`src/main.rs` boots the CLI and routes subcommands into `src/commands`. Connection plumbing lives in `src/postgres`, snapshot flows in `src/migration`, and logical replication helpers in `src/replication`. Terminal UX resides in `src/interactive.rs`, replication filters sit in `src/filters.rs`, while integration coverage belongs in `tests/integration_test.rs`. Keep design notes in `docs/` and generated artifacts in `target/` or outside the repo when sensitive.

## Build, Test, and Development Commands
- `cargo fmt` — enforce the canonical Rust style before every review.
- `cargo clippy --all-targets --all-features` — lint for footguns across binaries, libs, and tests.
- `cargo test` — run fast unit and smoke coverage.
- `cargo test --test integration_test -- --ignored` — execute the destructive replication suite once test databases are ready.
- `cargo run -- validate --source $SRC --target $TGT --yes` — dry-run validation with disposable Neon endpoints; set `TEST_SOURCE_URL` / `TEST_TARGET_URL` for real instances.

## Coding Style & Naming Conventions
Work on Rust 1.70+ with 4-space indents. Prefer descriptive `snake_case` functions/modules, `UpperCamelCase` types, and `SCREAMING_SNAKE_CASE` constants. Document public APIs with `///` doc comments, split modules once they near 400 lines, and rely on `cargo fmt` plus `clippy` to settle disagreements quickly.

## Testing Guidelines
Practice TDD: introduce a failing test, watch it fail, then implement the fix. Co-locate unit tests beside the module under `#[cfg(test)]` and reserve the ignored integration suite for full replication sweeps. Assert on measurable outcomes (lag thresholds, row counts) rather than log strings, and never delete a failing test—surface it to Taariq if it blocks progress.

## Commit & Pull Request Guidelines
Use imperative commit subjects under 72 characters (e.g., “Tighten replication filter logging”) and wrap message bodies near 100 columns with `Closes #123` style footers when applicable. PRs must describe the scenario, enumerate automated and manual tests (`cargo test`, ignored integration runs, CLI smoke), and attach screenshots for UX tweaks. Request review from someone familiar with the impacted subsystem before merging.

## Agent Collaboration Notes
Rule #1 in CLAUDE.md still applies: doing it right beats doing it fast. Push back on ambiguous directives, document any exceptions taken, and pause immediately if instructions conflict. Prefer the smallest safe change, avoid mocks in end-to-end flows, and keep secrets out of the repo; coordinate with Taariq before touching production data.
