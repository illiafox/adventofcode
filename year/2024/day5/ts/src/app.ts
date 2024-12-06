import { PathLike } from "node:fs";

import fs from "fs";
import readline from "readline";

// https://en.wikipedia.org/wiki/Topological_sorting
function topologicalSort(appliedRules: number[][]) {
    const out: number[] = [];
    const edgesNum = new Map<number, number>();

    const graph = new Map<number, number[]>();
    for (const [x, y] of appliedRules) {
        if (!graph.has(x)) graph.set(x, []);
        if (!graph.has(y)) graph.set(y, []);

        graph.get(x)!.push(y);
        edgesNum.set(x, edgesNum.get(x) || 0);
        edgesNum.set(y, (edgesNum.get(y) || 0) + 1);
    }

    const withNoEdges: number[] = [];
    for (const [k, v] of edgesNum) {
        if (v === 0) {
            withNoEdges.push(k);
        }
    }

    while (withNoEdges.length >= 0) {
        const n = withNoEdges.shift();
        if (!n) break;
        out.push(n);

        for (const m of graph.get(n) || []) {
            edgesNum.set(m, edgesNum.get(m)! - 1);
            if (edgesNum.get(m) === 0) {
                withNoEdges.push(m);
            }
        }
    }

    if (out.length !== edgesNum.size) {
        console.log("cycle detected");
        return [];
    }

    return out;
}

function sort(update: number[], rules: Map<number, number[]>): number[] {
    const position = new Map<number, number>();

    update.forEach((element, idx) => {
        position.set(element, idx);
    });

    const appliedRules: number[][] = [];

    for (const up of update) {
        const rls = rules.get(up);
        if (rls === undefined) continue;

        for (const val of rls) {
            const v = position.get(val);
            const k = position.get(up);
            if (v !== undefined && k !== undefined) {
                appliedRules.push([up, val]);
            }
        }
    }

    update = topologicalSort(appliedRules);

    return update;
}

function checkUpdate(update: number[], rules: Map<number, number[]>): boolean {
    const position = new Map<number, number>();

    update.forEach((element, idx) => {
        position.set(element, idx);
    });

    for (const up of update) {
        const rls = rules.get(up);
        if (rls === undefined) continue;

        for (const val of rls) {
            const v = position.get(val);
            const k = position.get(up);
            if (v !== undefined && k !== undefined && v < k) {
                // console.log(`Rule violated: ${val} (${v}) must appear after ${up} (${k})`);
                return false;
            }
        }
    }

    return true;
}

async function processLineByLine(path: PathLike) {
    const fileStream = fs.createReadStream(path);

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity,
    });

    const rulesMap = new Map<number, number[]>();

    for await (const line of rl) {
        if (line === "") break;

        const nums = line.split("|").map(Number);
        const a = nums[0];
        const b = nums[1];

        let m = rulesMap.get(a);
        if (!m) {
            m = [];
        }
        m.push(b);
        rulesMap.set(a, m);
    }

    let part1 = 0;
    let part2 = 0;

    for await (const line of rl) {
        let update = line.split(",").map(Number);

        if (checkUpdate(update, rulesMap)) {
            // console.log('good', update)
            part1 += update[Math.floor(update.length / 2)];
        } else {
            update = sort(update, rulesMap);
            part2 += update[Math.floor(update.length / 2)];
        }
    }

    // const result = await processReports(rl);
    console.log("part 1:", part1);
    console.log("part 2:", part2);
    // console.log("part 2: tolerated safe reports:", result.toleratedSafeReports);
}

if (process.argv.length < 3) {
    console.error("one argument is expected");
    process.exit();
}

processLineByLine(process.argv[2]).catch((reason) => console.error(reason));
