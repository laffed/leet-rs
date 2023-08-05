#![allow(dead_code)]

/*
#11 Container With Most Water
You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.

ex.
Input: height = [1,8,6,2,5,4,8,3,7]
Output: 49
Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7].
In this case, the max area of water (blue section) the container can contain is 49.

Constraints:

n == height.length
2 <= n <= 10^5
0 <= height[i] <= 10^4
*/

pub fn solution_a(_height: Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(solution_a(input), 49);
    }

    #[test]
    fn b() {
        let input = vec![1, 1];
        assert_eq!(solution_a(input), 1);
    }
}
