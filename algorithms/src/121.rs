// Given an array *prices* where prices[i] is the price of a given stock on ith day, return the maximum profit that can be achieved by:
// Choosing a single day to buy one stock and choosing a different day in the future to sell the stock.
// If profit cannot be achieved, return 0.
// Constraints: 1 <= prices.length <= 10exp5 | 0 <= prices[i] <= 10exp4.

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX; // Initialized with i32::MAX to ensure the first stock price updates min_price.
    let mut max_profit = 0;

    for &price in &prices {
        min_price = min_price.min(price); // Using .min() to update min_price with the lesser of its current value and the encountered price.
        max_profit = max_profit.max(price - min_price); // Using .max() to update max_profit with the greater of its current value or the potential profit from selling at the current encountered price.
    }

    max_profit
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

        // Ascending prices, profit achievable by buying on day 1 and selling on the last day.
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);

        // Large dataset to test the upper bound.
        let large_data: Vec<i32> = (1..=100_000).collect();
        assert_eq!(max_profit(large_data), 99_999);

        // Multiple peaks after low point.
        assert_eq!(max_profit(vec![3, 2, 6, 5, 0, 8]), 8); // Buy at 0, sell at 8.

        // Duplicate prices.
        assert_eq!(max_profit(vec![3, 3, 5, 0, 0, 8, 8]), 8); // Buy at 0, sell at 8.

        // High initial value followed by much lower value.
        assert_eq!(max_profit(vec![1000, 1, 2, 3, 4, 5]), 4); // Buy at 1, sell at 5.

        // Alternating highs and lows.
        assert_eq!(max_profit(vec![5, 1, 5, 1, 5]), 4); // Buy at 1, sell at any instance of 5.

        // Constant price over time.
        assert_eq!(max_profit(vec![5, 5, 5, 5, 5]), 0); // No profit possible.

        // Duplicate optimal choices.
        assert_eq!(max_profit(vec![3, 1, 5, 1, 5]), 4); // Buy at either 1 and sell at either 5 subject to still being viable.
    }
}