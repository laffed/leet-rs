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

/*
Naive solution
T: O(N^3)
S: O(N)
*/
pub fn solution_a(_nums: Vec<i32>) -> Vec<Vec<i32>> {
    vec![]
}

/*
T:
S:
*/
pub fn solution_b(_nums: Vec<i32>) -> Vec<Vec<i32>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_0() {
        assert_eq!(
            solution_a(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
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
