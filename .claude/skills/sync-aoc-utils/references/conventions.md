# Porting conventions

How each language expresses the same util. Most utils started out in Rust, so this is written as "Rust shape → the others". It works in any direction.

## Where things live

| | Rust | Go | TypeScript | Python |
| --- | --- | --- | --- | --- |
| Module | `rust/src/utils/<module>.rs`, plus `pub mod <module>;` in `utils/mod.rs` | `go/utils/<module>.go` (package `utils`, imported as `advent/utils`) | `typescript/src/utils/<module>.ts` | `py/src/py/utils/<module>.py` |
| Tests | `#[cfg(test)] mod tests` at the bottom of the module | `go/utils/grid_test.go` for grid, `utils_test.go` for the small modules, `<module>_test.go` for a big new one | `grid.test.ts`, `utils.test.ts` or `<module>.test.ts` next to the module | `py/tests/test_grid.py`, `test_utils.py` or `test_<module>.py` |
| Check (run from that language's dir) | `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --message-format=short 2>&1 \| grep '^src/utils'`, which should print nothing | `gofmt -l utils`, `go vet ./...`, `go test ./...` | `pnpm test` (runs `tsc`, then every `dist/**/*.test.js`), `npx prettier --check <files you touched>` | `uv run python -m unittest discover -s tests` |

- **Fresh worktree:** run `pnpm i` in `typescript/` first. `uv` creates Python's venv by itself.
- **Python version:** the project targets 3.14 through uv, and the system `python3` may be older. Run Python code through `uv run`. The inventory script is the exception, since it works on any Python 3.8+.
- **Unused utils:** Rust's `main.rs` allows `dead_code`, so a util that no solution uses yet is fine.
- **Existing lint debt:** `typescript/src/utils/graph.ts` already fails prettier, so check only the files you touched. Clippy reports errors in `src/years/y2024`, which is why the command above keeps only `src/utils` (and `--all-targets` includes the tests).
- **Stale compiled tests:** `tsc` never deletes old output, and `pnpm test` runs every `dist/**/*.test.js`. After you delete or rename a `.test.ts`, delete its compiled copy in `dist/` too.

## Running a solution

Do this when a fix in a util could change a solution's answer. Run the solution before and after the fix, and compare.

- **Command:** `aoc run <rust|go|ts|py> -y <year> -d <day> -i <input>`. `aoc` is the repo's CLI, installed from `cli/`.
- **Finding inputs:** inputs are gitignored and live in the main checkout's `inputs/<year>/<day>.txt`. The main checkout is the first entry of `git worktree list`. A worktree has no inputs or session token of its own, so always pass `-i` from there.
- **Side effect:** `aoc run` remembers what it ran in that checkout's `.aoc-defaults`, which is gitignored, and later runs default to it. Running from a worktree leaves the user's main checkout alone. Running from the main checkout changes their default, so mention it if you do.
- **Finding callers:** search `rust/src/years`, `go/years`, `typescript/src/years` and `py/src/py/years` for the util's name.

## Names

| Rust | Go | TypeScript | Python |
| --- | --- | --- | --- |
| `snake_case` fn or method | `PascalCase` (exported) | `camelCase` | `snake_case` |
| `CONSTANT` | `Constant` | `CONSTANT` | `CONSTANT` |
| `Type::new(…)` | `NewType(…)` | `new Type(…)` | `Type(…)` |
| `Type::from_x(…)` | `TypeFromX(…)` | `static Type.fromX(…)` | `@classmethod Type.from_x(…)` |
| enum variant `North` | `const North` | `static readonly North` | `NORTH` |

Keep the reference's words, even where another style would choose differently: `get_node` becomes `GetNode`, not `Node`. This lets someone who knows one language find the util in another. A standard library function re-exported as a util takes the util's name: `from itertools import batched as chunks`.

## Shapes

| Rust | Go | TypeScript | Python |
| --- | --- | --- | --- |
| `Option<T>` | `nil` pointer, or `(T, bool)` for values | `T \| undefined` | `T \| None` |
| tuple `(a, b)` | two params or two returns | `[a, b]` tuple type | tuple |
| `impl Iterator<Item = T>` | `iter.Seq[T]` (`iter.Seq2[A, B]` for pairs) | `Generator<T>` (or `IterableIterator<T>` when returning a built-in iterator) | `Iterator[T]` from a generator |
| `.rev()` on a double-ended iterator | separate `…Rev` method | separate `…Rev` method | separate `…_rev` method |
| a struct that only exists to implement `Iterator` | not ported | not ported | not ported |
| enum with methods | `type X int`, `iota` constants, lookup arrays indexed by the constant | class with `static readonly` instances and a private constructor (TS enums can't have methods) | `Enum` whose values carry the data |
| derived `PartialEq`/`Hash` | comparable struct (works as a map key), plus `Equals` when equality is narrower than all fields | `equals()`, plus `key(): string` for use in a `Set` or `Map` | `@dataclass(frozen=True, slots=True)`, or `__eq__`/`__hash__` on the same fields |
| `Rc<RefCell<T>>` | `*T` | object reference | object reference |
| constructors that differ only in ownership | one constructor | one constructor | one constructor |
| `panic!` or `unwrap()` on bad input | `panic` | `throw new RangeError(…)` for bounds, `Error` otherwise | `raise IndexError` or `ValueError` |
| `i64` / `isize` | `int64` / `int` (generic integer math uses `types.Number` from `advent/types`) | `number` (exact to 2^53, so say so if results can be larger) | `int` |
| `char` iteration | runes (`for i := range s` gives each rune's byte offset) | code points (`for (const ch of s)`) | `str` items |

## Behaviour that must match

- **Floats:** compute integer sums first and take a single `sqrt`, as Rust does, so distances are bit-identical.
- **Out of bounds:** lookups outside a grid return the missing value. Python lists accept negative indices, so check bounds explicitly. TS arrays return `undefined` for `-1`.
- **Eager errors:** a util that returns an iterator must reject bad arguments when it's called, not when iteration starts. Otherwise the error appears far from the mistake, or never if nothing iterates. In TypeScript and Python, a generator function's body doesn't run until the first `next()`. So validate in a plain function, then return a generator expression (Python) or call a private generator function (TypeScript). See `string_chunks` and `stringChunks`. Go's `iter.Seq` constructors validate before building the closure.
- **Early exit:** a Go `iter.Seq` must stop as soon as `yield` returns false. Test that with a `break`.
- **Order:** iteration order (for example neighbours clockwise from north) is part of the API. Keep it identical everywhere.

## Language gotchas

- **TypeScript:** a static field or `static {}` block that reads constructor parameter properties fails with TS2729 ("used before its initialization"). Build lookup tables at module level, after the class.
- **TypeScript:** `noUncheckedIndexedAccess` is on, so indexing returns `T | undefined`.
- **TypeScript:** avoid `push(...items)` on lists that might be large, because the spread can overflow the stack.
- **Python:** turning a value back into an `Enum` member with `GridDirection(value)` is slow in hot loops. Precompute dicts for lookups such as opposites and rotations.
- **Python:** 3.14 evaluates annotations lazily, so a class can refer to itself without quotes. Use PEP 695 generics (`class Grid[T]:`).
- **Go:** return pointers into the backing slice (`&g.InnerData[y][x]`) rather than copies, so callers share one node per cell, like Rust's references.

## Shared test cases

Every language checks the same cases, so the expected values are written once and ported verbatim:

- The grid tests share this 10×10 fixture, built through each language's `from_iter` equivalent:

  ```text
  MMMSXXMASM
  MSAMXMSMSA
  AMXSXMAAMM
  MSAMASMSMX
  XMASAMXAMM
  XXAMMXXAMA
  SMSMSASXSS
  SAXAMASAAA
  MAMMMXMMMM
  MXMXAXMASX
  ```

- The tree tests share this tree: `/` has children `a` and `b`, `a` has `c`, and `b` has `d`. DFS gives `/ b d a c` and BFS gives `/ a b c d`. The second grandchild catches a BFS that orders the next level wrongly, which a single grandchild can't.
- Name a ported test after the original in the target's style: `test_grid_plus_walk` becomes `TestGridPlusWalk`, `"grid plus walk"` and `test_plus_walk`.
- Work out the expected values for a new case by walking the fixture, then confirm them by running the case in a language that already has the util. Don't trust a value you computed in your head.
