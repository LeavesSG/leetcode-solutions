/*
 * @lc app=leetcode id=781 lang=rust
 *
 * [781] Rabbits in Forest
 */

use super::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        answers
            .into_iter()
            .for_each(|ans| match map.get(&(ans + 1)) {
                None => {
                    map.insert(ans + 1, 1);
                }
                Some(f) => {
                    map.insert(ans + 1, f + 1);
                }
            });
        let mut count = 0;
        map.into_iter().for_each(|(key, val)| {
            let mut div = val / key;
            if div * key != val {
                div += 1
            }
            count += div * key
        });
        count
    }
}
// @lc code=end

#[test]
fn test() {
    let answers = vec![1, 0, 1, 0, 0];
    let res = Solution::num_rabbits(answers);
    println!("{res}");
}
