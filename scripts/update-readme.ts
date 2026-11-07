import axios from "axios";
import { readFileSync, writeFileSync } from "fs";

type AdventAPIReponse = {
  event: string;
  members: {
    [id: string]: {
      name: string;
      completion_day_level: {
        [day: string]: {
          [part: string]: {
            star_index: number;
            get_star_ts: string;
          }
        }
      }
    }
  }
}

const CHECKMARK = "✅";
const XMARK = "❌";

const LEADERBOARD_ID = 403802;
const AOC_UTC_OFFSET_HOURS = -5;

const ADVENT_SESSION_KEY = process.env.ADVENT_SESSION_KEY!;

const README_BASE = readFileSync("README.BASE.md", "utf-8");

/** The most recent event on AoC's clock: this year's in December, otherwise last year's. */
function latestEventYear(now: Date) {
  const aocNow = new Date(now.getTime() + AOC_UTC_OFFSET_HOURS * 60 * 60 * 1000);
  const year = aocNow.getUTCFullYear();
  return aocNow.getUTCMonth() === 11 ? year : year - 1;
}

function createRow(day: number, part1: boolean, part2: boolean) {
  return `| ${day} | ${part1 ? CHECKMARK : XMARK} | ${part2 ? CHECKMARK : XMARK} |\n`;
}

async function main() {
  const year = latestEventYear(new Date());
  const url = `https://adventofcode.com/${year}/leaderboard/private/view/${LEADERBOARD_ID}.json`;
  const { data } = await axios.get<AdventAPIReponse>(url, {
    headers: {
      Cookie: `session=${ADVENT_SESSION_KEY}`
    }
  });

  if (!data.members) {
    throw new Error("Leaderboard response has no members. Has ADVENT_SESSION_KEY expired?");
  }

  const quellen = Object.values(data.members).find(member => member.name === "quellen-sol");
  if (!quellen) {
    throw new Error("Could not find quellen-sol in the leaderboard");
  }

  const newRows: string[] = [];
  for (let day = 1; day <= 25; day++) {
    const part1 = quellen.completion_day_level[day.toString()]?.["1"]?.get_star_ts;
    const part2 = quellen.completion_day_level[day.toString()]?.["2"]?.get_star_ts;
    if (!part1 && !part2) {
      continue;
    }
    newRows.push(createRow(day, !!part1, !!part2));
  }

  const rowString = newRows.join("");
  const completeReadme = README_BASE.replaceAll("{{YEAR}}", year.toString()) + rowString;
  writeFileSync("README.md", completeReadme, "utf-8");
}

main();
