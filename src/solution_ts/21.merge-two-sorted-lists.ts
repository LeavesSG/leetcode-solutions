/*
 * @lc app=leetcode id=21 lang=typescript
 *
 * [21] Merge Two Sorted Lists
 *
 * https://leetcode.com/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (59.16%)
 * Likes:    10478
 * Dislikes: 985
 * Total Accepted:    1.9M
 * Total Submissions: 3.3M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * You are given the heads of two sorted linked lists list1 and list2.
 *
 * Merge the two lists in a one sorted list. The list should be made by
 * splicing together the nodes of the first two lists.
 *
 * Return the head of the merged linked list.
 *
 *
 * Example 1:
 *
 *
 * Input: list1 = [1,2,4], list2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 *
 *
 * Example 2:
 *
 *
 * Input: list1 = [], list2 = []
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: list1 = [], list2 = [0]
 * Output: [0]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in both lists is in the range [0, 50].
 * -100 <= Node.val <= 100
 * Both list1 and list2 are sorted in non-decreasing order.
 *
 *
 */

import { ListNode } from "./utils";

// @lc code=start
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */
function mergeTwoLists(list1: ListNode | null, list2: ListNode | null): ListNode | null {
  const __less = (i: ListNode | null, j: ListNode | null) => {
    if (!i && !j) return null;
    else if (!i) return j;
    else if (!j) return i;
    else return i.val <= j.val ? i : j;
  };
  let root = null;
  let pointer: ListNode | null = null;
  while (list1 || list2) {
    const less = __less(list1, list2);
    if (less === list1) {
      list1 = list1?.next || null;
    } else {
      list2 = list2?.next || null;
    }
    if (!root) {
      root = less;
      pointer = root;
    } else if (pointer) {
      pointer.next = less;
      pointer = less;
    }
  }

  return root;
}
// @lc code=end

// class ListNode {
//   val: number;
//   next: ListNode | null;
//   constructor(val?: number, next?: ListNode | null) {
//     this.val = val === undefined ? 0 : val;
//     this.next = next === undefined ? null : next;
//   }
// }
// const __form_new_node_list = (arr: number[]) => {
//   if (arr.length === 0) return null;
//   return arr.reverse().reduce((a: ListNode | null, c) => new ListNode(c, a), null);
// };
// const arr1 = __form_new_node_list([1, 2, 4]);
// const arr2 = __form_new_node_list([1, 2, 3]);
// console.log(fetchResult(arr1), fetchResult(arr2));
// const result = mergeTwoLists(arr1, arr2);
// function fetchResult(l: ListNode | null) {
//   const arr = [];
//   let i = l;
//   while (i) {
//     arr.push(i.val);
//     i = i.next;
//   }
//   return arr;
// }
// console.log(fetchResult(result));
