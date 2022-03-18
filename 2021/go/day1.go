package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open(os.Args[1])
	if err != nil {
		fmt.Fprintf(os.Stderr, "aoc1 %v", err)
	}
	scanner := bufio.NewScanner(f)

	// scan first depth
	scanner.Scan()
	previousDepth, _ := strconv.Atoi(scanner.Text())
	var depth_increase_count int

	for scanner.Scan() {
		depth, _ := strconv.Atoi(scanner.Text())
		if depth > previousDepth {
			depth_increase_count++
		}
		previousDepth = depth
	}
	fmt.Println(depth_increase_count)
}
