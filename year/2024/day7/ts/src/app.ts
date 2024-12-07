import { PathLike } from "node:fs";

import fs from "fs";
import readline from "readline";

enum Action {
    Add = 1,
    Mul,
    Con,
}

function isPossible(
    required: number,
    current: number,
    vals: number[],
    nextAction: Action,
    allActions: Action[],
): boolean {
    const next = vals[0];
    vals = vals.slice(1);

    switch (nextAction) {
        case Action.Add:
            current += next;
            break;
        case Action.Mul:
            current *= next;
            break;
        case Action.Con:
            current = Number(current.toString() + next.toString());
    }

    if (vals.length === 0) return required === current;

    return allActions.some((action) =>
        isPossible(required, current, vals, action, allActions),
    );
}

async function processLineByLine(path: PathLike) {
    const fileStream = fs.createReadStream(path);

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity,
    });

    const part1Actions = [Action.Mul, Action.Add];
    const part2Actions = [...part1Actions, Action.Con];

    const startParams = [
        [Action.Add, 0], // 0 + val = val
        [Action.Mul, 1], // 1 * val = val
    ];

    let part1 = 0;
    let part2 = 0;

    for await (const line of rl) {
        if (line === "") break;
        const nums = line.split(": ");

        const required = Number(nums[0]);
        const values = nums[1].split(" ").map(Number);

        for (const [action, start] of startParams) {
            if (isPossible(required, start, values, action, part1Actions)) {
                part1 += required;
                part2 += required;
                break;
            }
            if (isPossible(required, start, values, action, part2Actions)) {
                part2 += required;
                break;
            }
        }
    }

    console.log("part 1: ", part1);
    console.log("part 2: ", part2);
}

if (process.argv.length < 3) {
    console.error("one argument is expected");
    process.exit();
}

processLineByLine(process.argv[2]).catch((reason) => console.error(reason));
