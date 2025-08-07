# cargo-prepost

**cargo-prepost** is a wrapper tool that allows you to automatically run custom scripts or binaries before and after any cargo command. This makes it easy to automate and customize your Rust development workflow by hooking into build, test, or run steps.

## Installation

```bash
git clone https://github.com/pol-inc/cargo-prepost
cd cargo-prepost
cargo install --path .
```

## Setup

Add the following line to your shell configuration file (e.g., `.bashrc` or `.zshrc`):

```bash
export PATH="$(cargo prepost setup)"
```

This ensures that all `cargo` commands are automatically proxied through cargo-prepost.

## Usage

Simply use cargo as usual. If there are corresponding pre/post scripts, they will be executed automatically.

For example:

```bash
cargo run
```

When you run the above command, the following steps occur:

1. `prepost/prerun` (or `prepost/prerun.rs`) is executed
2. The original `cargo run` command is executed
3. `prepost/postrun` (or `prepost/postrun.rs`) is executed

If a Rust file such as `prepost/prerun.rs` exists, it will be automatically compiled before execution.

## Examples

See the [`examples`](./examples) directory for sample projects and pre/post scripts. You can customize these scripts to fit your own project's needs.
