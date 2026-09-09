# Contributing to chilon_rs

Thank you for your interest in contributing!

## Development Setup

1. **Clone and build:**

   ```bash
   git clone https://github.com/andrefs/chilon_rs.git
   cd chilon_rs
   cargo build --workspace
   ```

2. **Run the full validation gate (required before any PR):**

   ```bash
   cargo fmt --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace --all-targets
   cargo build --release
   ```

## Code Style

- Rust 2021 edition
- `rustfmt` enforced (run `cargo fmt --check`)
- `clippy` with `-D warnings` — zero warnings tolerated
- All tests must pass

## Commit Conventions

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — new feature
- `fix:` — bug fix
- `refactor:` — code restructuring without behavior change
- `test:` — adding or modifying tests
- `docs:` — documentation changes
- `chore:` — maintenance, tooling, dependencies

Example: `feat(cli): add --output-format flag`

## Pull Request Process

1. Fork the repo and create a feature branch from `dev`.
2. Make your changes with clear, focused commits.
3. Run the full validation gate locally (see above).
4. Open a PR against `dev` with a clear description of the change and rationale.
5. Ensure CI passes (Rust workflow runs `build`, `clippy`, `test`).

## Reporting Issues

- Use the GitHub issue tracker.
- Include Rust version (`rustc --version`), OS, and steps to reproduce.
- For performance issues, include input size and timing data if possible.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).