/*
 * @lc app=leetcode id=554 lang=rust
 *
 * [554] Brick Wall
 */

use super::Solution;
// @lc code=start
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let len = wall.len() as i32;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for row in wall {
            let mut pos = 0;
            for block in row.iter().take(row.len() - 1) {
                pos += block;
                let find = map.get(&pos);
                match find {
                    Some(num) => map.insert(pos, num + 1),
                    None => map.insert(pos, 1),
                };
            }
        }
        println!("{:?}", map);
        len - match map.into_iter().max_by(|some1, some2| some1.1.cmp(&some2.1)) {
            None => 0,
            Some(num) => num.1,
        }
    }
}
// @lc code=end
#[test]
fn test() {
    let test = vec![vec![1], vec![1], vec![1]];
    let result = Solution::least_bricks(test);
    println!("{}", result)
}
