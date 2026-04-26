# aoc22

Advent of Code 2022 solutions in Rust. Days 1–21 implemented.

## Layout

- `src/days/dayN.rs` — solutions, each exposing `part1(input: &str) -> Result<String>` and `part2(...)`. Days 15, 16, and 17 take an extra numeric argument (target row / minutes / rock count).
- `src/utils/file/` — input parsers, one per puzzle category (`groups`, `signals`, `valves`, `walls`, `sensors`, `monkeys`, …). Re-exported via `utils::file`.
- `src/utils/vec.rs` — small generic helpers (matrix transforms, distinct-substring search).
- `src/main.rs` — thin CLI dispatching `(day, part)` to the matching `partN`.
- `src/lib.rs` — library entry point so the binary and tests share the same modules.
- `tests/integration.rs` — runs every day against `input/dayNN/test.txt` in-process.

## Build & run

```
$ cargo build --release
$ ./target/release/aoc22 <day> <part> <path> [extra]
```

`<day>` is `1`–`21`, `<part>` is `1` or `2`. `[extra]` overrides the default for the puzzles that need it:

| day | part | extra |
| --- | ---- | ----- |
| 15  | 1    | target row (default `2_000_000`) |
| 15  | 2    | search bound (default `4_000_000`) |
| 16  | 1    | minutes (default `30`) |
| 16  | 2    | minutes (default `26`) |
| 17  | 1    | rock count (default `2022`) |
| 17  | 2    | rock count (default `1_000_000_000_000`) |

Example:

```
$ ./target/release/aoc22 1 1 ./input/day01/test.txt
[INFO] The maximum calorie count is 24000
```

## Tests

```
$ cargo test --release
```

Runs unit tests (parser correctness, malformed-input errors, `Signal` ordering) plus 21 integration tests that exercise every `partN` against its sample input.

## Toolchain

`rust-toolchain.toml` pins the channel (currently `1.95.0`) with `rustfmt` and `clippy`. Formatting and lint policy:

```
$ cargo fmt
$ cargo clippy --all-targets -- -D warnings
```
