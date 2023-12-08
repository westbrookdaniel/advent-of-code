package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type hand struct {
	cards []string
	bid   int
	rank  int
}

func main() {
	data, err := os.ReadFile("input.txt")
	check(err)
	input := string(data)
	input = strings.TrimSpace(input)

	lines := strings.Split(input, "\n")

	hands := make([]hand, 0)
	for _, line := range lines {
		line := strings.TrimSpace(line)

		parts := strings.Split(line, " ")
		cards := strings.Split(parts[0], "")
		bid, err := strconv.Atoi(parts[1])
		check(err)

		rank := getRank(cards)

		hands = append(hands, hand{cards, bid, rank})
	}

	slices.SortFunc(hands, func(a, b hand) int {
		if a.rank == b.rank {
			return compareCards(a.cards, b.cards)
		}
		if a.rank > b.rank {
			return 1
		}
		return -1
	})

	out := 0
	for i, h := range hands {
		out += h.bid * (i + 1)
	}

	fmt.Println(out)
}

func getRank(cards []string) int {
	insts := make(map[string]int)

	for _, card := range cards {
		insts[card]++
	}

	if len(insts) == 1 {
		return 6
	} else if anyIsLen(insts, 4) {
		return 5
	} else if anyIsLen(insts, 3) && anyIsLen(insts, 2) {
		return 4
	} else if anyIsLen(insts, 3) {
		return 3
	} else if len(insts) == 3 && anyIsLen(insts, 2) {
		return 2
	} else if anyIsLen(insts, 2) {
		return 1
	}

	return 0
}

func anyIsLen(insts map[string]int, i int) bool {
	for _, v := range insts {
		if v == i {
			return true
		}
	}
	return false
}

const Strengths = "23456789TJQKA"

func compareCards(a []string, b []string) int {
	for i := 0; i < len(a); i++ {
		if a[i] == b[i] {
			continue
		}
		iA := strings.Index(Strengths, a[i])
		iB := strings.Index(Strengths, b[i])
		if iA > iB {
			return 1
		}
		return -1
	}
	return 0
}
