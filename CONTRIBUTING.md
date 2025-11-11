# Contributing

When contributing please make sure that you follow the guidelines set out within this document.

## Getting Started

1. Install [Rust](https://rust-lang.org/) if not already installed.
2. Clone the repository with `git clone https://gitlab.cim.rhul.ac.uk/zlac272/productive-peg.git`.
3. Use `cargo run` to see runnable targets.
4. Use `cargo test` to test.

## Commits

All commits should follow the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) guidelines to help ensure consistency. This looks something like `type: description` for example `fix: optional type` or `docs: add docs for cst struct`. Using a type and a short description for the change.

## Branching

When creating a branch please ensure that it follows similar convention to commits, with the it being `<type>/<short-description>` for example `feat/memo-table` or `fix/cst-generation`.

## Pull/Merge Requests

Pull requests follow the convention of commits a bit closer. They should be named such that `type: description` for example `feat: add new rule` or `fix: cst generation`. Before merging pull requests must have passed all pipelines, which consist of formatting checks, builds and testing. Please make sure that all new updates are appropriately unit tested.

## Code Style & Linting

- `cargo fmt` passes without issue.
- `cargo clippy` has no warnings or issues.

## Testing

- Using `cargo test` please ensure the whole test suite passes.

## Issues

Please feel free to open an issue with any bugs or problems found with the library!
