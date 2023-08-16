#![allow(dead_code)]

/*
#53 Maximum Subarray

Given an integer array nums, find the subarray
with the largest sum, and return its sum.

Constraints:
1 <= nums.length <= 10^5
-10^4 <= nums[i] <= 10^4

Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
*/

/*
*/
pub fn solution_a(_nums: Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_0() {
        let input = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(solution_a(input), 6);
    }
    #[test]
    fn a_1() {
        let input = vec![1];
        assert_eq!(solution_a(input), 1);
    }
    #[test]
    fn a_2() {
        let input = vec![5, 4, -1, 7, 8];
        assert_eq!(solution_a(input), 23);
    }
}
