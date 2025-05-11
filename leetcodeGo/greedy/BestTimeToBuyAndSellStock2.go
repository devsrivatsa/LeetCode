func maxProfit(prices []int) int {
    var maxProfit int
    l, r := 0, 1
    for {
        if r >= len(prices) {
            break
        }
        if prices[r] > prices[l] {
            maxProfit += prices[r] - prices[l]
        }
        r += 1
        l += 1
    }
    return maxProfit
}

//TC, SC = O(n)
