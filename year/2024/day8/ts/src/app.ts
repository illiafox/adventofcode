import { PathLike } from "node:fs";
import fs from "fs/promises";
import * as os from "node:os";

function combinations(
    antennas: Iterable<[number, number][]>,
    callback: (
        x1: number,
        y1: number,
        x2: number,
        y2: number,
        diffX: number,
        diffY: number,
    ) => void,
) {
    for (const coords of antennas) {
        coords.forEach(([x1, y1], i) => {
            coords.forEach(([x2, y2], j) => {
                if (i === j) return;
                callback(x1, y1, x2, y2, x2 - x1, y2 - y1);
            });
        });
    }
}

function calculatePart1(
    grid: string[][],
    antennas: Iterable<[number, number][]>,
): number {
    let count = 0;

    combinations(antennas, (x1, y1, x2, y2, diffX, diffY) => {
        const positions = [
            [x1 - diffX, y1 - diffY],
            [x2 + diffX, y2 + diffY],
        ];

        positions.forEach(([x, y]) => {
            if (grid[y]?.[x] !== "#" && grid[y]?.[x] !== undefined) {
                count++;
                grid[y][x] = "#";
            }
        });
    });

    return count;
}

function calculatePart2(
    grid: string[][],
    antennas: Iterable<[number, number][]>,
): number {
    combinations(antennas, (x1, y1, x2, y2, diffX, diffY) => {
        let found = true;

        for (let step = 1; found; step++) {
            found = false;

            const positions = [
                [x1 - diffX * step, y1 - diffY * step],
                [x1 + diffX * step, y1 + diffY * step],
            ];

            positions.forEach(([x, y]) => {
                if (grid[y]?.[x] !== undefined) {
                    grid[y][x] = "#";
                    found = true;
                }
            });
        }
    });

    return grid.flat().filter((cell) => cell !== ".").length;
}

async function processInputFile(path: PathLike) {
    const fileStream = await fs.readFile(path);
    const grid = fileStream
        .toString()
        .split(os.EOL)
        .map((line) => line.split(""));

    const antennas = new Map<string, [number, number][]>();

    grid.forEach((line, i) => {
        line.forEach((symbol, j) => {
            if (symbol !== ".") {
                if (!antennas.has(symbol)) {
                    antennas.set(symbol, [[j, i]]);
                } else {
                    antennas.get(symbol)!.push([j, i]);
                }
            }
        });
    });

    const part1 = calculatePart1(grid, antennas.values());
    const part2 = calculatePart2(grid, antennas.values());

    console.log("part 1:", part1);
    console.log("part 2:", part2);
}

if (process.argv.length < 3) {
    console.error("one argument is expected (path to input file).");
    process.exit();
}

processInputFile(process.argv[2]).catch((reason) => console.error(reason));
