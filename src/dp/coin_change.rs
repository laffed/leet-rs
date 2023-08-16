#![allow(dead_code)]

/*
#322 Coin Change
You are given an integer array coins representing coins of different denominations
and an integer amount representing a total amount of money.
Return the fewest number of coins that you need to make up that amount.
If that amount of money cannot be made up by any combination of the coins, return -1.
You may assume that you have an infinite number of each kind of coin.

Constraints:
1 <= coins.length <= 12
1 <= coins[i] <= 2^31 - 1
0 <= amount <= 10^4
*/

pub fn solution(_coins: Vec<u32>, _amount: u32) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_0() {
        assert_eq!(solution(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn a_1() {
        assert_eq!(solution(vec![2], 3), -1);
    }

    #[test]
    fn a_2() {
        assert_eq!(solution(vec![1], 0), 0);
    }
}
