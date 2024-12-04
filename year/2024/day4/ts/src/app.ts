import {PathLike} from "node:fs";

import fs from "fs/promises";
import * as os from "node:os";

function traverse(
    arr: string[],
    i: number,
    j: number,
    change: number[],
): number {
    const word = ["M", "A", "S"];

    for (let state = 0; state < word.length; state++) {
        [i, j] = [i + change[1], j + change[0]];

        if (!arr[i] || arr[i][j] !== word[state]) return 0;
    }

    return 1;
}

function xmas(lines: string[]): [number, number] {
    let part1 = 0;
    let part2 = 0;

    for (let i = 0; i < lines.length; i++) {
        for (let j = 0; j < lines[i].length; j++) {
            if (lines[i][j] === "X") {
                // part 1
                const moves = [
                    [1, 0], [-1, 0], [0, 1], [0, -1], // up, down, right, left
                    [1, 1], [-1, -1], [1, -1], [-1, 1], // diagonals
                ]

                for (const move of moves) {
                    part1 += traverse(lines, i, j, move);
                }
            }

            if (
                lines[i][j] === "A" &&
                lines[i - 1]?.[i - 1] && // check bounds
                lines[i + 1]?.[i + 1]
            ) {
                // part 2
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
