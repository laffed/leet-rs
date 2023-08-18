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

/*
Naive approach
T: O(N^2)
S: O(1)
*/
pub fn solution_a(heights: Vec<u32>) -> u32 {
    let mut max_area = 0;
    'left: for (i, a) in heights.iter().enumerate() {
        for (j, b) in heights.iter().enumerate().rev() {
            if i == j {
                continue 'left;
            }
            // constraint enforces that 2^32 - 1 > usize (10^5)
            let i = i as u32;
            let j = j as u32;
            let area = (j - i) * a.min(b);
            max_area = max_area.max(area);
        }
    }
    max_area
}

/*
Sliding window
T: O(N)
S: O(1)
*/
pub fn solution_b(heights: Vec<u32>) -> u32 {
    let mut max_area = 0;
    let mut heights = heights.iter().enumerate();

    let mut left = heights.next();
    let mut right = heights.next_back();

    while let (Some((i, a)), Some((j, b))) = (left, right) {
        max_area = max_area.max(a.min(b) * (j - i) as u32);

        if a <= b {
            left = heights.next();
        } else {
            right = heights.next_back();
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_0() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(solution_a(input), 49);
    }

    #[test]
    fn a_1() {
        let input = vec![1, 1];
        assert_eq!(solution_a(input), 1);
    }

    #[test]
    fn b_0() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(solution_b(input), 49);
    }

    #[test]
    fn b_1() {
        let input = vec![1, 1];
        assert_eq!(solution_b(input), 1);
    }
}
