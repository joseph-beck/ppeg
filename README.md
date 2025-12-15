# Productive PEG

## Getting started

### PPEG Library Usage

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

### PPEG UI & API Usage

```sh
# 1. clone the repository
git clone https://gitlab.cim.rhul.ac.uk/zlac272/productive-peg.git

# 2. run api
cargo run --bin ppeg-api
# or run
make api

# 3. install ui dependencies
cd ppeg-ui
pnpm i

# 4. run ui
pnpm run dev
# or run in at project root
make ui
```

### Hooks

To ensure that all hooks are working please run the commands.

```sh
git config core.hooksPath .githooks
chmod +x .githooks/commit-msg
chmod +x .githooks/pre-commit
chmod +x .githooks/pre-push
```
