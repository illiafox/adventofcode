import { describe, expect, it } from "@jest/globals";

import { countAllStones } from "./blink";

const testData = [30, 71441, 3784, 580926, 2, 8122942, 0, 291];

describe("count stones after blinks", () => {
    it("part 1: 25 blinks", async () => {
        const part1 = await countAllStones(testData, 25);
        expect(part1).toBe(191690);
    });
    it("part 2: 75 blinks", async () => {
        const part2 = await countAllStones(testData, 75);
        expect(part2).toBe(228651922369703);
    });
});
