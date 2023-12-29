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
        let mut max_area_through = vec![];
        for i in 0..heights.len() {
            if heights.get(i) == heights.get(i - 1) {
                continue;
            };
            let mut lb = i;
            let mut ub = i + 1;
            while lb > 0 && heights[lb - 1] >= heights[i] {
                lb -= 1;
            }
            while ub < heights.len() && heights[ub] >= heights[i] {
                ub += 1;
            }
            max_area_through.push((ub - lb) as i32 * heights[i]);
        }
        max_area_through.into_iter().max().unwrap() as i32
    }
}
// @lc code=end

#[test]
fn test() {
    let res = Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]);
    println!("{}", res);
}
