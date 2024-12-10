import { PathLike } from "node:fs";
import fs from "fs/promises";
import * as os from "node:os";

function calculate(
    grid: number[][],
    startI: number,
    startJ: number,
): [number, number] {
    const visited = new Set<string>();

    const directions = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    const oppositeDirections = [1, 0, 3, 2];

    function traverse(
        curNum: number,
        i: number,
        j: number,
        lastDirection: number,
    ): number {
        const val = grid[i]?.[j];
        if (val === undefined || val <= curNum || val - curNum > 1) return 0;

        curNum++;
        if (curNum === 9) {
            visited.add(JSON.stringify([i, j]));
            return 1;
        }

        let result = 0;
        for (let d = 0; d < directions.length; d++) {
            if (oppositeDirections[lastDirection] === d) continue;

            const [dx, dy] = directions[d];
            result += traverse(curNum, i + dx, j + dy, d);
        }

        return result;
    }

    const totalPaths = traverse(-1, startI, startJ, -1);
    return [visited.size, totalPaths];
}

async function processInputFile(path: PathLike) {
    const fileStream = await fs.readFile(path);
    const grid = fileStream
        .toString()
        .split(os.EOL)
        .map((line) => line.split("").map(Number));

    let part1 = 0;
    let part2 = 0;

    grid.forEach((line, i) => {
        line.forEach((num, j) => {
            if (num == 0) {
                const [a, b] = calculate(grid, i, j);
                part1 += a;
                part2 += b;
            }
        });
    });

    console.log("part 1:", part1);
    console.log("part 2:", part2);
}

if (process.argv.length < 3) {
    console.error("one argument is expected (path to input file).");
    process.exit();
}

processInputFile(process.argv[2]).catch((reason) => console.error(reason));
