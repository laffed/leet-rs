#![allow(dead_code)]

/*
#121 Best Time to Buy and Sell Stock
You are given an array prices where prices[i] is the price of a given stock on the ith day.
You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

Constraints:
1 <= prices.length <= 10^5
0 <= prices[i] <= 10^4
*/

/*
T: O(N)
S: O(1)
*/
pub fn solution<P>(prices: P) -> i32
where
    P: IntoIterator<Item = i32> + Copy,
{
    let mut profit = 0;
    // opinion: slightly more elegant than setting to el[0] and starting loop at 1
    let mut current_min = i32::MAX;
    for i in prices.into_iter() {
        profit = profit.max(i - current_min);
        current_min = current_min.min(i);
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::TestCase;

    const TEST_CASES: [TestCase<[i32; 6], i32>; 3] = [
        TestCase {
            input: [7, 1, 5, 3, 6, 4],
            expected: 5,
        },
        TestCase {
            input: [7, 6, 4, 3, 1, 0],
            expected: 0,
        },
        TestCase {
            input: [7, 2, 6, 3, 1, 3],
            expected: 4,
        },
    ];

    #[test]
    fn a() {
        for c in TEST_CASES.iter() {
            assert_eq!(solution(c.input), c.expected);
        }
    }
}
