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
		fmt.Fprintf(os.Stderr, "error: %q\n", err)
	}

	s := bufio.NewScanner(f)
	const numBits = 12
	var bitCounts [numBits]map[byte]int16
	for s.Scan() {
		t := s.Text()
		// Split text into strings of single bits
		digits := strings.SplitN(t, "", numBits)
		// Iterate over single bit strings
		for i, dStr := range digits {
			d := dStr[0] // Get the byte from the single bit string
			m := bitCounts[i]
			if m != nil {
				m[d]++
			} else {
				// Initial case
				bitCounts[i] = make(map[byte]int16)
				bitCounts[i][d] = 1
			}
		}
	}

	var gammaRateBits []byte
	var epsilonRateBits []byte

	for i := 0; i < numBits; i++ {
		m := bitCounts[i]
		zeros := m['0']
		ones := m['1']
		var mostCommonDigit byte
		var leastCommonDigit byte
		if zeros > ones {
			mostCommonDigit = '0'
			leastCommonDigit = '1'
		} else {
			mostCommonDigit = '1'
			leastCommonDigit = '0'
		}
		gammaRateBits = append(gammaRateBits, mostCommonDigit)
		epsilonRateBits = append(epsilonRateBits, leastCommonDigit)
	}

	gammaRateStr := string(gammaRateBits)
	epsilonRateStr := string(epsilonRateBits)

	gammaRate, _ := strconv.ParseInt(gammaRateStr, 2, 16)
	epsilonRate, _ := strconv.ParseInt(epsilonRateStr, 2, 16)

	powerConsumption := gammaRate * epsilonRate
	fmt.Printf("G: %d, E: %d, P: %d\n", gammaRate, epsilonRate, powerConsumption)
}
