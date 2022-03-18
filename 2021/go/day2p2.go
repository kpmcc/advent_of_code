package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	filename := os.Args[1]
	f, err := os.Open(filename)
	if err != nil {
		_ = fmt.Errorf("error: %q", err)
	}
	horizontal := int64(0)
	aim := int64(0)
	depth := int64(0)
	directions := bufio.NewScanner(f)
	for directions.Scan() {
		t := directions.Text()
		d := strings.SplitN(t, " ", 2)
		direction := d[0]
		distance, _ := strconv.ParseInt(d[1], 10, 0)
		switch direction {
		case "forward":
			horizontal += distance
			depth += aim * distance
		case "up":
			aim -= distance
		case "down":
			aim += distance
		}
	}
	final_pos := horizontal * depth
	fmt.Printf("H: %d D: %d\n", horizontal, depth)
	fmt.Printf("Final: %d\n", final_pos)

}
