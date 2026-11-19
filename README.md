# ratatui-json-editor

A small terminal JSON editor built with [Ratatui](https://ratatui.rs). You add key/value pairs in
a TUI, confirm on exit, and the result is printed as JSON.

## Requirements

- Rust 1.85 or newer (the crate uses edition 2024)

## Running

```sh
cargo run
```

The interface is drawn to stderr, so stdout only ever receives the JSON result. That means you can
redirect the output while still seeing the editor:

```sh
cargo run --quiet > pairs.json
```

The same command works in PowerShell and in a POSIX shell.

## Keys

| Screen       | Key                     | Action                                  |
| ------------ | ----------------------- | --------------------------------------- |
| Main         | `e`                     | Open the editor for a new pair          |
| Main         | `q`, `Ctrl+C`           | Ask whether to print JSON and quit      |
| Editing      | `Tab`                   | Switch between the key and value fields |
| Editing      | `Enter`                 | Move from key to value, save on value   |
| Editing      | `Esc`, `Ctrl+C`         | Discard the pair and return to main     |
| Editing      | `Backspace`             | Delete the last character               |
| Exit prompt  | `y`                     | Print the JSON and quit                 |
| Exit prompt  | `n`, `Esc`              | Return to the editor                    |

## Output

```json
{
  "name": "ratatui",
  "version": "0.30.2"
}
```

Keys are sorted and values are always strings, so the output is the same on every run.

## Development

```sh
cargo build          # compile
cargo test           # state machine unit tests
cargo fmt --check    # formatting
cargo clippy         # lints
```

## Project layout

- `src/main.rs` sets up the terminal, restores it on exit or panic, and maps key presses to app
  actions.
- `src/app.rs` holds the application state: the pairs, the input buffers, and the current screen.
- `src/ui.rs` renders the header, pair list, footer, and the editing and exit popups.

## Limitations

- Values are strings only; there is no support for nested objects or arrays.
- Pairs cannot be edited or deleted once saved. Re-entering the same key overwrites its value.
- The quit prompt is the only way to exit through the UI.
