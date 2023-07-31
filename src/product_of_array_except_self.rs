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

/*
T: O(N)
S: O(1)
*/
pub fn solution_a(nums: Vec<i32>) -> Vec<i32> {
    let mut contains_zero = NumZeroes::NoZeros;
    let mut product_without_zero = 1;
    let mut res = vec![];

    for n in nums.iter() {
        if *n == 0 {
            match contains_zero {
                NumZeroes::NoZeros => contains_zero = NumZeroes::OneZero,
                // more than one zero makes all values zero
                _ => return vec![0; nums.len()],
            }
        } else {
            product_without_zero *= n;
        }
    }

    for (i, n) in nums.iter().enumerate() {
        let v = match contains_zero {
            NumZeroes::NoZeros => product_without_zero / n,
            _ => {
                if *n == 0 {
                    product_without_zero
                } else {
                    0
                }
            }
        };

        res.insert(i, v);
    }

    res
}

enum NumZeroes {
    NoZeros,
    OneZero,
    MultipleZeroes,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_0() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];

        assert_eq!(solution_a(nums), expected);
    }

    #[test]
    fn a_1() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];

        assert_eq!(solution_a(nums), expected);
    }

    #[test]
    fn a_2() {
        let nums = vec![0, 1, 2, 3, 0];
        let expected = vec![0, 0, 0, 0, 0];

        assert_eq!(solution_a(nums), expected);
    }
}
