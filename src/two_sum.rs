#![allow(dead_code)]
/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order.

Constraints:
2 <= nums.length <= 10^4
-10^9 <= nums[i] <= 10^9
-10^9 <= target <= 10^9
Only one valid answer exists.
*/

use std::collections::HashMap;

/*
Naive solution
T: O(N^2)
S: O(1)
*/
pub fn solution_a<N>(nums: N, target: i32) -> (i32, i32)
where
    N: IntoIterator<Item = i32> + Copy,
{
    for (i, x) in nums.into_iter().enumerate() {
        for (j, y) in nums.into_iter().enumerate() {
            if i == j {
                continue;
            }

            let sum = x + y;
            if sum == target {
                return (i as i32, j as i32);
            }
        }
    }

    (-1, -1)
}

/*
Linear solution
T: O(N)
S: O(N)
*/
pub fn solution_b<N>(nums: N, target: i32) -> (i32, i32)
where
    N: IntoIterator<Item = i32> + Copy,
{
    let mut dict: HashMap<i32, usize> = HashMap::new();
    for (j, x) in nums.into_iter().enumerate() {
        let diff = target - x;
        if let Some(i) = dict.get(&diff) {
            return (i.clone() as i32, j as i32);
        }

        dict.insert(x, j);
    }

    (-1, -1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::TestCase;

    const TEST_CASES: [TestCase<([i32; 4], i32), (i32, i32)>; 3] = [
        TestCase {
            input: ([2, 7, 11, 15], 9),
            expected: (0, 1),
        },
        TestCase {
            input: ([3, 2, 4, 0], 6),
            expected: (1, 2),
        },
        TestCase {
            input: ([3, 3, 0, 0], 6),
            expected: (0, 1),
        },
    ];

    #[test]
    fn a() {
        for c in TEST_CASES.iter() {
            let (nums, target) = c.input;
            let expected = c.expected;

            let res = solution_a(nums, target);

            assert_eq!(res, expected);
        }
    }

    #[test]
    fn b() {
        for c in TEST_CASES.iter() {
            let (nums, target) = c.input;
            let expected = c.expected;

            let res = solution_b(nums, target);

            assert_eq!(res, expected);
        }
    }
}
