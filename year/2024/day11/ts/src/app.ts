import { PathLike } from "node:fs";
import fs from "fs/promises";

function blink(dist: Map<number, number>): Map<number, number> {
    const newDist = new Map<number, number>();

    for (const [stone, count] of dist.entries()) {
        const val = stone;

        if (val === 0) {
            // rule 1: 0 -> 1
            newDist.set(1, (newDist.get(1) || 0) + count);
        } else {
            const s = stone.toString();
            if (s.length % 2 === 0) {
                // rule 2: even length -> split
                const half = s.length / 2;
                const left = Number(s.slice(0, half));
                const right = Number(s.slice(half));

                newDist.set(left, (newDist.get(left) || 0) + count);
                newDist.set(right, (newDist.get(right) || 0) + count);
            } else {
                // rule 3: else -> multiply by 2024
                const newVal = val * 2024;
                newDist.set(newVal, (newDist.get(newVal) || 0) + count);
            }
        }
    }

    return newDist;
}

async function countStones(n: number, blinks: number): Promise<number> {
    return new Promise((resolve) => {
        let dist = new Map<number, number>();
        dist.set(n, 1);

        for (let i = 0; i < blinks; i++) {
            dist = blink(dist);
        }

        let total = 0;
        for (const val of dist.values()) {
            total += val;
        }

        resolve(total);
    });
}

async function countAllStones(nums: number[], blinks: number): Promise<number> {
    const promises = nums.map((n) => countStones(n, blinks));
    const results = await Promise.all(promises);
    return results.reduce((acc, val) => acc + val, 0);
}

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
