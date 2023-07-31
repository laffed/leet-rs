#![allow(dead_code)]

/*
#238 Product of Array Except Self

Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
You must write an algorithm that runs in O(n) time and without using the division operation.

Constraints:
2 <= nums.length <= 10^5
-30 <= nums[i] <= 30
The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)
*/

pub fn solution_a(nums: Vec<i32>) -> Vec<i32> {
    vec![0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_0() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![1, 2, 3, 4];

        assert_eq!(solution_a(nums), expected);
    }

    #[test]
    fn a_1() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];

        assert_eq!(solution_a(nums), expected);
    }
}
