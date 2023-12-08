package main

import (
	"fmt"
	"os"
	"slices"
	"sort"
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
	jokers := 0
	instsMap := make(map[string]int)

	for _, card := range cards {
		if card == "J" {
			jokers++
			continue
		}
		instsMap[card]++
	}

	var insts []int
	for _, v := range instsMap {
		insts = append(insts, v)
	}
	sort.Sort(sort.Reverse(sort.IntSlice(insts)))

	if jokers == 5 {
		return 6
	}

	if anyIsLen(insts, []int{5}, jokers) {
		return 6
	} else if anyIsLen(insts, []int{4}, jokers) {
		return 5
	} else if anyIsLen(insts, []int{3, 2}, jokers) {
		return 4
	} else if anyIsLen(insts, []int{3}, jokers) {
		return 3
	} else if anyIsLen(insts, []int{2, 2}, jokers) {
		return 2
	} else if anyIsLen(insts, []int{2}, jokers) {
		return 1
	}
	return 0
}

func anyIsLen(insts []int, lens []int, jokers int) bool {
	jokersLeft := jokers
	for _, l := range lens {
		for _, v := range insts {
			if v == l {
				break
			}
			if v < l && v+jokersLeft >= l {
				jokersLeft -= l - v
				break
			}
			return false
		}
	}
	return true
}

const Strengths = "J23456789TQKA"

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
