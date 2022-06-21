use super::Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let m = nums.len();
        if m < 2 {
            return 0;
        }
        let mut lb = 0;
        let mut ub = 1;
        let mut min = 0;
        let mut max = 1;
        for i in 2..max {
            if nums[i] < nums[lb] {
                if nums[lb] - nums[i] > nums[max] - nums[min] {
                    min = i;
                    max = lb;
                }
                lb = i;
            } else if nums[i] > nums[ub] {
                if nums[i] - nums[ub] > nums[max] - nums[min] {
                    max = i;
                    min = ub;
                }
                ub = i;
            }
        }
        return nums[min] - nums[max];
    }
}
