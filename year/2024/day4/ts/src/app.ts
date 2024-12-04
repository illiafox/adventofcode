import {PathLike} from "node:fs";

import fs from "fs/promises";
import * as os from "node:os";

const word = ["M", "A", "S"];

function traverse(
    arr: string[],
    symbolIdx: number,
    i: number,
    j: number,
    nextChange: number[],
): number {
    while (true) {
        i += nextChange[1];
        j += nextChange[0];

        switch (true) {
            case i < 0 || i >= arr.length:
            case j < 0 || j >= arr[0].length:
                return 0;
        }

        if (arr[i][j] != word[symbolIdx]) {
            return 0;
        }

        symbolIdx++;
        if (word.length == symbolIdx) return 1;
    }
}

function xmas(lines: string[]): [number, number] {
    let part1 = 0;
    let part2 = 0;

    // for part 2
    const isValidPosition = (i: number, j: number) =>
        i > 0 && j > 0 && i < lines.length - 1 && j < lines[0].length - 1;

    for (let i = 0; i < lines.length; i++) {
        for (let j = 0; j < lines[0].length; j++) {

            if (lines[i][j] === "X") { // part 1
                for (const move of [
                    [1, 0], [-1, 0], [0, 1], [0, -1], // up, down, right, left
                    [1, 1], [-1, -1], [1, -1], [-1, 1], // diagonals
                ]) {
                    part1 += traverse(lines, 0, i, j, move);
                }
            }

            if (lines[i][j] === "A") { // part 2
                if (!isValidPosition(i, j)) continue;

                const firstDiag = lines[i + 1][j + 1] + lines[i - 1][j - 1];
                const secondDiag = lines[i + 1][j - 1] + lines[i - 1][j + 1];

                const search = ["MS", "SM"];
                if (search.includes(firstDiag) && search.includes(secondDiag))
                    part2++;
            }
        }
    }

    return [part1, part2];
}

async function processFile(path: PathLike) {
    const file = await fs.readFile(path);
    const lines = file.toString().split(os.EOL);

    const res = xmas(lines);

    console.log("part 1:", res[0]);
    console.log("part 2:", res[1]);
}

if (process.argv.length < 3) {
    console.error("one argument is expected");
    process.exit();
}

processFile(process.argv[2]).catch((reason) => console.error(reason));
