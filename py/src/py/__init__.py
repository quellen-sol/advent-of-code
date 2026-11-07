import argparse
import importlib


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--year", type=int, required=True)
    parser.add_argument("--day", type=int, required=True)
    parser.add_argument("--input", required=True)
    args = parser.parse_args()

    module = importlib.import_module(f"py.years.y{args.year}.day_{args.day}")
    solution = getattr(module, f"Day{args.day}Solution")(args.input)
    solution.time_solutions()
