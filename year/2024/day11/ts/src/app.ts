import { PathLike } from "node:fs";
import fs from "fs/promises";
import { countAllStones } from "./blink";

async function processInputFile(path: PathLike) {
    const fileStream = await fs.readFile(path);
    const stones = fileStream.toString().split(" ").map(Number);

    const part1 = await countAllStones(stones, 25);
    console.log("part 1:", part1);

    const part2 = await countAllStones(stones, 75);
    console.log("part 1:", part2);
}

if (process.argv.length < 3) {
    console.error("one argument is expected (path to input file).");
    process.exit();
}

processInputFile(process.argv[2]).catch((reason) => console.error(reason));
