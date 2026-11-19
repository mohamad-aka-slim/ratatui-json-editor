# AGENTS.md

Single-binary Ratatui TUI for entering JSON key/value pairs, then printing the result as
pretty-printed JSON. No README or CI — `src/` is the only source of truth.

## Commands

- `cargo run` — starts the TUI. Needs an interactive terminal; it also works in background
  smoke runs that only check startup.
- `cargo build` / `cargo clippy` / `cargo fmt --check` — all clean.
- `cargo test` — 3 unit tests covering `App` state transitions; no integration tests.

## Layout and flow

- `src/main.rs` — terminal lifecycle (`init_terminal`, `restore_terminal`, `install_panic_hook`),
  the `run_app` event loop, and `handle_key` which maps key presses to `App` methods.
- `src/app.rs` — `App` state machine: private fields plus methods (`begin_editing`,
  `submit_editing`, `save_pair`, `cancel_editing`, `push_char`, ...). UI reads state through
  getters. Unit tests live at the bottom.
- `src/ui.rs` — stateless rendering; color constants at the top, one `render_*` helper per region,
  and `centered_rect` for popups.

## Keys

- `Main`: `e` new pair, `q` or `Ctrl+C` quit prompt.
- `Editing`: typing/`Backspace` edit the focused field, `Tab` switches key↔value, `Enter` advances
  from key to value, `Enter` on value saves (empty keys are rejected), `Esc`/`Ctrl+C` cancels.
- `Exiting`: `y` prints JSON and exits, `n`/`Esc` returns to `Main`.

## Gotchas

- The TUI draws to **stderr**; the final JSON is printed with `println!` to **stdout**. This split
  keeps JSON pipeable — never render the UI to stdout.
- crossterm is used through `ratatui::crossterm` (ratatui 0.30.2 re-exports crossterm 0.29).
  Do not add a separate `crossterm` dependency.
- `run_app` returns `io::Result<bool>` where `true` means "print JSON after terminal restore";
  only the `y` confirm path returns `true`.
- JSON is serialized from a `BTreeMap`, so key order is sorted and stable; changing it to
  `HashMap` makes output order random.
- Edition 2024 requires Rust 1.85+ (toolchain here is 1.99).
- Running the built binary holds a lock on `target/debug/ratatui-json-editor.exe`; stop it before
  rebuilding if `cargo` reports "Access is denied".
- All of `src/` is uncommitted work relative to HEAD (HEAD has only empty stubs); do not reset it.
