/*
 * @lc app=leetcode id=2024 lang=rust
 *
 * [2024] Maximize the Con&&&fusion of an Exam
 */
use super::Solution;
#[allow(dead_code)]

// @lc code=start
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut chars: Vec<_> = answer_key.chars().collect();
        let mut last_char: char = '0';
        let mut char_count = 0;
        let mut max = 0;
        let mut tracking: Vec<(char, i32, i32)> = Vec::new();
        fn check(
            tracking: &mut Vec<(char, i32, i32)>,
            last_char: char,
            char_count: i32,
            mut max: i32,
            last_call: bool,
        ) -> i32 {
            let mut buffer = Vec::new();
            buffer.append(tracking);
            while !buffer.is_empty() {
                let mut item = buffer.pop().expect("");
                if item.0 == last_char {
                    item.2 += &char_count;
                } else {
                    item.1 -= &char_count;
                    item.2 += &char_count;
                }
                if item.1 >= 0 {
                    tracking.push(item);
                } else {
                    max = max.max(item.1 + item.2);
                }
            }
            if last_call {
                while !tracking.is_empty() {
                    let item = tracking.pop().expect("msg");
                    max = max.max(item.2);
                }
            }
            max
        }

        for item in &chars {
            if last_char == *item {
                char_count += 1
            } else {
                max = check(&mut tracking, last_char, char_count, max, false);
                last_char = *item;
                char_count = 1;
                tracking.push((last_char, k, 0));
            }
        }
        max = check(&mut tracking, last_char, char_count, max, true);
        chars.reverse();
        last_char = '0';
        char_count = 0;

        for item in chars {
            if last_char == item {
                char_count += 1
            } else {
                max = check(&mut tracking, last_char, char_count, max, false);

                last_char = item;
                char_count = 1;
                tracking.push((last_char, k, 0));
            }
        }
        max = check(&mut tracking, last_char, char_count, max, true);
        max
    }
}
// @lc code=end

#[test]
fn test() {
    let answer_key = String::from("TTTTTTFFFFFTTTTTTFFFTTTTT");
    let k = 4;
    let result = Solution::max_consecutive_answers(answer_key, k);
    println!("{}", result);
}
