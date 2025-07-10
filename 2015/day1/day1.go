package day1

import (
	"fmt"
	"log"
	"os"
)

func readFile(path string) string {
	data, err := os.ReadFile(path)
	if err != nil {
		log.Fatal("Error reading input file.")
	}
	return string(data)
}

func firstHalf(data string) int {
	floor := 0
	for _, ch := range data {
		switch ch {
		case '(':
			floor++
		case ')':
			floor--
		}
	}
	return floor
}

func secondHalf(data string) int {
	floor := 0
	for i, ch := range data {
		switch ch {
		case '(':
			floor++
		case ')':
			floor--
		}
		if floor == -1 {
			return i + 1
		}
	}
	return 0
}

func Run() {
	data := readFile("2015/day1/day1.txt")
	ans1 := firstHalf(data)
	ans2 := secondHalf(data)
	fmt.Println("First Half (Floor) : ", ans1)
	fmt.Println("Second Half (Position) :", ans2)
}
