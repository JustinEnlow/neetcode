// 136. Single Number
//
// Given a non-empty array of integers nums, every element appears twice except 
// for one. Find that single one.
//
// You must implement a solution with a linear runtime complexity and use only 
// constant extra space.
//
// Constraints:
//     1 <= nums.length <= 3 * 104
//     -3 * 104 <= nums[i] <= 3 * 104
//     Each element in the array appears twice except for one element which appears only once.





pub fn single_number(nums: Vec<i32>) -> i32{
    let mut non_matching = 0;

    for num in nums{
        non_matching ^= num; //non_matching = num ^ non_matching;
    }

    non_matching
}




fn _do_test(nums: &[i32], expected: i32){
    let result = single_number(nums.to_vec());
    assert!(
        result == expected,
        "\nInput = {nums:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: nums = [2,2,1], Output: 1
    _do_test(&[2, 2, 1], 1);
}

#[test]
fn example_2(){
    //Input: nums = [4,1,2,1,2], Output: 4
    _do_test(&[4, 1, 2, 1, 2], 4);
}

#[test]
fn example_3(){
    //Input: nums = [1], Output: 1
    _do_test(&[1], 1);
}