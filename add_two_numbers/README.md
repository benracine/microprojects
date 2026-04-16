# add-two-numbers — Rust basics demo

Small crate you can walk through with newcomers: one library function, one binary, tests, and docs.

## Project layout

- **`src/lib.rs`** — Library code (crate `add_two_numbers`, from package `add-two-numbers`). This is where **public API** and **doctests** live. Rust only runs doctests on library targets, not on `main.rs` alone.
- **`src/main.rs`** — Binary entry point. Imports the library (`add_two_numbers::…`) and prints a result. Keeps “app” separate from reusable logic.
- **`Cargo.toml`** — Package name, edition, dependencies. Hyphens in the package name become underscores in Rust (`add-two-numbers` → `add_two_numbers` in code and doctests).

## Run the program

- **`cargo run`** — Build and run the default binary.
- **`cargo run --release`** — Optimized build (slower compile, faster run).

## Tests

- **`cargo test`** — Runs **unit tests** (`#[cfg(test)]` modules), **integration tests** in `tests/` if any, and **doctests** from `lib.rs` doc comments.
- **`cargo test -- --nocapture`** — Show `println!` output from tests (by default test output can be captured).
- **`cargo test add_two_numbers`** — Filter tests whose names match a substring.
- Show **unit tests** in `src/lib.rs`: ordinary `#[test]` functions next to the code they exercise.

## Doctests

- Doc comments on items use `///`; **executable examples** use fenced code blocks in those comments.
- **`cargo test`** compiles and runs those examples as tests (failures = doc or API drift).
- Prefer **`add_two_numbers::function_name`** in examples so they match how external callers use the crate.
- **`cargo test --doc`** — Only run doctests (skips unit/integration tests).

## Documentation (`rustdoc`)

- **`cargo doc`** — Generate HTML API docs for dependencies + this crate (output under `target/doc/`).
- **`cargo doc --no-deps`** — Only document this workspace, not dependencies (faster, good for demos).
- **`cargo doc --open`** — Build and open the docs in a browser.

## Faster / nicer test runs: `cargo nextest`

[`cargo-nextest`](https://nexte.st/) is a popular alternative test runner (parallel, clearer output, JUnit reports, etc.). It is **not** part of the default Rust install.

- **Install** (pick one): `cargo install cargo-nextest --locked`, or see [nexte.st installation](https://nexte.st/docs/installation/).
- **`cargo nextest run`** — Run tests (similar role to `cargo test` for unit/integration tests; **doctest support** has evolved—check current docs if you rely on doctests in CI).
- Good **talking point**: compare `cargo test` vs `nextest` output and speed on a larger project.

## Other one-liners worth mentioning

- **`cargo check`** — Typecheck without producing a full binary (fast feedback while editing).
- **`cargo clippy`** — Lints (style, correctness, performance hints).
- **`cargo fmt`** — Format code with `rustfmt` (team-wide consistent style).

## Demo script idea (bullets for you)

1. Open `src/lib.rs` — point at `///` docs and the ` ``` ` examples (doctests).
2. Open `src/main.rs` — show `add_two_numbers::add_two_numbers` vs duplicate logic in `main` (why `lib` exists).
3. Run **`cargo test`** — unit tests + doctests pass together.
4. Run **`cargo doc --no-deps --open`** — show the rendered `add_two_numbers` page.
5. Optionally run **`cargo nextest run`** if installed — contrast with `cargo test`.

## Add some notes about cargo watch
