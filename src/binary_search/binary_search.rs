// 704. Binary Search
//
// Given an array of integers nums which is sorted in ascending order, and an 
// integer target, write a function to search target in nums. If target exists, 
// then return its index. Otherwise, return -1.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Constraints:
//     1 <= nums.length <= 104
//     -104 < nums[i], target < 104
//     All the integers in nums are unique.
//     nums is sorted in ascending order.


use std::cmp::Ordering;


pub fn search(nums: Vec<i32>, target: i32) -> i32{
    binary_search(&nums, target, 0, nums.len() - 1)
}

fn binary_search(nums: &[i32], target: i32, low: usize, high: usize) -> i32{
    if low > high{
        -1
    }
    else{
        let mid = (low + high) / 2;
        match target.cmp(&nums[mid]){
            Ordering::Equal => {
                mid as i32
            }
            Ordering::Greater => {
                binary_search(nums, target, mid + 1, high)
            }
            Ordering::Less => {
                binary_search(nums, target, low, mid - 1)
            }
        }
    }
}





fn _do_test(nums: &[i32], target: i32, expected: i32){
    let result = search(nums.to_vec(), target);
    assert!(
        result == expected,
        "\nInput = {nums:?} and {target:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: nums = [-1,0,3,5,9,12], target = 9, Output: 4
    //Explanation: 9 exists in nums and its index is 4
    _do_test(&[-1,0,3,5,9,12], 9, 4);
}

#[test]
fn example_2(){
    //Input: nums = [-1,0,3,5,9,12], target = 2, Output: -1
    //Explanation: 2 does not exist in nums so return -1
    _do_test(&[-1,0,3,5,9,12], 2, -1);
}