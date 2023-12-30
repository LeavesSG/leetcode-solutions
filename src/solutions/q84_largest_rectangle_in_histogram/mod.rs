/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 */

use super::Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        let max = *heights.iter().max().unwrap();
        let mut lb_with_same_height = (0..len).into_iter().collect::<Vec<_>>();
        let mut ub_with_same_height = lb_with_same_height.clone();
        for i in 0..len {
            let val = heights[i];
            if (val * len as i32) < max {
                continue;
            }
            let mut left = i;
            while left > 0 && heights[left - 1] >= val {
                left = lb_with_same_height[left - 1];
            }
            lb_with_same_height[i] = left;
        }
        for i in 0..len {
            let j = len - 1 - i;
            let val = heights[j];
            let mut right = j;
            while right < len - 1 && heights[right + 1] >= val {
                right = ub_with_same_height[right + 1];
            }
            ub_with_same_height[j] = right;
        }
        let mut max_area: Vec<i32> = Vec::with_capacity(len);
        for i in 0..len {
            let len = ub_with_same_height[i] - lb_with_same_height[i] + 1;
            max_area.push(len as i32 * heights[i]);
        }

        max_area.into_iter().max().unwrap()
    }
}
// @lc code=end

#[test]
fn test() {
    let res = Solution::largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0]);
    println!("{}", res);
}
