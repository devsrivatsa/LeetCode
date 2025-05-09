func maxProfit(prices []int) int {
    var maxProfit int
    l, r := 0, 1
    for {
        if r >= len(prices) { 
            break 
        }
        if prices[l] < prices[r] {
            currentProfit := prices[r] - prices[l]
            if currentProfit > maxProfit {
                maxProfit = currentProfit
            }
        } else {
            l = r
        }
        r += 1
    }
    return maxProfit    
}

//TC: O(n)
//SC: O(1)
