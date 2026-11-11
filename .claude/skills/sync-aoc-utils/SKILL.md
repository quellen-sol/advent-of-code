---
name: sync-aoc-utils
description: Syncs this Advent of Code repo's util libraries across Rust, Go, TypeScript and Python. It ports every util that exists in one language to the others, with tests, and keeps the cross-language API map current. Use it whenever a util is added, changed, renamed or removed in rust/src/utils, go/utils, typescript/src/utils or py/src/py/utils. Also use it when someone asks to port, mirror or sync helpers between languages, wants a helper from one language in another, or asks which utils are missing where, even if they never say "sync".
argument-hint: "[check] [module ...]"
---

# Sync AoC utils

Solutions in this repo can be written in Rust, Go, TypeScript or Python, and each language has its own utils library: grids, points, math, trees and so on. The libraries should offer the same toolbox, so that switching language never means rewriting a helper first. Three files keep them in step:

- `references/api-map.md` is the contract. It has one row per public symbol, names that symbol in every language, and records deliberate differences. Read its header for what each kind of cell means.
- `scripts/inventory.py` compares the map with the code and reports where they disagree.
- `references/conventions.md` covers how each language expresses the same API (iterators, missing values, enums, equality), where modules and tests live, and the commands that check them. Read it before writing a port.

Paths below are relative to this skill's directory, except shell commands, which run from the repo root.

## Arguments

- **No arguments:** sync every module.
- **`check`:** report only. Run the inventory, summarize what's out of sync and what you would do about it, and change nothing.
- **Module names** (for example `grid point`): limit the sync to those modules, and pass the names to the inventory too.

## 1. Take inventory

```sh
python3 .claude/skills/sync-aoc-utils/scripts/inventory.py [module ...]
```

It prints three lists and exits 0 only when all of them are empty:

- **Unmapped:** public symbols in the code that the map doesn't mention, meaning new or renamed utils.
- **Stale:** names in the map that the code no longer has, meaning renamed or deleted utils.
- **To port:** map rows with an empty cell, meaning a util that some language doesn't have yet. Each row's Notes are shown alongside.

It also prints a fourth list, **Waiting**: rows marked `wip` because their reference is unfinished. These don't affect the exit status, but check each one on every sync. Once the reference is finished, clear its `wip` cells, which turns them into "to port".

The script finds public symbols with line-based rules that rely on each language's formatter. If a result looks off, such as a member that's missing or a private helper that's listed, check the source and fix the rule in the script rather than working around it. A wrong inventory makes every later sync wrong too.

In `check` mode, stop here and summarize by module.

## 2. Bring the map up to date

Read the source of every unmapped symbol and decide what it is:

- **A new public util:** add a row to its module's table. Fill in that language's cell and leave the other cells empty, so that they show up as "to port". If the module is new, add a `` ## `module` `` section for it.
- **A rename:** a stale name in the same language and module that does the same job means the util was renamed. Update the cell. If the new name is better, ask the user whether the other languages should follow, since a rename also touches the solutions that call it.
- **Plumbing:** something that exists only because of the language, such as a Rust iterator struct, trait glue or an ownership wrapper. If the module already has a row for that kind of plumbing, add the name to it. Otherwise add a row with `—` in the other cells and say why in Notes.

A stale name with no matching rename means the util was deleted from that language. Don't delete it from the others on your own, because solutions in those languages may still use it. Ask the user.

## 3. Decide how to fill each empty cell

Read the util in every language that already has it, then pick one of these for each empty cell:

- **The source is unfinished.** It has `todo!()`, empty bodies, `throw new Error("Not implemented")` or a WIP note. Write `wip` in its empty cells and put a line under the module heading saying what's unfinished, then mention it in the report. Porting an API that's still moving multiplies the churn.
- **The target's standard library already does it.** Don't reimplement it.
  - If it's a function that can be re-exported without a wrapper (Python's `from math import gcd`, Rust's `pub use`, TypeScript's `export { … } from`), re-export it from the matching utils module, so the import path looks the same in every language.
  - Re-export it under the util's name even when the standard library calls it something else, for example `from itertools import batched as chunks`.
  - Smaller differences, such as yielding tuples instead of lists, are fine. Record them in Notes.
  - If it's a method or a generic that can't be re-exported (Rust's `<[T]>::chunks`, Go's `slices.Chunk`), write `native: …` in the cell instead of wrapping it.
- **The target already has a better implementation**, one that is faster or more general, or that solutions already use. Keep it, map it, and note any difference in behaviour.
- **Anything else:** port it, following conventions.md. Port the behaviour and the shape of the API rather than the syntax. A Rust iterator struct becomes a generator or an `iter.Seq`, and constructors that exist only for ownership collapse into one.

When several languages already have a util, treat the clearest and best-tested version as the reference. If they disagree, one of them has a bug. Step 4 covers that.

## 4. Port behaviour, not bugs

Reading code closely enough to port it is when its bugs show up. The first syncs found several this way:

- Rust's `diagonal_walk` returned orthogonal neighbours on edges.
- Rust's `traverse_row` skipped its first node.
- A rectangle area was wrong in one orientation.
- `GridNode`'s hash disagreed with its equality.
- TypeScript's `chunks` looped forever when given a size of 0.

When the reference does something that looks unintended:

1. Prove it by running the reference on that case, with a temporary test that you remove afterwards. In TypeScript, also delete its compiled copy in `dist/`, or `pnpm test` keeps running it.
2. Port the intended behaviour, and add a test in the target that pins it down. If every target turned out `native:`, there's no port to pin it in, so the reference is the only place a fix can go.
3. Report it, and ask before changing the reference, because existing solutions may depend on the current behaviour. If the user agrees, fix it there with the same test. Then rerun every solution that uses the util, before and after the fix, to see whether its answer changes. conventions.md says how to run a solution from a worktree.

## 5. Port the tests

Every port gets its reference's tests, case for case, with the same fixtures and the same expected values. That way all four languages are checked against one set of expectations. The grid and tree tests have shared fixtures, which are in conventions.md. Reuse a fixture where it fits, and add a new one there when a module needs its own.

Add cases for the edges you had to think about: bounds, negative numbers, empty input, multi-byte characters, invalid arguments and stopping an iterator early.

The goal is that every language ends up with the same cases. So if the reference has no tests for the util, write them there as well. If you add a case that the reference lacks, add it there too, provided it passes.

## 6. Update the map

- Fill in every cell you ported.
- Record each `—` and `native:` decision.
- Put deliberate differences in behaviour in Notes, so that the next sync doesn't undo them.

## 7. Verify

Run every language's checks, using the commands in conventions.md, not only the languages you touched, because a shared fixture or a map change can affect all of them. Then rerun the inventory. It should exit 0, and anything you skipped should appear under Waiting.

## 8. Report

Tell the user:

- What was ported, by module and language.
- Where a native or existing implementation was kept instead, and why.
- Differences in behaviour that you recorded in the map.
- Bugs you found, how you confirmed each one, and whether the reference was changed.
- What you skipped (rows marked `wip`, deletions waiting for a decision) and the inventory's final output.

## Large syncs

When many utils need porting, you can split the work by target language once steps 1 to 3 are done. Give one subagent per language its rows from your plan, the reference source files, conventions.md and the test cases to port. Do steps 1 to 3 and 6 to 8 yourself. The map and the shared test cases are what keep the languages consistent, so one agent should own them.
