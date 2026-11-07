import argparse
import importlib

from py.timing import run_solution


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--year", type=int, required=True)
    parser.add_argument("--day", type=int, required=True)
    parser.add_argument("--input", required=True)
    parser.add_argument("--bench", type=float, default=0.0)
    args = parser.parse_args()

    module = importlib.import_module(f"py.years.y{args.year}.day_{args.day}")
    solution = getattr(module, f"Day{args.day}Solution")
    run_solution(lambda: solution(args.input), args.bench)
