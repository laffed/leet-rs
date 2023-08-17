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

/*
* T: O(S * N)
* S: O(S)
* where:
*   S -> amount
*   N -> coins.len()
*
* Note: for idiomaticity, solution returns None instead of -1
*/
pub fn solution(coins: Vec<u32>, amount: u32) -> Option<u32> {
    dfs(&coins, Some(amount))
}

fn dfs(coins: &Vec<u32>, rem: Option<u32>) -> Option<u32> {
    let rem = match rem {
        None => {
            return None;
        }
        Some(0) => return Some(0),
        Some(n) => n,
    };

    let mut min_cost = u32::MAX;
    for coin in coins.iter() {
        let next_rem = rem.checked_sub(*coin);
        let res = dfs(&coins, next_rem);
        if let Some(r) = res {
            min_cost = min_cost.min(r + 1);
        }
    }

    if min_cost == u32::MAX {
        return None;
    }

    Some(min_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_0() {
        assert_eq!(solution(vec![1, 2, 5], 11), Some(3));
    }

    #[test]
    fn a_1() {
        assert_eq!(solution(vec![2], 3), None);
    }

    #[test]
    fn a_2() {
        assert_eq!(solution(vec![1], 0), Some(0));
    }
}
