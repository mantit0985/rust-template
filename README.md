# [Project Name] - Service Starter Guide

![CI](https://github.com/mantit0985/rust-template/actions/workflows/ci.yml/badge.svg) ![CD](https://github.com/mantit0985/rust-template/actions/workflows/cd.yml/badge.svg) ![Rust](https://img.shields.io/badge/rust-stable-blue)

This guide provides the essential steps to get your service up and running and explains the project's structural organization.

## Compliance & Standards

- **Styling**: Enforces the [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/) via `rustfmt`.
- **Security**: Verified against [CWE](https://cwe.mitre.org/) via Semgrep SAST and [RustSec](https://rustsec.org/) via `cargo-audit`.
- **Quality**: Adheres to the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) via `clippy`.

## Getting Started

### Prerequisites

- Rust 1.99+
- Docker
- PostgreSQL
- **Portability Note**: If targeting Alpine Linux or any musl-based distribution, you must use a musl target (e.g., `x86_64-unknown-linux-musl`) to avoid linkage errors.

### Installation & Development

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd rust-template
   ```

2. Configure environment variables in `.env`.

3. Run the service:

   ```bash
   cargo run
   ```

## Project Structure

This project uses a feature-based directory structure to ensure a clean separation of concerns.

### Core Directories

- `src/core/`: Contains cross-cutting concerns such as configuration, database connectivity, error handling, and telemetry.
- `src/features/`: Contains domain-specific logic organized by feature.

## Release process

Releases are made by pushing an annotated GPG-signed tag (`git tag -a vX.Y.Z -m "release vX.Y.Z" && git push origin vX.Y.Z`). A pre-push hook (release-gate.sh) validates the tag at push time:

1. tag name matches the VERSION file at that commit
2. tagged commit is origin/main tip
3. latest CI run for that commit is green
4. tag signature verifies

On success the hub release workflow publishes the GitHub Release with SHA256SUMS and its detached signature. Enable the hook in your clone with:
`git config core.hooksPath <hub-repo-clone>/.github/hooks`
