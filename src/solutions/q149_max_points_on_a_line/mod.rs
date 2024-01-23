/*
 * @lc app=leetcode id=149 lang=rust
 *
 * [149] Max Points on a Line
 */

// (x2 - x1)y - (x2- x1)y1 = (y2 - y1)x - (y2 - y1)x1
// (x2 - x1)y = (y2 - y1)x - (y2 - y1) * x1 + (x2 - x1) * y1
// y = (y2 - y1) x  + x2y1 - x1y2
// k = (y2 - y1) / (x2-x1)
// b = (x2y1 - x1y2) / (x2 - x1)

use super::Solution;

// @lc code=start
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();
        let get_key = |k: f64, b: f64| {
            format!("{},{}", k, b)
                .replace("-0", "0")
                .replace("-inf", "inf")
        };
        let calc_kb = |x1: f64, y1: f64, x2: f64, y2: f64| {
            if x2 == x1 {
                return (std::f64::INFINITY, x2);
            }
            return ((y2 - y1) / (x2 - x1), (x2 * y1 - x1 * y2) / (x2 - x1));
        };
        let mut counter: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        for i in 0..len - 1 {
            for j in i + 1..len {
                let x1 = points[i][0] as f64;
                let y1 = points[i][1] as f64;
                let x2 = points[j][0] as f64;
                let y2 = points[j][1] as f64;
                let (k, b) = calc_kb(x1, y1, x2, y2);
                let key = get_key(k, b);
                let count = counter.get(&key);
                match count {
                    Some(count) => counter.insert(key, count + 1),
                    None => counter.insert(key, 1),
                };
            }
        }
        let calc_count = |count: f64| (((count * 8.0 + 1.0).sqrt() - 1.0) / 2.0) as i32;
        counter
            .into_iter()
            .map(|(_, count)| calc_count(count as f64) + 1)
            .max()
            .unwrap_or(1)
    }
}
// @lc code=end

#[test]
fn test() {
    let points = vec![vec![4, 5], vec![4, -1], vec![4, 0]];
    let res = Solution::max_points(points);
    println!("{res}");
}
