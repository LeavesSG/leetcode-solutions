/*
 * @lc app=leetcode id=464 lang=rust
 *
 * [464] Can I Win
 */

use super::Solution;
#[allow(dead_code)]
// @lc code=start
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        const LEN: usize = 2_usize.pow(20);

        let candidates: Vec<_> = (1..max_choosable_integer + 1).into_iter().collect();
        let sum = candidates.iter().sum::<i32>();
        match sum.cmp(&desired_total) {
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Equal => return max_choosable_integer % 2 != 0,
            _ => (),
        }

        let mut dict: [u8; LEN] = [0; LEN];

        fn get_permutations(state: i32) -> Vec<i32> {
            let mut index = 0;
            let mut num = state;
            let mut dig;
            let mut vec = vec![];
            while num > 0 {
                if num == 0 {
                    return vec;
                }
                dig = num & 1;
                if dig == 1 {
                    let converted = state ^ (1 << index);
                    vec.push(converted);
                }
                num >>= 1;
                index += 1;
            }
            vec
        }

        fn get_pmt_max(mut pmt: i32) -> i32 {
            let mut max = 0;
            let mut index = 1;
            while pmt > 0 {
                if pmt & 1 == 1 {
                    max = index;
                }
                pmt >>= 1;
                index += 1;
            }
            max
        }

        fn get_diff(mut num: i32) -> i32 {
            let mut index = 0;
            while num > 0 {
                num >>= 1;
                index += 1
            }
            index
        }

        fn recursion(candidate: i32, target: i32, dict: &mut [u8; LEN]) -> bool {
            match dict[candidate as usize] {
                1 => true,
                2 => false,
                0 => {
                    let bool = get_permutations(candidate).into_iter().any(|num| {
                        let diff = target - get_diff(candidate ^ num);
                        let max = get_pmt_max(num);
                        if max >= diff {
                            return false;
                        }
                        !recursion(num, diff, dict)
                    });
                    dict[candidate as usize] = if bool { 1 } else { 2 };
                    bool
                }
                _ => true,
            }
        }
        if max_choosable_integer >= desired_total {
            return true;
        }
        recursion(
            2_i32.pow(max_choosable_integer as u32) - 1,
            desired_total,
            &mut dict,
        )
    }
}
// @lc code=end

#[test]
fn test() {
    let res = Solution::can_i_win(20, 215);
    assert!(!res);
}
