/*
 * @lc app=leetcode id=593 lang=rust
 *
 * [593] Valid Square
 */

use super::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let anchor = p1.clone();
        let mut map = std::collections::HashMap::new();
        let mut points = [p1, p2, p3, p4];
        points.iter().for_each(|p| {
            map.insert((p[0], p[1]), true);
        });
        if map.len() != 4 {
            return false;
        }

        let get_distance = |p1: &Vec<i32>, p2: &Vec<i32>| {
            (((p2[0] - p1[0]).pow(2) + (p2[1] - p1[1]).pow(2)) as f64).sqrt()
        };
        let get_vec = |p1: &Vec<i32>, p2: &Vec<i32>| (p2[0] - p1[0], p2[1] - p1[1]);
        let is_vertical =
            |vec1: (i32, i32), vec2: (i32, i32)| (vec1.0 * vec2.0 + vec1.1 * vec2.1) == 0;
        points.sort_by(|p1, p2| {
            let temp = get_distance(&anchor, p1) - get_distance(&anchor, p2);
            if temp > 0.0 {
                return std::cmp::Ordering::Greater;
            } else if temp < 0.0 {
                return std::cmp::Ordering::Less;
            }
            std::cmp::Ordering::Equal
        });

        is_vertical(
            get_vec(&points[0], &points[3]),
            get_vec(&points[1], &points[2]),
        ) && is_vertical(
            get_vec(&points[0], &points[1]),
            get_vec(&points[0], &points[2]),
        ) && is_vertical(
            get_vec(&points[3], &points[1]),
            get_vec(&points[3], &points[2]),
        )
    }
}
// @lc code=end
#[test]
fn test() {
    let p1 = vec![0, 0];
    let p2 = vec![1, 1];
    let p3 = vec![0, 1];
    let p4 = vec![1, 0];
    let res = Solution::valid_square(p1, p2, p3, p4);
    assert!(res);
}
