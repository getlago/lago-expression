# Lago Expression

A cross-platform expression parser and evaluator for Lago, supporting multiple programming languages and platforms.

## Development

### Prerequisites

- Rust (latest stable)
- Ruby (for Ruby bindings)
- Node.js (for JavaScript bindings)
- Go (for Go bindings)

### Expression Core

This is the core expression parser and evaluator. It is written in Rust and is used by the other bindings.

```bash
# Build all components
cargo build -p expression-core
```

#### Testing

```bash
cargo test -p expression-core
```

### Expression Ruby

This is the Ruby extension for Lago Expression.

See [expression-ruby/README.md](expression-ruby/README.md) for more information.

### Expression JS

See [expression-js/README.md](expression-js/README.md) for more information.
