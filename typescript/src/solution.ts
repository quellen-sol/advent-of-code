import { readFileSync } from "fs";

export interface ISolution<T1, T2> {
    part1(): T1;
    part2(): T2;
}

export class Solution<T1, T2> implements ISolution<T1, T2> {
    protected readonly input: string;

    constructor(inputPath: string) {
        this.input = readFileSync(inputPath, "utf-8");
    }

    part1(): T1 {
        throw new Error("Method not implemented. Override in subclass.");
    }

    part2(): T2 {
        throw new Error("Method not implemented. Override in subclass.");
    }

    timeSolutions() {
        console.time("Part 1");
        console.log("Part 1 Solution:", this.part1());
        console.timeEnd("Part 1");

        console.time("Part 2");
        console.log("Part 2 Solution:", this.part2());
        console.timeEnd("Part 2");
    }
}
