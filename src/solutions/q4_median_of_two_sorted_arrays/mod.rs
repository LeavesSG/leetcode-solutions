/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

use std::vec;

use super::Solution;

// @lc code=start

struct AbstractMerge {
    buffer: Vec<Vec<i32>>,
}

impl AbstractMerge {
    fn len(&self) -> usize {
        return self.buffer.iter().fold(0, |acc, e| acc + e.len());
    }

    fn get_on_index(&self, index: usize) -> i32 {
        let mut index = index;
        for vec in self.buffer.iter() {
            if vec.len() > index {
                return vec[index];
            }
            index -= vec.len();
        }
        return 0;
    }

    fn find_median(&self) -> f64 {
        let l = self.len();
        let median = l / 2;
        if l % 2 == 1 {
            return self.get_on_index(median) as f64;
        } else {
            return (self.get_on_index(median) as f64 + self.get_on_index(median - 1) as f64)
                / 2f64;
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let merged: AbstractMerge;
        let m = nums1.len();
        let n = nums2.len();
        if m == 0 {
            merged = AbstractMerge {
                buffer: vec![nums2],
            }
        } else if n == 0 {
            merged = AbstractMerge {
                buffer: vec![nums1],
            }
        } else if nums1[m - 1] < nums2[0] {
            merged = AbstractMerge {
                buffer: vec![nums1, nums2],
            }
        } else if nums2[n - 1] < nums1[0] {
            merged = AbstractMerge {
                buffer: vec![nums2, nums1],
            }
        } else {
            let (mut i, mut j) = (m / 2, n / 2);
            loop {
                if i > 0 && j < n && nums1[i - 1] > nums2[j] && nums2.len() > n && m > 1 {
                    i -= 1;
                    j += 1;
                }
                if i < m && j > 0 && nums1[i] < nums2[j - 1] && nums1.len() > m && n > 1 {
                    i += 1;
                    j -= 1;
                }
                break;
            }
            let mut vec = vec![];
            if m % 2 == 0 {
                vec.push(nums1[i - 1]);
            }
            vec.push(nums1[i]);
            if n % 2 == 0 {
                vec.push(nums2[j - 1]);
            }
            vec.push(nums2[j]);
            vec.sort();
            merged = AbstractMerge { buffer: vec![vec] }
        }

        merged.find_median()
    }
}
// @lc code=end

#[test]
fn test() {
    let mut nums1 = vec![2];
    let mut nums2 = vec![1, 3, 4, 5, 6];
    let res = Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone());
    let mut num = vec![];
    num.append(&mut nums1);
    num.append(&mut nums2);
    println!("{res}");
    num.sort();
    println!("{:?}", num);
    println!("{}", AbstractMerge { buffer: vec![num] }.find_median());
}

// [1, 2, 3, 4|]
// [6, 7, 8, |10, |11, 12, 13, 14, 15, 16, ]

// Some(4), None, Some(8), Some(10)
