/*
 * @lc app=leetcode id=49 lang=typescript
 *
 * [49] Group Anagrams
 *
 * https://leetcode.com/problems/group-anagrams/description/
 *
 * algorithms
 * Medium (63.44%)
 * Likes:    8440
 * Dislikes: 289
 * Total Accepted:    1.2M
 * Total Submissions: 2M
 * Testcase Example:  '["eat","tea","tan","ate","nat","bat"]'
 *
 * Given an array of strings strs, group the anagrams together. You can return
 * the answer in any order.
 *
 * An Anagram is a word or phrase formed by rearranging the letters of a
 * different word or phrase, typically using all the original letters exactly
 * once.
 *
 *
 * Example 1:
 * Input: strs = ["eat","tea","tan","ate","nat","bat"]
 * Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 * Example 2:
 * Input: strs = [""]
 * Output: [[""]]
 * Example 3:
 * Input: strs = ["a"]
 * Output: [["a"]]
 *
 *
 * Constraints:
 *
 *
 * 1 <= strs.length <= 10^4
 * 0 <= strs[i].length <= 100
 * strs[i] consists of lowercase English letters.
 *
 *
 */

// @lc code=start
const nums = [-1, 2];
function groupAnagrams(strs: string[]): string[][] {
  const grouped: { [index: string]: string[] } = {};
  const hash = (str: string) => {
    return str.split("").sort().join();
  };
  strs.forEach((str) => {
    if (grouped[hash(str)]) grouped[hash(str)].push(str);
    else grouped[hash(str)] = [str];
  });
  return Object.values(grouped);
}
// @lc code=end
console.log(groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"]));
