#!/usr/bin/env python3
"""Compares every language's utils with the API map and lists what's out of sync.

    python3 .claude/skills/sync-aoc-utils/scripts/inventory.py [module ...]

Three kinds of drift are reported:

  unmapped  public symbols in the code that the map doesn't mention (new or renamed utils)
  stale     names in the map that the code no longer has (renamed or removed utils)
  to port   map rows with an empty cell (a util some language doesn't have yet)

Rows whose empty cells say `wip` are waiting for their reference to be finished.
They're listed too, but don't count as drift. The exit status is 1 only when
something is unmapped, stale or left to port, so it doubles as a check.

Symbols are found with line-based rules that rely on each language's formatter
(rustfmt, gofmt, prettier with 4-space indents, and PEP 8 for Python): members
are the lines indented exactly one level inside a type. Language plumbing that
isn't public API (trait impls, private names, tests) is skipped.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

SKILL_DIR = Path(__file__).resolve().parent.parent
REPO = SKILL_DIR.parent.parent.parent
API_MAP = SKILL_DIR / "references" / "api-map.md"

LANGUAGES = {
    "Rust": ("rust/src/utils", "*.rs"),
    "Go": ("go/utils", "*.go"),
    "TypeScript": ("typescript/src/utils", "*.ts"),
    "Python": ("py/src/py/utils", "*.py"),
}

NAME = re.compile(r"[A-Za-z_]\w*(?:(?:::|\.)[A-Za-z_]\w*)*")


def is_source(language: str, path: Path) -> bool:
    name = path.name
    if language == "Rust":
        return name != "mod.rs"
    if language == "Go":
        return not name.endswith("_test.go")
    if language == "TypeScript":
        return not name.endswith(".test.ts") and not name.endswith(".d.ts")
    return name != "__init__.py"


def strip_generics(text: str) -> str:
    """Drops a leading balanced <...>, so `<'a, T> Foo<'a, T>` becomes ` Foo<'a, T>`."""
    if not text.startswith("<"):
        return text
    depth = 0
    for i, char in enumerate(text):
        if char == "<":
            depth += 1
        elif char == ">":
            depth -= 1
            if depth == 0:
                return text[i + 1 :]
    return text


def rust_symbols(source: str) -> list[str]:
    names: list[str] = []
    owner, kind = None, None
    in_test = False
    for line in source.splitlines():
        if line.startswith("#[cfg(test)]"):
            in_test = True
            continue
        if in_test:
            if line.startswith("}"):
                in_test = False
            continue

        if line.startswith("}"):
            owner, kind = None, None
            continue

        if not line.startswith(" "):
            if m := re.match(r"pub (?:const|static|fn) (\w+)", line):
                names.append(m.group(1))
            elif m := re.match(r"pub (struct|enum|trait|type) (\w+)", line):
                names.append(m.group(2))
                if m.group(1) != "type" and line.rstrip().endswith("{"):
                    owner, kind = m.group(2), m.group(1)
            elif line.startswith("impl"):
                header = strip_generics(line[len("impl") :].rstrip(" {"))
                if " for " in header:
                    owner, kind = None, "trait impl"
                elif m := re.match(r"\s*(\w+)", header):
                    owner, kind = m.group(1), "impl"
            continue

        if owner is None or line.startswith("     "):
            continue
        if kind == "impl" and (m := re.match(r"    pub (?:const )?(?:fn )?(\w+)", line)):
            names.append(f"{owner}::{m.group(1)}")
        elif kind == "struct" and (m := re.match(r"    pub (\w+):", line)):
            names.append(f"{owner}.{m.group(1)}")
        elif kind == "enum" and (m := re.match(r"    ([A-Z]\w*)\s*(?:[,({]|$)", line)):
            names.append(f"{owner}::{m.group(1)}")
        elif kind == "trait" and (m := re.match(r"    fn (\w+)", line)):
            names.append(f"{owner}::{m.group(1)}")
    return names


def go_symbols(source: str) -> list[str]:
    names: list[str] = []
    block, owner = None, None
    for line in source.splitlines():
        if line in (")", "}"):
            block, owner = None, None
            continue

        if block == "decl":
            if m := re.match(r"\t(\w+(?:, \w+)*)\b", line):
                names.extend(n for n in m.group(1).split(", ") if n[0].isupper())
            continue
        if block == "struct":
            if m := re.match(r"\t(\w+(?:, \w+)*)\s+\S", line):
                names.extend(f"{owner}.{n}" for n in m.group(1).split(", ") if n[0].isupper())
            continue

        if m := re.match(r"func \(\w+ \*?(\w+)(?:\[[^\]]*\])?\) (\w+)", line):
            if m.group(2)[0].isupper():
                names.append(f"{m.group(1)}.{m.group(2)}")
        elif m := re.match(r"func (\w+)", line):
            if m.group(1)[0].isupper():
                names.append(m.group(1))
        elif m := re.match(r"type (\w+)(?:\[[^\]]*\])? (.*)", line):
            if m.group(1)[0].isupper():
                names.append(m.group(1))
                if m.group(2).startswith("struct {") and not line.endswith("}"):
                    block, owner = "struct", m.group(1)
        elif re.match(r"(?:var|const) \($", line):
            block = "decl"
        elif m := re.match(r"(?:var|const) (\w+)", line):
            if m.group(1)[0].isupper():
                names.append(m.group(1))
    return names


TS_MEMBER = re.compile(
    r"    (?:public )?(?:static )?(?:readonly )?(?:async )?(?:get |set )?\*?\s*(\w+)\s*[(<:=?!;]"
)
TS_PARAM_PROPERTY = re.compile(r"(?<!private )(?<!protected )(?:public|readonly)\s+(?:readonly\s+)?(\w+)")


def typescript_symbols(source: str) -> list[str]:
    names: list[str] = []
    owner, kind = None, None
    in_constructor_params = False
    for line in source.splitlines():
        if line.startswith("}"):
            owner, kind = None, None
            continue

        if not line.startswith(" "):
            if m := re.match(r"export (?:default )?(?:abstract )?class (\w+)", line):
                names.append(m.group(1))
                owner, kind = m.group(1), "class"
            elif m := re.match(r"export (?:default )?(?:async )?function\*?\s*(\w+)", line):
                names.append(m.group(1))
            elif m := re.match(r"export (?:declare )?(?:const|let|var|type|interface|enum) (\w+)", line):
                names.append(m.group(1))
                if line.rstrip().endswith("{"):
                    kind = "other"
            elif re.match(r"(?:abstract )?(?:class|interface|enum) ", line):
                kind = "private"
            continue

        if kind != "class":
            continue

        if in_constructor_params:
            if line.startswith("    )"):
                in_constructor_params = False
            elif m := re.match(r"        " + TS_PARAM_PROPERTY.pattern, line):
                names.append(f"{owner}.{m.group(1)}")
            continue

        if m := re.match(r"    (private |protected |public )?constructor\((.*)", line):
            if m.group(1) != "private " and m.group(1) != "protected ":
                names.append(f"{owner}.constructor")
            rest = m.group(2)
            if rest.strip() == "":
                in_constructor_params = True
            else:
                params = rest.split(")")[0]
                names.extend(f"{owner}.{p}" for p in TS_PARAM_PROPERTY.findall(params))
            continue

        if re.match(r"    (?:private |protected |#|//|/\*|\})", line) or line.startswith("     "):
            continue
        if m := TS_MEMBER.match(line):
            names.append(f"{owner}.{m.group(1)}")
    return names


def python_symbols(source: str) -> list[str]:
    names: list[str] = []
    owner = None
    in_docstring = False
    for line in source.splitlines():
        if line.count('"""') % 2 == 1:
            in_docstring = not in_docstring
            continue
        if in_docstring:
            continue

        if line and not line[0].isspace() and not line.startswith(("@", "#", ")", "]", '"""')):
            owner = None
            if m := re.match(r"class (\w+)", line):
                owner = m.group(1) if not m.group(1).startswith("_") else "_"
                if owner != "_":
                    names.append(owner)
            elif m := re.match(r"def (\w+)", line):
                if not m.group(1).startswith("_"):
                    names.append(m.group(1))
            elif m := re.match(r"([A-Za-z]\w*)\s*(?::[^=]+)?=", line):
                names.append(m.group(1))
            continue

        if owner in (None, "_"):
            continue
        if m := re.match(r"    def (\w+)", line):
            if m.group(1) == "__init__" or not m.group(1).startswith("_"):
                names.append(f"{owner}.{m.group(1)}")
        elif m := re.match(r"    ([A-Za-z]\w*)\s*(?::[^=]+)?(?:=|$)", line):
            names.append(f"{owner}.{m.group(1)}")
        elif m := re.match(r"        self\.([A-Za-z]\w*)\s*(?::[^=]+)?=", line):
            names.append(f"{owner}.{m.group(1)}")

    # Re-exports, such as stdlib functions a module passes through.
    if m := re.search(r"^__all__\s*=\s*[\[(](.*?)[\])]", source, re.MULTILINE | re.DOTALL):
        names.extend(re.findall(r"[\"'](\w+)[\"']", m.group(1)))
    return names


