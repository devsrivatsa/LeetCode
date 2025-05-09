use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let (mut l, mut r) = (0, 0);
        while r < prices.len() {
            if prices[l] < prices[r] { //since i32 implements Copy, there is no need for &prices[idx]
                let mut current_profit = prices[r]-prices[l];
                max_profit = cmp::max(current_profit, max_profit);
            } else {
                l = r; //since i32 implements Copy, there is no need for l = r.clone();
            }
            r += 1;
        }
        max_profit
    }
}

//TC: O(n)
//SC: O(1)
