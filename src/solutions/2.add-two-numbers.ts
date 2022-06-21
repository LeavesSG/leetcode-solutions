/*
 * @lc app=leetcode id=2 lang=typescript
 *
 * [2] Add Two Numbers
 */

class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

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

function addTwoNumbers(l1: ListNode | null, l2: ListNode | null): ListNode | null {
  let root: ListNode | null = null;
  let pointer: ListNode | null = root;
  let carry = 0;

  root = l1 || l2;
  pointer = root;
  if (!pointer) return null;
  while (l1 || l2) {
    pointer.val = (l1?.val || 0) + (l2?.val || 0) + (carry ? carry-- : 0);
    if (pointer.val >= 10) {
      pointer.val -= 10;
      carry++;
    }

    // increment
    if (l1?.next) {
      pointer.next = l1.next;
    } else if (l2?.next) {
      pointer.next = l2.next;
      l2.next = null;
    } else pointer.next = null;
    if (l1) l1 = l1.next;
    if (l2) l2 = l2.next;
    if (pointer.next) pointer = pointer.next;
    else if (carry) pointer.next = new ListNode(carry--, null);
  }
  return root;
}
// @lc code=end
