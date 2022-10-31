/*
 * @lc app=leetcode id=400 lang=rust
 *
 * [400] Nth Digit
 */

use super::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut index = 0_i32;
        let mut acc = 0_i32;
        let mut temp;
        let mut overflow;
        loop {
            index += 1;
            let res = (index * 9).overflowing_mul(10_i32.pow((index - 1) as u32));
            temp = res.0;
            overflow = res.1;
            if overflow || acc + temp > n {
                break;
            } else {
                acc += temp
            }
        }
        let floor = 10_i32.pow(index as u32 - 1) - 1 + (n - acc) / index;
        let frag = (n - acc) % index;
        if frag == 0 {
            floor
                .to_string()
                .into_bytes()
                .into_iter()
                .map(|num| (num - 48) as i32)
                .last()
                .unwrap()
        } else {
            (floor + 1)
                .to_string()
                .into_bytes()
                .into_iter()
                .map(|num| (num - 48) as i32)
                .nth((frag - 1) as usize)
                .unwrap()
        }
    }
}
// @lc code=end

#[test]
fn test() {
    let res = Solution::find_nth_digit(1000000000);
    println!("{:?}", res);
}
