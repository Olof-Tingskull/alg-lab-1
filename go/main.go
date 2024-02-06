package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"time"

	"gonum.org/v1/plot"
	"gonum.org/v1/plot/plotter"
	"gonum.org/v1/plot/vg"
)

// greadyMinCoinsRecursive finds the minimum number of coins needed to make the total value.
// The memory parameter is now a pointer, making it optional.
func greadyMinCoinsRecursive(totalValue int, currencies []int, memory *[]int, depth int) int {
	depthString := ""
	for i := 0; i < depth; i++ {
		depthString += " | "
	}
	fmt.Println(depthString+"running:", totalValue)

	if memory != nil && (*memory)[totalValue] != 0 {
		minCoins := (*memory)[totalValue]
		fmt.Println(depthString+"memory:", minCoins)
		return minCoins
	}

	if totalValue == 0 {
		return 0
	}

	minCoins := totalValue
	for _, currency := range currencies {
		if totalValue == currency {
			minCoins = 1
			break
		} else if totalValue > currency {
			fmt.Printf(depthString+"subtracted: %d\n", currency)
			currentMin := greadyMinCoinsRecursive(totalValue-currency, currencies, memory, depth+1) + 1
			fmt.Printf(depthString+"min-coins: %d\n", currentMin)
			if currentMin < minCoins {
				minCoins = currentMin
			}
		}
	}

	if memory != nil {
		(*memory)[totalValue] = minCoins
	}

	fmt.Println(depthString+"returning:", minCoins)

	return minCoins
}

func greadyMinCoins(totalValue int, currencies []int) int {
	minCoinsForValue := make([]int, totalValue+1)

	for currentValue := 1; currentValue <= totalValue; currentValue++ {
		currentMinCoins := int(^uint(0) >> 1) // Max int value

		for _, currency := range currencies {
			if currentValue >= currency {
				current := minCoinsForValue[currentValue-currency] + 1
				if current < currentMinCoins {
					currentMinCoins = current
				}
			}
		}

		minCoinsForValue[currentValue] = currentMinCoins
	}

	return minCoinsForValue[totalValue]
}

func timeFunction[V any, I ~[]V](f func(V), iter I) []struct {
	Value    V
	Duration time.Duration
} {
	var times []struct {
		Value    V
		Duration time.Duration
	}

	for _, i := range iter {
		start := time.Now()
		f(i)
		duration := time.Since(start)
		times = append(times, struct {
			Value    V
			Duration time.Duration
		}{i, duration})
	}

	return times
}

func plpotPoints(points plotter.XYs) error {
	p := plot.New()

	p.Title.Text = "Plot of Points"
	p.X.Label.Text = "X"
	p.Y.Label.Text = "Y"

	line, err := plotter.NewLine(points)
	if err != nil {
		return err
	}
	p.Add(line)

	// Save the plot to a PNG file
	if err := p.Save(4*vg.Inch, 4*vg.Inch, "points_plot.png"); err != nil {
		return err
	}

	return nil
}

func kattis() {
	scanner := bufio.NewScanner(os.Stdin)

	scanner.Scan()
	n, _ := strconv.Atoi(scanner.Text())

	scanner.Scan()
	a, _ := strconv.Atoi(scanner.Text())

	scanner.Scan()
	b, _ := strconv.Atoi(scanner.Text())

	scanner.Scan()
	c, _ := strconv.Atoi(scanner.Text())

	fmt.Println(greadyMinCoins(n, []int{a, b, c}))
}

func main() {
	memory := make([]int, 16)
	greadyMinCoinsRecursive(15, []int{7, 6, 5}, &memory, 0)
}
