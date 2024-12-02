import { PathLike } from "node:fs";

import fs from "fs";
import readline from "readline";
import { processReports } from "./report/report";

async function processLineByLine(path: PathLike) {
  const fileStream = fs.createReadStream(path);

  const rl = readline.createInterface({
    input: fileStream,
    crlfDelay: Infinity,
  });

  const result = await processReports(rl);
  console.log("part 1: safe reports:", result.safeReports);
  console.log("part 2: tolerated safe reports:", result.toleratedSafeReports);
}

if (process.argv.length < 3) {
  console.error("one argument is expected");
  process.exit();
}

processLineByLine(process.argv[2]).catch((reason) => console.error(reason));
