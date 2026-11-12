import { Solution } from "../../solution";
import { windows } from "../../utils/arrays";

export class Day10Solution2025 extends Solution<number, number> {
    constructor(inputPath: string) {
        super(inputPath);
    }

    part1(): number {
        return this.lines().map(this.getMachineFromLine).map((m) => m.solveP1()).reduce((a, c) => a + c, 0);
    }

    part2(): number {
        return this.lines().map(this.getMachineFromLine).map((m) => m.solveP2()).reduce((a, c) => a + c, 0);
    }

    getMachineFromLine(line: string): Machine {
        const split = line.split(" ");

        // Parse desired
        const desired = split[0]!.slice(1, -1);
        const desiredState = [];
        for (const c of desired) {
            switch (c) {
                case ".": {
                    desiredState.push(false);
                    break;
                }
                case "#": {
                    desiredState.push(true);
                    break;
                }
                default: {
                    throw new Error("UNREACHABLE: desired state not \".\" or \"#\"");
                }
            }
        }

        // Parse button inputs
        const buttons = [];
        for (let i = 1; i < split.length - 1; i++) {
            const button = split[i]!.slice(1, -1).split(",").map(
                (v) => Number(v)
            ) as Button;
            buttons.push(button);
        }

        // Parse joltages
        const joltages = split[split.length - 1]!.slice(1, -1).split(",").map((v) => +v);

        return new Machine(
            desiredState,
            buttons,
            joltages
        );
    }
}

class Machine {
    currentState: boolean[];
    currentJoltages: number[];

    constructor(
        public desiredState: boolean[],
        public buttons: Button[],
        public desiredJoltages: number[],
    ) {
        this.currentState = Array.from({ length: buttons.length }, () => false);
        this.currentJoltages = Array.from({ length: buttons.length }, () => 0);
    }

    solveP1() {
        let i = 0;
        while (true) {
            i++;
            for (const perm of this.buttonCombinationsP1(i)) {
                for (const ix of perm) {
                    this.pressButton(this.buttons[ix]!);
                }

                if (this.stateMatches()) {
                    return i;
                }

                this.resetState();
            }
        }
    }

    solveP2() {
        return 0;
    }

    stateMatches() {
        return this.arrayEqual(this.currentState, this.desiredState);
    }

    joltagesMatch() {
        return this.arrayEqual(this.currentJoltages, this.desiredJoltages);
    }

    pressButton(button: Button) {
        for (const i of button) {
            this.currentState[i] = !this.currentState[i];
        }
    }

    pressButtonP2(button: Button) {
        for (const i of button) {
            this.currentJoltages[i]! += 1;
        }
    }

    *buttonCombinationsP1(len: number): Generator<number[]> {
        const btnIndicies = Array.from({ length: len }, () => 0);
        const maxPerms = this.buttons.length ** len;
        for (let i = 0; i < maxPerms; i++) {
            yield this.dedupeCombo([...btnIndicies]);
            // Increment indicies by 1 and carry if needed
            for (let j = 0; j < btnIndicies.length; j++) {
                btnIndicies[j] = (btnIndicies[j]! + 1) % this.buttons.length;
                if (btnIndicies[j] === 0) {
                    continue;
                } else {
                    break;
                }
            }

        }
    }

    dedupeCombo(combo: number[]): number[] {
        while (true) {
            let deduped = false;
            if (combo.length < 2) {
                return combo;
            }
            for (const win of windows(combo, 2)) {
                if (win.window[0] === win.window[1]) {
                    const firstSlice = combo.slice(0, win.startIdx);
                    const afterSlice = combo.slice(win.endIdx);
                    combo = firstSlice.concat(afterSlice);
                    deduped = true;
                    break;
                }
            }

            if (!deduped) {
                break;
            }
        }

        return combo;
    }

    arrayEqual<T>(a: T[], b: T[]): boolean {
        if (a.length !== b.length) {
            return false;
        }
        for (let i = 0; i < a.length; i++) {
            if (a[i]! !== b[i]!) {
                return false;
            }
        }

        return true;
    }

    resetState() {
        this.currentState = Array.from({
            length: this.desiredState.length
        }, () => false);
        this.currentJoltages = Array.from({ length: this.desiredJoltages.length }, () => 0);
    }
};

type Button = number[];

class P2PressState {
    internal: {
        [x: number]: {
            max: number,
            buttons: {
                button: Button,
                presses: number,
                buttonIdx: number,
            }[],
        }
    };

    constructor() {
        this.internal = {};
    }

    validate(): boolean {
        return !Object.values(this.internal).some((v) => v.buttons.reduce((a, c) => a + c.presses, 0) > v.max);
    }
}