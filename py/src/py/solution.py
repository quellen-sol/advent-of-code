from pathlib import Path


class Solution[T1, T2]:
    def __init__(self, input_path: str) -> None:
        self.input = Path(input_path).read_text()

    @property
    def lines(self) -> list[str]:
        return self.input.splitlines()

    def part1(self) -> T1:
        raise NotImplementedError("Override in subclass.")

    def part2(self) -> T2:
        raise NotImplementedError("Override in subclass.")
