/*
 * @lc app=leetcode id=1 lang=javascript
 *
 * [1] Two Sum
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    let result = []
    for(let i=0;i<nums.length;i++){
      for(let j=i;j<nums.length;j++){
        if(j+1<=nums.length&&nums[i]+nums[j+1] === target)result.push(i,j+1)
      }
    }
    return result
};
// @lc code=end