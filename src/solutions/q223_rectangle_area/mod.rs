/*
 * @lc app=leetcode id=223 lang=rust
 *
 * [223] Rectangle Area
 */

use super::Solution;
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
// @lc code=start
impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let range_x1 = (ax1, ax2);
        let range_y1 = (ay1, ay2);

        let range_x2 = (bx1, bx2);
        let range_y2 = (by1, by2);

        let larger_range = |range1: (i32, i32), range2: (i32, i32)| -> ((i32, i32), (i32, i32)) {
            match range1.1 > range2.1 {
                true => (range1, range2),
                false => (range2, range1),
            }
        };

        let lower_range = |range1: (i32, i32), range2: (i32, i32)| -> ((i32, i32), (i32, i32)) {
            match range1.0 < range2.0 {
                true => (range1, range2),
                false => (range2, range1),
            }
        };

        let collapse_range = |range1: &(i32, i32), range2: &(i32, i32)| -> i32 {
            let (hi_ub, lo_ub) = larger_range(*range1, *range2);
            let (lo_lb, _) = lower_range(*range1, *range2);
            if hi_ub == lo_lb {
                lo_ub.1 - lo_ub.0
            } else {
                (lo_ub.1 - hi_ub.0).max(0)
            }
        };

        let area_sum = (range_x1.1 - range_x1.0) * (range_y1.1 - range_y1.0)
            + (range_x2.1 - range_x2.0) * (range_y2.1 - range_y2.0);

        let collapse_x = collapse_range(&range_x1, &range_x2);
        let collapse_y = collapse_range(&range_y1, &range_y2);

        let collapse_area = collapse_x * collapse_y;

        area_sum - collapse_area
    }
}
// @lc code=end

#[test]
fn test() {
    let (ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) = (-2, -2, 2, 2, -2, -2, 2, 2);
    let result = Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
    println!("{result}");
}
