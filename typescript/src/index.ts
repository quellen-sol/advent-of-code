import { parseArgs } from "node:util";

import { runSolution } from "./timing";

const { values } = parseArgs({
    options: {
        year: { type: "string" },
        day: { type: "string" },
        input: { type: "string" },
        bench: { type: "string" },
    },
});

const { year, day, input, bench } = values;
if (!year || !day || !input) {
    throw new Error(
        "Usage: --year <year> --day <day> --input <path> [--bench <seconds>]",
    );
}

const dayModule = require(`./years/${year}/day_${day}`);
const DaySolution = dayModule[`Day${day}Solution${year}`];
runSolution(() => new DaySolution(input), Number(bench ?? 0));
