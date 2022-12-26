// 206. Reverse Linked List

// Given the head of a singly linked list, reverse the list, and return the 
// reversed list.

// Constraints:
//     The number of nodes in the list is the range [0, 5000].
//     -5000 <= Node.val <= 5000

// Follow up: A linked list can be reversed either iteratively or recursively. 
// Could you implement both? 





// Leetcode specified Linked List and function signature

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode{
//     pub val: i32,
//     pub next: Option<Box<ListNode>>
// }
//
// impl ListNode{
//     #[inline]
//     fn new(val: i32) -> Self{
//         ListNode{
//             next: None,
//             val
//         }
//     }
// }
// pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{}

// my function definition using built in linkedlist

use std::collections::LinkedList;
pub fn reverse_list(head: LinkedList<i32>) -> LinkedList<i32>{
    let mut new_list = LinkedList::new();
    
    for &x in head.iter(){
        new_list.push_front(x);
    }
    
    new_list
}





fn _do_test(head: LinkedList<i32>, expected: LinkedList<i32>){
    let result = reverse_list(head.clone());
    assert!(
        result == expected,
        "\nInput = {head:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: head = [1,2,3,4,5], Output: [5,4,3,2,1]
    _do_test(
        LinkedList::from([1, 2, 3, 4, 5]),
        LinkedList::from([5, 4, 3, 2, 1])
    );
}

#[test]
fn example_2(){
    //Input: head = [1,2], Output: [2,1]
    _do_test(
        LinkedList::from([1, 2]),
        LinkedList::from([2, 1])
    );
}

#[test]
fn example_3(){
    //Input: head = [], Output: []
    _do_test(
        LinkedList::from([]),
        LinkedList::from([])
    );
}