# Productive PEG

## Getting started

### Usage

Ensure [Rust](https://rust-lang.org/learn/get-started/) is installed.

```sh
# 1. clone the repository
git clone https://gitlab.cim.rhul.ac.uk/zlac272/productive-peg.git

# 2. list the executables
cargo run --bin

# 3. fmt & clippy
cargo fmt && cargo clippy

# 4. test
cargo test

# 5. create rustdoc
cargo doc
```

### Hooks

To ensure that all hooks are working please run the commands.

```sh
git config core.hooksPath .githooks
chmod +x .githooks/commit-msg
chmod +x .githooks/pre-commit
chmod +x .githooks/pre-push
```
