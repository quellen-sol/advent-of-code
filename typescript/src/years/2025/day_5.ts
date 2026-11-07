import { Solution } from "../../solution";
import { chunks } from "../../utils/arrays";

export class Day5Solution2025 extends Solution<number, number> {
    ranges: Range[] = [];
    values: number[] = [];

    constructor(inputPath: string) {
        super(inputPath);

        // First ranges, then values
        let searchingValues = false;
        for (const line of this.input.split("\n")) {
            if (!searchingValues) {
                // Ranges
                if (line.length === 0) {
                    searchingValues = true;
                    continue;
                }

                const [num1, num2] = line.split("-").map((v) => Number(v));
                if (num1 === undefined || num2 === undefined) {
                    throw new Error("Bad \'-\' split when parsing input");
                }
                const range = new Range(num1, num2);
                this.ranges.push(range);
            } else {
                // Values
                const num = Number(line);
                this.values.push(num);
            }
        }

        this.mergeRanges();
    }

    part1(): number {
        let totalValids = 0;
        for (const val of this.values) {
            let isInRange = false;
            for (const range of this.ranges) {
                if (range.isValueWithinRange(val)) {
                    isInRange = true;
                    break;
                }
            }
            if (isInRange) {
                totalValids++;
            }
        }

        return totalValids;
    }

    part2(): number {
        return this.ranges.reduce<number>((acc, curr) => {
            acc += curr.end - curr.start + 1
            return acc;
        }, 0)
    }

    mergeRanges() {
        this.ranges.sort((a, b) => a.start - b.start)
        let newRanges: Range[] = [];
        let oldLen = this.ranges.length;
        do {
            newRanges = []
            for (const chunk of chunks(this.ranges, 2)) {
                const r1 = chunk[0];
                const r2 = chunk[1];
                if (r1 === undefined || r2 === undefined) {
                    continue;
                }

                const merged = r1.merge(r2);
                if (merged !== undefined) {
                    newRanges.push(merged);
                } else {
                    newRanges.push(r1, r2);
                }
            }

            oldLen = this.ranges.length;
            this.ranges = newRanges;
        } while (oldLen != newRanges.length)
    }
}

class Range {
    constructor(
        public start: number,
        public end: number,
    ) { }

    merge(other: Range): Range | undefined {
        if (this.start <= other.end && this.end >= other.start) {
            return new Range(Math.min(this.start, other.start), Math.max(this.end, other.end));
        }

        return undefined;
    }

    isValueWithinRange(value: number): boolean {
        return value >= this.start && value <= this.end;
    }
}