EXTRACTORS = {
    "Rust": rust_symbols,
    "Go": go_symbols,
    "TypeScript": typescript_symbols,
    "Python": python_symbols,
}


def code_symbols() -> dict[str, dict[str, list[str]]]:
    """{module: {language: [symbol, ...]}}"""
    found: dict[str, dict[str, list[str]]] = {}
    for language, (directory, pattern) in LANGUAGES.items():
        for path in sorted((REPO / directory).glob(pattern)):
            if not is_source(language, path):
                continue
            symbols = EXTRACTORS[language](path.read_text())
            module = found.setdefault(path.stem, {})
            module[language] = list(dict.fromkeys(symbols))
    return found


class Row:
    def __init__(self, line_number: int, cells: dict[str, str], notes: str) -> None:
        self.line_number = line_number
        self.cells = cells
        self.notes = notes

    def names(self, language: str) -> list[str]:
        cell = self.cells.get(language, "")
        if cell.startswith("native:"):
            return []
        return [name for name in re.findall(r"`([^`]+)`", cell) if NAME.fullmatch(name)]

    def missing(self) -> list[str]:
        return [language for language in LANGUAGES if self.cells.get(language, "") == ""]

    def waiting(self) -> list[str]:
        return [language for language in LANGUAGES if self.cells.get(language, "").lower() == "wip"]

    def label(self) -> str:
        for language in LANGUAGES:
            if names := self.names(language):
                return f"{language} {', '.join(names)}"
        return f"line {self.line_number}"


