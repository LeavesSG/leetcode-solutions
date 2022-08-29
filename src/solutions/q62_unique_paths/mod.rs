/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */
use super::Solution;
#[allow(dead_code)]
// @lc code=start
impl Solution {
    fn recursion(m: i32, n: i32, hash_map: &mut std::collections::HashMap<(i32, i32), i32>) -> i32 {
        let find = hash_map.get(&(m, n));
        match find {
            Some(v) => *v,
            None => {
                let result = if m == 1 || n == 1 {
                    1
                } else {
                    Self::recursion(m - 1, n, hash_map) + Self::recursion(m, n - 1, hash_map)
                };
                hash_map.insert((m, n), result);
                result
            }
        }
    }
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        Self::recursion(m, n, &mut map)
    }
}
// @lc code=end

#[test]
fn test() {
    let (m, n) = (51, 9);
    let result = Solution::unique_paths(m, n);
    println!("{}", result)
}
