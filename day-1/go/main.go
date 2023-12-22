package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	search := map[int][]string{
		1: {"1", "one"},
		2: {"2", "two"},
		3: {"3", "three"},
		4: {"4", "four"},
		5: {"5", "five"},
		6: {"6", "six"},
		7: {"7", "seven"},
		8: {"8", "eight"},
		9: {"9", "nine"},
	}

	oneStarResult := 0
	twoStarResult := 0

	file, err := os.Open("../input.txt")
	if err != nil {
		log.Fatal(err)
	}

	reader := bufio.NewReader(file)

	for {
		line, _, err := reader.ReadLine()
		oneStarMap := make(map[int]int)
		twoStarMap := make(map[int]int)

		if len(line) > 0 {
			for key, value := range search {
				for searchKey, searchValue := range value {

					first := strings.Index(string(line), searchValue)
					last := strings.LastIndex(string(line), searchValue)

					if first != -1 {
						if searchKey == 0 {
							oneStarMap[first] = key
						}
						twoStarMap[first] = key
					}

					if last != -1 {
						if searchKey == 0 {
							oneStarMap[last] = key
						}
						twoStarMap[last] = key
					}
				}
			}

		}
		if err != nil {
			break
		}

		oneStarTotal := getTotal(oneStarMap)
		oneStarResult += oneStarTotal

		twoStarTotal := getTotal(twoStarMap)
		twoStarResult += twoStarTotal
	}

	fmt.Println("1 Star Result:", oneStarResult)
	fmt.Println("2 Star Result:", twoStarResult)
}

func getTotal(starMap map[int]int) int {
	sum := 0

	if len(starMap) > 0 {
		keys := []int{}

		for key := range starMap {
			keys = append(keys, key)
		}

		sort.Ints(keys)

		firstStr := strconv.Itoa(starMap[keys[0]])
		lastStr := strconv.Itoa(starMap[keys[len(keys)-1]])
		strConcat := firstStr + lastStr

		newSum, err := strconv.Atoi(strConcat)
		if err != nil {
			panic(err)
		}

		sum = newSum
	}

	return sum
}