def split_row(line: str) -> list[str]:
    """Splits a table row on its unescaped pipes."""
    return [cell.strip() for cell in re.split(r"(?<!\\)\|", line.strip().strip("|"))]


def read_map() -> dict[str, list[Row]]:
    """{module: [row, ...]} from the `## module` sections of the API map."""
    modules: dict[str, list[Row]] = {}
    if not API_MAP.exists():
        return modules
    module, columns = None, None
    for number, line in enumerate(API_MAP.read_text().splitlines(), start=1):
        if line.startswith("#"):
            # Only headings like "## `grid`" start a module; any other heading ends one.
            m = re.match(r"## `(\w+)`\s*$", line)
            module, columns = (m.group(1) if m else None), None
            if module:
                modules.setdefault(module, [])
        elif module and line.startswith("|"):
            cells = split_row(line)
            if columns is None:
                columns = cells
            elif not set(line) <= set("|-: "):
                values = dict(zip(columns, cells))
                row_cells = {language: values.get(language, "") for language in LANGUAGES}
                modules[module].append(Row(number, row_cells, values.get("Notes", "")))
        else:
            columns = None
    return modules


def main(scope: list[str]) -> int:
    code = code_symbols()
    api_map = read_map()
    modules = sorted(set(code) | set(api_map))
    if scope:
        unknown = set(scope) - set(modules)
        if unknown:
            print(f"Unknown modules: {', '.join(sorted(unknown))}. Known: {', '.join(modules)}")
            return 2
        modules = [m for m in modules if m in scope]

    unmapped, stale, to_port, waiting = [], [], [], []
    for module in modules:
        rows = api_map.get(module, [])
        for language in LANGUAGES:
            in_code = code.get(module, {}).get(language, [])
            in_map = [name for row in rows for name in row.names(language)]
            unmapped += [f"{module}  {language}  {n}" for n in in_code if n not in in_map]
            stale += [f"{module}  {language}  {n}" for n in in_map if n not in in_code]
        for row in rows:
            note = f"  [{row.notes}]" if row.notes else ""
            if missing := row.missing():
                to_port.append(f"{module}  {row.label()}  ->  {', '.join(missing)}{note}")
            if languages := row.waiting():
                waiting.append(f"{module}  {row.label()}  ->  {', '.join(languages)}{note}")

    present = {
        module: [language for language in LANGUAGES if language in code.get(module, {})]
        for module in modules
    }
    print("Modules (languages with a file):")
    for module in modules:
        print(f"  {module:<12} {', '.join(present[module]) or '(none)'}")

    for title, items in (
        ("Unmapped: in the code, not in the map", unmapped),
        ("Stale: in the map, not in the code", stale),
        ("To port: rows with an empty cell", to_port),
        ("Waiting: rows marked wip, not counted as drift", waiting),
    ):
        print(f"\n{title} ({len(items)})")
        for item in items:
            print(f"  {item}")

    return 1 if unmapped or stale or to_port else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
