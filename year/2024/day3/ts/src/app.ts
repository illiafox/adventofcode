import { PathLike } from "node:fs";

import fs from "fs/promises";

async function processLineByLine(path: PathLike) {
  const file = await fs.readFile(path);

  const regex = /mul\((\d+),(\d+)\)|do\(\)|don't\(\)/g;

  const matches = file.toString().matchAll(regex);

  let sum = 0;

  let sumWithInstructions = 0;
  let enabled = true;

  for (const match of matches) {
    switch (match[0]) {
      case "do()":
        enabled = true;
        continue;
      case "don't()":
        enabled = false;
        continue;
    }

    // mul(x,y)

    const res = Number(match[1]) * Number(match[2]);

    sum += res;
    if (enabled) sumWithInstructions += res;
  }

  console.log("part 1: sum:", sum);
  console.log(
    "part 2: sum with do and don't instructions:",
    sumWithInstructions,
  );
}

if (process.argv.length < 3) {
  console.error("one argument is expected");
  process.exit();
}

processLineByLine(process.argv[2]).catch((reason) => console.error(reason));
