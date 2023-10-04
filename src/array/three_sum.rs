#![allow(dead_code)]

/*
#15 3Sum

Given an integer array nums, return all the triplets
[nums[i], nums[j], nums[k]] such that
i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

Notice that the solution set must not contain duplicate triplets.

Constraints:
3 <= nums.length <= 3000
-10^5 <= nums[i] <= 10^5
*/

use std::collections::HashSet;

/*
Two Pointer Solution
T: O(N^2 + NLogN) -> O(N^2)
S: O(LogN)
*/
pub fn solution_a(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut nums = nums.clone();
    nums.sort();

    for (i, n) in nums.iter().enumerate() {
        if *n > 0 {
            break;
        }
        if i == 0 || nums[i - 1] != nums[i] {
            two_sum_ii(&nums, i, &mut res);
        }
    }

    res
}

fn two_sum_ii(nums: &Vec<i32>, i: usize, res: &mut Vec<Vec<i32>>) {
    let mut lo = i;
    let mut hi = nums.len() - 1;

    while lo < hi {
        let sum = nums[i] + nums[lo] + nums[hi];
        if sum < 0 {
            lo += 1;
        } else if sum > 0 {
            hi -= 1;
        } else {
            res.push(vec![nums[i], nums[lo], nums[hi]]);
            lo += 1;
            hi -= 1;
            while lo < hi && nums[lo] == nums[lo - 1] {
                lo += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_0() {
        let right = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(solution_a(vec![-1, 0, 1, 2, -1, -4]), right);
    }

    #[test]
    fn a_1() {
        let right: Vec<Vec<i32>> = vec![];
        assert_eq!(solution_a(vec![0, 1, 1]), right);
    }

    #[test]
    fn a_2() {
        assert_eq!(solution_a(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
