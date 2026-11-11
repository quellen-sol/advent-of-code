# Porting conventions

How each language expresses the same util. Most utils started out in Rust, so this is written as "Rust shape → the others". It works in any direction.

## Where things live

| | Rust | Go | TypeScript | Python |
| --- | --- | --- | --- | --- |
| Module | `rust/src/utils/<module>.rs`, plus `pub mod <module>;` in `utils/mod.rs` | `go/utils/<module>.go` (package `utils`, imported as `advent/utils`) | `typescript/src/utils/<module>.ts` | `py/src/py/utils/<module>.py` |
| Tests | `#[cfg(test)] mod tests` at the bottom of the module | `go/utils/grid_test.go` for grid, `utils_test.go` for the small modules, `<module>_test.go` for a big new one | `grid.test.ts`, `utils.test.ts` or `<module>.test.ts` next to the module | `py/tests/test_grid.py`, `test_utils.py` or `test_<module>.py` |
| Check (run from that language's dir) | `cargo fmt --check`, `cargo clippy`, `cargo test` | `gofmt -l utils`, `go vet ./...`, `go test ./...` | `pnpm test` (runs `tsc`, then every `dist/**/*.test.js`), `npx prettier --check <files you touched>` | `uv run python -m unittest discover -s tests` |

- **Fresh worktree:** run `pnpm i` in `typescript/` first. `uv` creates Python's venv by itself.
- **Python version:** the project targets 3.14 through uv, and the system `python3` may be older. Run Python code through `uv run`. The inventory script is the exception, since it works on any Python 3.8+.
- **Unused utils:** Rust's `main.rs` allows `dead_code`, so a util that no solution uses yet is fine.
- **Existing formatting debt:** `typescript/src/utils/graph.ts` already fails prettier. Check only the files you touched.

## Names

| Rust | Go | TypeScript | Python |
| --- | --- | --- | --- |
| `snake_case` fn or method | `PascalCase` (exported) | `camelCase` | `snake_case` |
| `CONSTANT` | `Constant` | `CONSTANT` | `CONSTANT` |
| `Type::new(…)` | `NewType(…)` | `new Type(…)` | `Type(…)` |
| `Type::from_x(…)` | `TypeFromX(…)` | `static Type.fromX(…)` | `@classmethod Type.from_x(…)` |
| enum variant `North` | `const North` | `static readonly North` | `NORTH` |

Keep the Rust name's words, even where another style would choose differently (`get_node` → `GetNode`, not `Node`). This lets someone who knows one language find the util in another.

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
- **Eager errors:** Go and TS can validate before returning the iterator. A Python generator raises only when first advanced, so put the check in a plain function that returns the generator.
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

- Name a ported test after the original in the target's style: `test_grid_plus_walk` becomes `TestGridPlusWalk`, `"grid plus walk"` and `test_plus_walk`.
- Work out the expected values for a new case by walking the fixture, then confirm them by running the case in a language that already has the util. Don't trust a value you computed in your head.
