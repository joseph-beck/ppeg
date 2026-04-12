# Productive PEG

Productive PEG is a Rust PEG parsing library with support for left recursive grammars.

## Getting Started

To get started with using Productive PEG follow the below guides.

### Productive PEG Usage

```sh
# 1. clone the repository
git clone https://gitlab.cim.rhul.ac.uk/zlac272/productive-peg.git

# 2. install rust https://rust-lang.org/learn/get-started/ here and verify the installation.
rustc --version

# 3. list the executables
cargo run --bin

# 4. fmt & clippy
cargo fmt && cargo clippy

# 5. test
cargo test

# 6. create rustdoc
cargo doc
```

### Creating a New Executable

Requires having cloned the repository with the steps above.

```sh
# 1. create a new executable
cargo new <name>

# 2. create a parser using Productive PEG in your new project, and add the ppeg_core dependency in Cargo.toml
...

[dependencies]
ppeg-core = { path = "path-to/ppeg-core" }

# 3. use the ppeg_core library in your executable.
use ppeg_core::prelude::*;

fn main() {
    ...
}

# 4. run your executable
cargo run --bin <name>
```

### User Interface Usage

```sh
# 1. install wasm-pack
cargo install wasm-pack

# 2. install pnpm, checkout https://pnpm.io/installation for more information
npm i -g pnpm

# 3. cd into ppeg-ui and install dependencies
cd ppeg-ui
pnpm i

# 4. run the application
pnpm run dev

# 5. build the application
pnpm run build
```

### Hooks

To ensure that all hooks are working please run the commands.

```sh
git config core.hooksPath .githooks
chmod +x .githooks/commit-msg
chmod +x .githooks/pre-commit
chmod +x .githooks/pre-push
```

## Meta

Using the meta syntax it is possible to define a grammar such that:

```txt
// Char expression of 'a'
char := { 'a' }
```

To see the full specification please see [SPEC.md](./SPEC.md).

## Contributing

For more information on how to contribute and standards followed within this codebase please see [CONTRIBUTING.md](./CONTRIBUTING.md).
