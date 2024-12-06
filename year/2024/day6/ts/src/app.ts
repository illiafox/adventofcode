import { PathLike } from "node:fs";

import fs from "fs";
import readline from "readline";

function traverse(
    map: string[][],
    startPos: [number, number],
    limit: number | null,
): number {
    const m = new Set<string>();

    const directions = [
        [0, -1],
        [1, 0],
        [0, 1],
        [-1, 0],
    ];

    let dirIdx = 0;
    let [x, y] = startPos;
    let steps = 0;

    while (true) {
        if (limit !== null && steps > limit) return -1;

        const curDir = directions[dirIdx];
        y += curDir[1];
        x += curDir[0];

        if (map[y] === undefined || map[y][x] === undefined) {
            return m.size;
        }

        if (map[y][x] === "#") {
            y -= curDir[1];
            x -= curDir[0];

            dirIdx = (dirIdx + 1) % directions.length;
        } else {
            const o: [number, number] = [x, y];
            m.add(JSON.stringify(o));

            steps++;
        }
    }
}

async function processLineByLine(path: PathLike) {
    const fileStream = fs.createReadStream(path);

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity,
    });

    const map: string[][] = [];

    let startPos: [number, number] = [-1, -1];

    let i = 0;
    for await (const line of rl) {
        if (line === "") break;

        const nums = line.split("");

        // console.log(line);

        const idx = nums.indexOf("^");
        if (idx != -1) {
            startPos = [idx, i];
        }

        map.push(nums);
        i++;
    }

    // console.log(startPos);

    const good = traverse(map, startPos, null);

    const limit = good * 2;
    console.log("part 1:", good);

    let part2 = 0;

    for (let i = 0; i < map.length; i++) {
        for (let j = 0; j < map[0].length; j++) {
            if (map[i][j] === "^" || map[i][j] === "#") continue;

            const old = map[i][j];
            map[i][j] = "#";
            part2 += traverse(map, startPos, limit) === -1 ? 1 : 0;
            map[i][j] = old;
        }
    }

    console.log("part 2: ", part2);
}

if (process.argv.length < 3) {
    console.error("one argument is expected");
    process.exit();
}

processLineByLine(process.argv[2]).catch((reason) => console.error(reason));
