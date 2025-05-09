maxProfit(prices) {
    let maxProfit = 0;
    let [l, r] = [0, 1];
    while (r < prices.length) {
        if (prices[l] < prices[r]) {
            let currentProfit = prices[r] - prices[l];
            maxProfit = Math.max(currentProfit, maxProfit);
        } else {
            l = r
        }
        r += 1
    }
    return maxProfit
}

//TC: O(n)
//SC: O(1)
