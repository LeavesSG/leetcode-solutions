/*
 * @lc app=leetcode id=59 lang=rust
 *
 * [59] Spiral Matrix II
 */
use super::Solution;
#[allow(dead_code)]

// @lc code=start

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        // let mut dir = 1_u8;
        // 1 -> right, 2 -> bottom 3 -> left 4 -> top
        // let dir_increment = |i: u8| if i == 3 { 0 } else { i + 1 };

        let mut vec = (0..n)
            .map(|_| (0..n).map(|_| -1).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut row = 0usize;
        let mut col = 0usize;
        let mut len = (n - 1) as usize;
        let mut counter = 1;
        loop {
            for dir in 1u8..5u8 {
                for _ in 0..len {
                    vec[row][col] = counter;
                    counter += 1;
                    match dir {
                        1 => col += 1,
                        2 => row += 1,
                        3 => col -= 1,
                        4 => row -= 1,
                        _ => {}
                    }
                }
            }
            row += 1;
            col += 1;
            if len <= 2 {
                break;
            }
            len -= 2;
        }
        if n % 2 == 1 {
            let middle = (n / 2) as usize;
            vec[middle][middle] = n.pow(2);
        }
        vec
    }
}

// @lc code=end

#[test]
fn test() {
    let res = Solution::generate_matrix(8);
    println!("{:?}", res);
}
