# rhonetyping

Terminal typing speed trainer. Pick a language mode, type the snippet, see your WPM and accuracy.

Built with [ratatui](https://github.com/ratatui-org/ratatui) and crossterm.

## Build

```
cargo build --release
```

Requires Rust 2024 edition (rustup stable >= 1.85).

## Run

```
cargo run --release
```

Or after building:

```
./target/release/rhonetyping
```

## Controls

| Screen  | Key              | Action           |
|---------|------------------|------------------|
| Menu    | ↑ / k, ↓ / j    | navigate modes   |
| Menu    | Enter / Space    | start session    |
| Menu    | q                | quit             |
| Typing  | (type)           | advance cursor   |
| Typing  | Backspace        | delete last char |
| Typing  | Esc              | back to menu     |
| Results | Enter / r        | retry snippet    |
| Results | n                | new snippet      |
| Results | Esc / q          | back to menu     |
| Any     | Ctrl-C           | quit             |

## Modes

- **English** — short prose sentences
- **Rust** — real Rust function snippets
- **Python** — real Python function snippets
