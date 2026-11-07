import { parseArgs } from "node:util";

const { values } = parseArgs({
    options: {
        year: { type: "string" },
        day: { type: "string" },
        input: { type: "string" },
    },
});

const { year, day, input } = values;
if (!year || !day || !input) {
    throw new Error("Usage: --year <year> --day <day> --input <path>");
}

const dayModule = require(`./years/${year}/day_${day}`);
const DaySolution = dayModule[`Day${day}Solution${year}`];
new DaySolution(input).timeSolutions();
