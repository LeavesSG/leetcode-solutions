/*
 * @lc app=leetcode id=1812 lang=rust
 *
 * [1812] Determine Color of a Chessboard Square
 */

use super::Solution;
#[allow(dead_code)]
// @lc code=start
impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let a = coordinates
            .chars()
            .next()
            .expect("123")
            .to_digit(20)
            .expect("");
        let b = coordinates
            .chars()
            .nth(1)
            .expect("456")
            .to_digit(10)
            .expect("");
        (a + b) % 2 == 0
    }
}
// @lc code=end

#[test]
fn test() {
    let coordinates = String::from("h3");
    let test = Solution::square_is_white(coordinates);
    assert!(test);
}
