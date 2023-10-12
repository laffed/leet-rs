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
Kadane's Algo
T: O(N)
S: O(1)
*/
pub fn solution_a(nums: Vec<i32>) -> i32 {
    let mut max_sub = i32::MIN;
    let mut curr_sub = i32::MIN;

    for n in nums.iter() {
        let curr_sub_plus_n = curr_sub.saturating_add(*n);
        // if curr_sub is negative, curr_sub_plus_n will always be < n
        // therefore max of n and curr_sub_plus_n is equivalent to "reset curr_sub if negative"
        curr_sub = (*n).max(curr_sub_plus_n);
        max_sub = max_sub.max(curr_sub);
    }

    max_sub
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
