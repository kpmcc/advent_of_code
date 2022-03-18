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
	vertical := int64(0)
	directions := bufio.NewScanner(f)
	for directions.Scan() {
		t := directions.Text()
		d := strings.SplitN(t, " ", 2)
		direction := d[0]
		distance, _ := strconv.ParseInt(d[1], 10, 0)
		switch direction {
		case "forward":
			horizontal += distance
		case "up":
			vertical -= distance
		case "down":
			vertical += distance
		}
	}
	final_pos := horizontal * vertical
	fmt.Printf("H: %d V: %d\n", horizontal, vertical)
	fmt.Printf("Final: %d\n", final_pos)

}
