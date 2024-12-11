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

export async function countAllStones(
    nums: number[],
    blinks: number,
): Promise<number> {
    const promises = nums.map((n) => countStones(n, blinks));
    const results = await Promise.all(promises);
    return results.reduce((acc, val) => acc + val, 0);
}
