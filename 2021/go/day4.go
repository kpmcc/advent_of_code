package main

import (
		"bufio"
		"fmt"
		"io/ioutil"
		"os"
		"strings"
		"strconv"
)

type Board struct {
	tiles [5][5]bool
	vals map[uint][2]uint
}

func (b *Board) calculateScore() uint {
	var sum uint
	for k, v := range b.vals {
		if b.tiles[v[0]][v[1]] != true {
			sum += k
		}
	}
	return sum
}

func (b *Board) hasWinCondition() bool {
	for i := 0; i < 5; i++ {
		rowCondition := true
		for j := 0; j < 5; j++ {
			rowCondition = rowCondition && b.tiles[i][j]
		}
		if rowCondition {
			return true
		}
	}
	for j := 0; j < 5; j++ {
		colCondition := true
		for i := 0; i < 5; i++ {
			colCondition = colCondition && b.tiles[i][j]
		}
		if colCondition {
			return true
		}
	}
	return false
}

func (b *Board) check (x uint) bool {
	v, ok := b.vals[x]
	if ok {
		b.tiles[v[0]][v[1]] = true
	}
	return ok
}

func (b *Board) print() {
	for _, r := range b.tiles {
		for _, c := range r {
			fmt.Printf("%d ", c)
		}
		fmt.Printf("\n")
	}
}

func playBingo(vals []uint, boards []*Board) uint {
	for i, v := range vals {
		for _, b := range boards {
			b.check(v)
			if b.hasWinCondition() {
				fmt.Printf("found winner?\n")
				b.print()
				fmt.Printf("vals: %v\n", vals[:i])

				boardTally := b.calculateScore()
				score := boardTally * v
				return score
			}
		}
	}
	return 0
}

func main() {
	//var myBoard Board
	var myBoard Board
	myBoard.vals = make(map[uint][2]uint)
	fmt.Printf("myboard wins: %t\n", myBoard.hasWinCondition())
	myBoard.tiles[0][0] = true
	myBoard.tiles[1][0] = true
	myBoard.tiles[2][0] = true
	myBoard.tiles[3][0] = true
	myBoard.tiles[4][0] = true
	myBoard.vals[23] = [2]uint{0, 0}
	ok := myBoard.check(23)
	fmt.Printf("myboard has 23: %t\n", ok)
	fmt.Printf("myboard wins: %t\n", myBoard.hasWinCondition())
	boardFile := "/Users/kale/advent_of_code/2021/input/04_boards.txt"
	valsFile := "/Users/kale/advent_of_code/2021/input/04_vals.txt"
	f, err := os.Open(valsFile)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error opening %s: %v\n", valsFile, err)
		return
	}
	input := bufio.NewScanner(f)
	vals := []uint{}
	for input.Scan() {
		strVals := strings.Split(input.Text(), ",")
		for _, s := range(strVals) {
			x, err := strconv.ParseInt(s, 10, 32)
			if err != nil {
				fmt.Fprintf(os.Stderr, "error parsing int - 83: %v", err)
				return
			}
			vals = append(vals, uint(x))
		}
	}

	boardContents, err := ioutil.ReadFile(boardFile)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error opening %s: %v\n", boardFile, err)
		return
	}

	boarditems := strings.Split(string(boardContents), "\n\n")
	var boards []*Board
	for _, bi := range boarditems {
		newBoard := new(Board)
		newBoard.vals = make(map[uint][2]uint)
		lines := strings.Split(bi, "\n")
		for lineNum, l := range lines {
			if l == "" {
				continue
			}
			rawNums := strings.Split(l, " ")
			nums := []uint{}
			for _, n := range rawNums {
				if n == " " || n == "" {
					continue
				}
				x, err := strconv.ParseInt(n, 10, 32)
				if err != nil {
					fmt.Fprintf(os.Stderr, "error parsing int - 107: %v\n", err)
					fmt.Fprintf(os.Stderr, "l is %s\n", l)
					return
				}
				nums = append(nums, uint(x))
			}
			if len(nums) != 5 {
				fmt.Fprintf(os.Stderr, "wrong number of nums: %d\n", len(nums))
				return
			}
			for colNum, n := range nums {
				newBoard.vals[n] = [2]uint{uint(lineNum), uint(colNum)}
			}

		}
		boards = append(boards, newBoard)
	}

	fmt.Printf("num boards: %d\n", len(boards))

	score := playBingo(vals, boards)
	fmt.Printf("score is %d\n", score)
}
