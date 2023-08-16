#![allow(dead_code)]

/*
#217 Contains Duplicate
Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Constraints:

1 <= nums.length <= 10^5
-10^9 <= nums[i] <= 10^9
*/

use std::collections::HashSet;

/*
Naive solution
T: O(N^2)
S: O(1)
*/
pub fn solution_a<N>(nums: N) -> bool
where
    N: IntoIterator<Item = i32> + Copy,
{
    for (i, x) in nums.into_iter().enumerate() {
        for (j, y) in nums.into_iter().enumerate() {
            if i == j {
                continue;
            }
            if x == y {
                return true;
            }
        }
    }
    false
}

/*
Set solution
T: O(N)
S: O(N)
*/
pub fn solution_b<N>(nums: N) -> bool
where
    N: IntoIterator<Item = i32> + Copy,
{
    let mut set: HashSet<i32> = HashSet::new();

    for i in nums.into_iter() {
        if !set.insert(i) {
            return true;
        }
    }

    false
}

/*
Sorted solution in place
T: O(NlogN)
S: O(1)
*/
pub fn solution_c<N>(_nums: N) -> bool
where
    N: IntoIterator<Item = i32> + Copy,
{
    todo!();
}

/*
Sorted solution cloned
T: O(NlogN)
S: O(N) -> if unable to sort in place
*/
pub fn solution_d<N>(_nums: &N) -> bool
where
    N: IntoIterator<Item = i32> + Copy,
{
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::TestCase;

    const TC_0: TestCase<[i32; 4], bool> = TestCase {
        input: [1, 2, 3, 1],
        expected: true,
    };

    const TC_1: TestCase<[i32; 4], bool> = TestCase {
        input: [1, 2, 3, 4],
        expected: false,
    };

    const TC_2: TestCase<[i32; 10], bool> = TestCase {
        input: [1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
        expected: true,
    };

    #[test]
    fn a() {
        assert_eq!(solution_a(TC_0.input), TC_0.expected);
        assert_eq!(solution_a(TC_1.input), TC_1.expected);
        assert_eq!(solution_a(TC_2.input), TC_2.expected);
    }

    #[test]
    fn b() {
        assert_eq!(solution_b(TC_0.input), TC_0.expected);
        assert_eq!(solution_b(TC_1.input), TC_1.expected);
        assert_eq!(solution_b(TC_2.input), TC_2.expected);
    }

    // #[test]
    // fn c() {
    //     assert_eq!(solution_c(TC_0.input), TC_0.expected);
    //     assert_eq!(solution_c(TC_1.input), TC_1.expected);
    //     assert_eq!(solution_c(TC_2.input), TC_2.expected);
    // }
    //
    // #[test]
    // fn d() {
    //     assert_eq!(solution_d(&TC_0.input), TC_0.expected);
    //     assert_eq!(solution_d(&TC_1.input), TC_1.expected);
    //     assert_eq!(solution_d(&TC_2.input), TC_2.expected);
    // }
}
