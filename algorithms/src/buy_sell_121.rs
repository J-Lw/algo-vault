// Given an array *prices* where prices[i] is the price of a given stock on ith day, return the maximum profit that can be achieved by:
// Choosing a single day to buy one stock and choosing a different day in the future to sell the stock.
// If profit cannot be achieved, return 0.
// Constraints: 1 <= prices.length <= 10exp5 | 0 <= prices[i] <= 10exp4.

pub fn max_profit(prices: Vec<i32>) -> i32 {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        // Single day, no transaction possible.
        assert_eq!(max_profit(vec![8]), 0);

        // Descending prices, profit not achievable.
        assert_eq!(max_profit(vec![9, 8, 7, 6, 1]), 0);

        // Buy on day 2 (price = 1) and sell on day 5 price = 6, profit = 6-1 = 5.]
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}