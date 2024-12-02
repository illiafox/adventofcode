package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(file)

	var (
		report               []int
		buf                  []int
		safeReports          int // part 1
		toleratedSafeReports int // part 2
	)

	for scanner.Scan() {
		report = report[:0]

		for _, v := range strings.Split(scanner.Text(), " ") {
			n, err := strconv.Atoi(v)
			if err != nil {
				log.Fatal(err)
			}

			report = append(report, n)
		}

		if reportSafe(report) {
			safeReports++
			toleratedSafeReports++
			continue
		}

		buf = slices.Grow(buf, len(report)-1)
		buf = buf[:len(report)-1]
		for i := range report {
			copy(buf, report[:i])
			copy(buf[i:], report[i+1:])

			if reportSafe(buf) {
				toleratedSafeReports++
				break
			}
		}
	}

	fmt.Println("part 1: safe reports:", safeReports)
	fmt.Println("part 2: tolerated safe reports:", toleratedSafeReports)
}

func reportSafe(reports []int) bool {
	decr := reports[0] > reports[1]

	for i := 1; i < len(reports); i++ {
		diff := reports[i] - reports[i-1]
		if diff == 0 || abs(diff) > 3 {
			return false
		}

		if (decr && diff > 0) || (!decr && diff < 0) {
			return false
		}
	}

	return true
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
