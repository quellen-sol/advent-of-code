# ❄️ Advent of Code 2025 Completion ❄️

## Usage

Install the CLI once, and reinstall it after changing anything in `cli/`:

```sh
cargo install --path cli
aoc session set   # paste the `session` cookie from adventofcode.com
```

Then, from anywhere in the repo:

```sh
aoc new [lang] [-y YEAR] [-d DAY]            # create a solution and download its input
aoc run [lang] [-y YEAR] [-d DAY] [-i FILE]  # build and run it, optionally on a sample file
```

`lang` is `rust` (default), `ts`, `go` or `py`. The year and day default to today on AoC's clock (UTC-5), so they roll over the moment a puzzle unlocks.

Inputs are stored in `inputs/{year}/{day}.txt` and the session token in `.env`. Both are gitignored. On a fresh clone, run `pnpm i` in `typescript/` first.

Each language keeps its template at its root, using `__YEAR__` and `__DAY__` placeholders. `rust/src/years/**/mod.rs` and `go/years/registry.go` are generated, so don't edit them by hand.

## Progress

| Day | Part One | Part Two |
| :-: | :------: | :------: |
|  1  |    ✅    |    ✅    |
|  2  |    ✅    |    ✅    |
|  3  |    ✅    |    ❌    |
|  4  |    ✅    |    ✅    |
|  5  |    ✅    |    ✅    |
|  6  |    ✅    |    ❌    |
|  7  |    ✅    |    ❌    |
|  9  |    ✅    |    ❌    |
