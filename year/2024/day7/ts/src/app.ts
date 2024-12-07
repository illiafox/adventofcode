import { PathLike } from "node:fs";

import fs from "fs";
import readline from "readline";

enum Action {
    Add = 1,
    Mul,
    Con,
}

function isPossible(
    testVal: number,
    current: number,
    vals: number[],
    action: Action,
    allowedActions: Action[],
): boolean {
    const next = vals[0];
    vals = vals.slice(1);

    switch (action) {
        case Action.Add:
            current += next;
            break;
        case Action.Mul:
            current *= next;
            break;
        case Action.Con:
            current = Number(current.toString() + next.toString());
    }

    if (vals.length === 0) return testVal === current;

    for (const a of allowedActions) {
        if (isPossible(testVal, current, vals, a, allowedActions)) return true;
    }

    return false;
}

async function processLineByLine(path: PathLike) {
    const fileStream = fs.createReadStream(path);

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity,
    });

    let part1 = 0;
    let part2 = 0;

    for await (const line of rl) {
        if (line === "") break;

        const nums = line.split(": ");
        const testValue = Number(nums[0]);

        const vals = nums[1].split(" ").map(Number);

        const part1Actions = [Action.Mul, Action.Add];
        const part2Actions = [...part1Actions, Action.Con];

        for (const [action, start] of [
            [Action.Add, 0],
            [Action.Mul, 1],
        ]) {
            if (isPossible(testValue, start, vals, action, part1Actions)) {
                part1 += testValue;
                part2 += testValue;
                break;
            }
            if (isPossible(testValue, start, vals, action, part2Actions)) {
                part2 += testValue;
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
