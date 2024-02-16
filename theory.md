# Teori Questions

1. *Simulate what the solution to 1(a) will do on the input n=15, a=5, b=6, c=7: draw the tree of recursive calls that the recurrence gives rise to.*

```
running: 15
subtracted: 7
 | running: 8
 | subtracted: 7
 |  | running: 1
 |  | returning: 1
 | min-coins: 2
 | subtracted: 6
 |  | running: 2
 |  | returning: 2
 | min-coins: 3
 | subtracted: 5
 |  | running: 3
 |  | returning: 3
 | min-coins: 4
 | returning: 2
min-coins: 3
subtracted: 6
 | running: 9
 | subtracted: 7
 |  | running: 2
 |  | returning: 2
 | min-coins: 3
 | subtracted: 6
 |  | running: 3
 |  | returning: 3
 | min-coins: 4
 | subtracted: 5
 |  | running: 4
 |  | returning: 4
 | min-coins: 5
 | returning: 3
min-coins: 4
subtracted: 5
 | running: 10
 | subtracted: 7
 |  | running: 3
 |  | returning: 3
 | min-coins: 4
 | subtracted: 6
 |  | running: 4
 |  | returning: 4
 | min-coins: 5
 | subtracted: 5
 |  | running: 5
 |  | returning: 1
 | min-coins: 2
 | returning: 2
min-coins: 3
returning: 3
```

2. *How will the recursive calls made in the memoized version in 1(c) differ from in this case (n=15, a=5, b=6, c=7)?*

```
running: 15
subtracted: 7
 | running: 8
 | subtracted: 7
 |  | running: 1
 |  | returning: 1
 | min-coins: 2
 | subtracted: 6
 |  | running: 2
 |  | returning: 2
 | min-coins: 3
 | subtracted: 5
 |  | running: 3
 |  | returning: 3
 | min-coins: 4
 | returning: 2
min-coins: 3
subtracted: 6
 | running: 9
 | subtracted: 7
 |  | running: 2
 |  | memory: 2
 | min-coins: 3
 | subtracted: 6
 |  | running: 3
 |  | memory: 3
 | min-coins: 4
 | subtracted: 5
 |  | running: 4
 |  | returning: 4
 | min-coins: 5
 | returning: 3
min-coins: 4
subtracted: 5
 | running: 10
 | subtracted: 7
 |  | running: 3
 |  | memory: 3
 | min-coins: 4
 | subtracted: 6
 |  | running: 4
 |  | memory: 4
 | min-coins: 5
 | subtracted: 5
 |  | running: 5
 |  | returning: 1
 | min-coins: 2
 | returning: 2
min-coins: 3
returning: 3
```

3. *Study the recurrence Coins(n) in problem 1. Why is this a correct recurrence that solves the problem?*
The Coins(n) recurrence correctly solves the coin change problem because it explores all possible combinations of using the three types of coins to make up the amount n. It «checks the minimum number of coins needed by considering each coin type and reducing the problem to a smaller subproblem (n-a, n-b, and n-c). The base case ensures correct termination, and the recurrence relation correctly accumulates the minimum number of coins needed.

4. *Suppose we were trying to solve a variant of the coin change problem where we only had silver, gold and platinum coins at our disposal, no copper coins.  How would the recurrence describing the answer to the problem change in this variant?*
It would work as wells as the alrothim returns inf when n < 0. So when n reaches a values which is less than all currencies, silver, gold and platinum, the alorithim would return inf and would not be chosen as an option when min() is applied.

5. *Study the function f(x, y) defined in subproblem 2(a). What does the value of f(x, y) represent, in words? Argue why computing f(n, k) gives a solution to the problem.*

The function f(x, y) represents the probability of having a winning streak of exactly y games in a series after x games have been played, implying if the streak is greater than 0, y > 0, the last game have to been won. 

6. *Of the five different programs that you will write (in problems 1(a), 1(c), 1(e), 2(a), and 2(c)), which ones will have exponential time complexity, and which ones will have polynomial time complexity?*

Exponential Time Complexity: 1(a) and 2(a) because of the nature of recursive calls without memoization or dynamic programming.
Polynomial Time Complexity: 1(c), 1(e), and 2(c) because of the use of memoization (1c, 2c) and the iterative bottom-up approach (1e), which significantly reduce the number of calculations.

7. *Of the ones with polynomial time complexity, which ones will have linear time complexity?*

Both 1(c) and 1(e) have linear time complexity. While 1(c) is a memoized version and 1(e) is a bottom-up dynamic programming implementation, they essentially perform the same computations. The bottom-up approach in 1(e) clearly indicates a linear time complexity due to its straightforward for-loop structure that computes each value exactly once. Despite the overhead of checking the memoization table in 1(c), this does not change the overall linear complexity of the algorithm. For 2(c), the presence of the single for-loop and the manner in which computations are performed also indicate a linear time complexity. This is because each value is computed exactly once in a sequential manner, thus making the algorithm linear with respect to the input size.