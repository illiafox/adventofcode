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
	defer file.Close()

	var (
		left  []int
		right []int
	)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		s := strings.Split(line, "   ")

		first, err := strconv.Atoi(s[0])
		if err != nil {
			log.Fatal(err)
		}

		second, err := strconv.Atoi(s[1])
		if err != nil {
			log.Fatal(err)
		}

		left = append(left, first)
		right = append(right, second)
	}

	slices.Sort(left)
	slices.Sort(right)

	totalDistance := 0
	for i := 0; i < len(left); i++ {
		totalDistance += abs(left[i] - right[i])
	}
	fmt.Println("part 1: total distance", totalDistance)

	occurrences := make(map[int]int)
	for i := range right {
		occurrences[right[i]]++
	}

	similarityScore := 0

	for i := range left {
		similarityScore += left[i] * occurrences[left[i]]
	}

	fmt.Println("part 2: similarity score", similarityScore)
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
