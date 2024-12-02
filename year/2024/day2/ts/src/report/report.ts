export async function processReports(rl: AsyncIterable<string>): Promise<{
  safeReports: number;
  toleratedSafeReports: number;
}> {
  let safeReports = 0; // part 1
  let toleratedSafeReports = 0; // part 2

  for await (const line of rl) {
    const report = line.split(" ").map(Number);

    if (isReportSafe(report)) {
      safeReports++;
      toleratedSafeReports++;
      continue;
    }

    for (let i = 0; i < report.length; i++) {
      const toleratedReport = [...report.slice(0, i), ...report.slice(i + 1)];
      if (isReportSafe(toleratedReport)) {
        toleratedSafeReports++;
        break;
      }
    }
  }

  return {
    safeReports,
    toleratedSafeReports,
  };
}

function isReportSafe(report: readonly number[]): boolean {
  const order = report[0] > report[1]; // true if decreasing, false if increasing

  for (let i = 1; i < report.length; i++) {
    const diff = report[i] - report[i - 1];

    if (diff === 0 || Math.abs(diff) > 3 || (order ? diff > 0 : diff < 0)) {
      return false;
    }
  }

  return true;
}
