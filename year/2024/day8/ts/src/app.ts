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
    map: string[][],
    antennas: Iterable<[number, number][]>,
): number {
    let count = 0;

    combinations(antennas, (x1, y1, x2, y2, diffX, diffY) => {
        const positions = [
            [x1 - diffX, y1 - diffY],
            [x2 + diffX, y2 + diffY],
        ];

        positions.forEach(([x, y]) => {
            if (map[y]?.[x] !== "#" && map[y]?.[x] !== undefined) {
                count++;
                map[y][x] = "#";
            }
        });
    });

    return count;
}

function calculatePart2(
    map: string[][],
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
                if (map[y]?.[x] !== undefined) {
                    map[y][x] = "#";
                    found = true;
                }
            });
        }
    });

    return map.flat().filter((cell) => cell !== ".").length;
}

async function processInputFile(path: PathLike) {
    const fileStream = await fs.readFile(path);
    const lines = fileStream
        .toString()
        .split(os.EOL)
        .map((line) => line.split(""));

    const antennasMap = new Map<string, [number, number][]>();

    lines.forEach((line, rowIndex) => {
        line.forEach((symbol, colIndex) => {
            if (symbol !== ".") {
                if (!antennasMap.has(symbol)) {
                    antennasMap.set(symbol, [[colIndex, rowIndex]]);
                } else {
                    antennasMap.get(symbol)!.push([colIndex, rowIndex]);
                }
            }
        });
    });

    const part1 = calculatePart1(lines, antennasMap.values());
    const part2 = calculatePart2(lines, antennasMap.values());

    console.log("part 1:", part1);
    console.log("part 2:", part2);
}

if (process.argv.length < 3) {
    console.error("one argument is expected (path to input file).");
    process.exit();
}

processInputFile(process.argv[2]).catch((reason) => console.error(reason));
