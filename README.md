# ❄️ Advent of Code 2025 Completion ❄️

## Usage

Install the CLI once, and reinstall it after changing anything in `cli/`:

```sh
cargo install --path cli
aoc session set   # paste the `session` cookie from adventofcode.com
```

Then, from anywhere in the repo:

```sh
aoc new [lang] [-y YEAR] [-d DAY]                        # create a solution and download its input
aoc run [lang] [-y YEAR] [-d DAY] [-i FILE]              # build and run it, optionally on a sample file
aoc bench [lang] [-y YEAR] [-d DAY] [-i FILE] [-t SECS]  # measure it over repeated runs
```

`lang` is `rust` (default), `ts`, `go` or `py`. The year and day default to today on AoC's clock (UTC-5), so they roll over the moment a puzzle unlocks. `run` and `bench` remember the last solution instead, language included: `aoc new` sets the default for both, and `aoc run` sets it for `bench`. So after `aoc new py -y 2022 -d 8`, a plain `aoc run` or `aoc bench` uses the Python solution for 2022 day 8, and `-d 9` alone means Python 2022 day 9.

Inputs are stored in `inputs/{year}/{day}.txt`, the session token in `.env` and the remembered solutions in `.aoc-defaults`. All three are gitignored. On a fresh clone, run `pnpm i` in `typescript/` first.

Each language keeps its template at its root, using `__YEAR__` and `__DAY__` placeholders. `rust/src/years/**/mod.rs` and `go/years/registry.go` are generated, so don't edit them by hand.

## Timing

`aoc run` measures everything once. `aoc bench` repeats it for `-t` seconds each (1 by default) and reports the median along with the spread. All four languages follow the same rules and print the same table, so their numbers can be compared:

- **Setup** covers reading the input and constructing the solution, so parsing counts wherever a solution does it. **Total** is setup plus both parts.
- Every run gets a new solution, built outside the timer, so a part can mutate its state without affecting anything else. That also means parts can't pass anything to each other through the instance.
- `aoc bench` discards the first 20% of its time as warm-up, then records up to 100,000 runs. `cold` is the very first run, and a part slower than `-t` gets that run alone, so raise `-t` to sample it properly.
- If an answer changes between runs, `aoc bench` stops with an error instead of reporting timings for it.
- Anything a solution prints repeats on every run and counts towards its time, so remove debug prints before benchmarking.

Each language's harness lives in `rust/src/timing.rs`, `go/adventlib/timing.go`, `typescript/src/timing.ts` and `py/src/py/timing.py`. Change them together, along with the test cases they share.

## Utils

Every language has the same utils, in `rust/src/utils`, `go/utils`, `typescript/src/utils` and `py/src/py/utils`. `.claude/skills/sync-aoc-utils/references/api-map.md` maps each util's name across the four languages. After adding or changing a util, run `/sync-aoc-utils` in Claude Code to port it to the others, or `/sync-aoc-utils check` to see what's out of sync.

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
