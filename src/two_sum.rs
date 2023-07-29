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

pub fn solution_a<N>(nums: N, target: i32) -> (i32, i32)
where
    N: IntoIterator<Item = i32>,
{
    (17, 18)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::TestCase;

    const TEST_CASES: [TestCase<([i32; 4], i32), (i32, i32)>; 3] = [
        TestCase {
            input: ([2i32, 7, 11, 15], 9i32),
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

    // let t: static = TestCase::new((vec![2, 7, 11, 15], 9), (0, 1));

    #[test]
    fn test_a() {
        for c in TEST_CASES.iter() {
            let (nums, target) = c.input();
            let expected = c.expected().clone();

            let res = solution_a(nums.clone(), target.clone());

            assert_eq!(res, expected.clone());
        }
    }
}
