import time
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

    def time_solutions(self) -> None:
        for name, part in (("Part 1", self.part1), ("Part 2", self.part2)):
            start = time.perf_counter()
            result = part()
            elapsed_ms = (time.perf_counter() - start) * 1000
            print(f"{name} Solution: {result}")
            print(f"{name}: {elapsed_ms:.3f}ms")
