package main

import (
		"bufio"
		"fmt"
		"os"
		"strconv"
)

type Node struct {
	Left * Node
	Right * Node
	NumChildren uint
}

func (n *Node) insert(v []rune) (*Node, error) {
	currNode := n
	for _, x := range v {
		switch x {
			case '0':
				currNode.NumChildren += 1
				if currNode.Left == nil {
					currNode.Left = new(Node)
				}
				currNode = currNode.Left
			case '1':
				currNode.NumChildren += 1
				if currNode.Right == nil {
					currNode.Right = new(Node)
				}
				currNode = currNode.Right
			default:
				return nil, fmt.Errorf("invalid value")
		}
	}
	return currNode, nil
}


func main() {
	fmt.Printf("aoc day3 pt2\n")
	var root Node
	f, err := os.Open("/Users/kale/advent_of_code/2021/input/03.txt")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Could not open file")
		return
	}
	input := bufio.NewScanner(f)
	for input.Scan() {
		t := input.Text()
		r := []rune(t)
		root.insert(r)
	}

	
	// Seeking more common
	vals := ""
	n := &root
	for n != nil {
		l := n.Left
		r := n.Right
		if l == nil && r == nil {
			break
		}
		if l == nil {
			// take right
			vals += "1"
			n = r
			continue
		}
		if r == nil {
			// take left 
			vals += "0"
			n = l
			continue
		}
		if r.NumChildren >= l.NumChildren {
			vals += "1"
			n = r
		} else {
			vals += "0"
			n = l
		}
	}
	fmt.Printf("Oxygen generator rating: %s\n", vals)
	o2gen, err := strconv.ParseInt(vals, 2, 32)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error parsing int: %v", err)
		return
	}

	vals = ""
	n = &root
	for n != nil {
		l := n.Left
		r := n.Right
		if l == nil && r == nil {
			break
		}
		if l == nil {
			// take right
			vals += "1"
			n = r
			continue
		}
		if r == nil {
			// take left 
			vals += "0"
			n = l
			continue
		}
		if l.NumChildren <= r.NumChildren {
			vals += "0"
			n = l
		} else {
			vals += "1"
			n = r
		}
	}
	fmt.Printf("CO2 scrubber rating: %s\n", vals)
	co2scrub, err := strconv.ParseInt(vals, 2, 32)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error parsing int: %v", err)
		return
	}

	life_support_rating := o2gen * co2scrub
	fmt.Printf("Life support rating: %d\n", life_support_rating)


	//fmt.Printf("root has %d children\n", root.NumChildren)
	//var a = []rune{'0', '1', '1'}
	//var b = []rune{'1', '0', '0'}
	//var c = []rune{'0', '0', '0'}
	//root.insert(a)
	//root.insert(b)
	//root.insert(c)
	fmt.Printf("root has %d children\n", root.NumChildren)
	fmt.Printf("root:left has %d children\n", root.Left.NumChildren)
	fmt.Printf("root:right has %d children\n", root.Right.NumChildren)
	//fmt.Printf("root:left:left has %d children\n", root.Left.Left.NumChildren)
	//fmt.Printf("root:left:right has %d children\n", root.Left.Right.NumChildren)
}
