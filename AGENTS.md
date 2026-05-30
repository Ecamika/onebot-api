# AGENTS.md

## Project overview
- Rust **library** implementing the OneBot V11 protocol. Not a workspace.
- Main crate `onebot-api` (version 1.2.3) with a separate proc-macro crate `onebot-api-macros` in `macros/`.
- Rust **edition 2024** — requires toolchain >= 1.85.
- `examples/communication/` contains a usage example.
- `changelogs/` contains daily change records.

## Build / check / test
```bash
cargo check          # fast compile check
cargo build          # full build (default features = "full")
cargo clippy         # lint
cargo fmt            # format (uses rustfmt.toml)
cargo test           # run all unit tests
```

No feature implies nothing compiles; `default = ["full"]` enables all protocol modules.
To build/test a single feature set:
```bash
cargo check --no-default-features --features "websocket,http"
```

## Formatting
- `rustfmt.toml` configures **hard tabs** with `tab_spaces = 2` and `edition = "2024"`.
- Run `cargo fmt` before committing. CI does not enforce formatting, but the convention is strong.

## Architecture
- `Client` (in `src/communication/utils.rs`) is the single entrypoint. It uses:
  - `flume` (mpsc) for API request channels (`InternalAPISender` / `InternalAPIReceiver`)
  - `tokio::sync::oneshot` + `Arc<Mutex<BTreeMap<String, _>>>` for API **response** routing — echo-keyed registry, not broadcast
  - `tokio::broadcast` (mpmc) for event dispatch (`PublicEventSender`)
  - Dependency injection via the `CommunicationService` trait for protocol implementations
- Protocol modules under `src/communication/` are all **feature-gated**:
  - `websocket` — single hand-written `WsTransfer` Future state machine (`ws/ws_transfer.rs`), replaces the old split-sink/stream dual-task model
  - `websocket-reverse`, `http`, `http-post`, `sse`
- Optional modules also feature-gated: `combiner`, `quick_operation`, `selector`
- `src/main.rs` is **gitignored** — exists only for local dev testing, not shipped.

## Release
- Triggered automatically on push to `master` via `.github/workflows/publish-release.yml`.
- Uses `deno task publish-release` (Deno script at `scripts/publish-release.ts`) to create a GitHub release and run `cargo publish`.
- Requires secrets: `GITHUB_TOKEN`, `CARGO_PUBLISH`.

## Key conventions
- Client is designed to be wrapped in `Arc<Client>` — no `Mutex` needed (channels handle synchronization).
- The `text!` macro (in `src/message/utils.rs`) creates a single-text-segment message; it delegates to `format!` internally.
- Error types: `APIRequestError` for API failures, `ServiceStartError` for connection setup failures. Both are in `src/error.rs`.
- `Selector` (feature-gated behind `selector`) provides chainable event filtering with sync and async variants of each method.

## Changelog rule
**Whenever any modification is made to the project (any file, any folder, including `src/`, `macros/`, `examples/`, `scripts/`, `.github/`, `AGENTS.md`, etc.), the change MUST be recorded in the `changelogs/` folder.**

Procedure:
1. Look for a markdown file named with the **current date** (format `YYYY-MM-DD.md`) in `changelogs/`.
2. If the file exists, append the current change to it.
3. If the file does not exist, create it and write the change.
4. Each entry should include: what was changed, which files were affected, and a brief reason or impact.

## Macros documentation rule
**Whenever any modification is made to `macros/` (adding, removing, or changing proc macros and their behavior), the documentation in `macros/docs/` MUST be kept in sync.**

- Adding a new proc macro: create a corresponding markdown file under `macros/docs/` named after the macro (e.g. `my_macro.md` for `#[my_macro]`), documenting its functionality, usage, and attributes.
- Removing a proc macro: delete the corresponding markdown file from `macros/docs/`.
- Changing macro behavior or attributes: update the existing markdown file to reflect the new behavior.
