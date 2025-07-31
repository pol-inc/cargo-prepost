# cargo-prepost

## Installation

```bash
git clone https://github.com/pol-inc/cargo-prepost
cd cargo-prepost
cargo install --path .
```

## Usage

Add a line into your shell config file such as `.bashrc`.

```bash
export PATH="$(cargo prepost setup)"
```

Now, you can use proxied cargo.

```bash
cargo run
```

This command executes `prepost/prerun.rs`, `cargo run`, and `prepost/postrun.rs`.

You can see example projects in `examples`.
