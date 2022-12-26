// 977. Square of a Sorted Array
//
// Given an integer array nums sorted in non-decreasing order, return an array 
// of the squares of each number sorted in non-decreasing order.

// Constraints:
//     1 <= nums.length <= 104
//     -104 <= nums[i] <= 104
//     nums is sorted in non-decreasing order.

// Follow up: 
//     Squaring each element and sorting the new array is very trivial, could 
//     you find an O(n) solution using a different approach?





pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32>{
    /////// Solution 1 ////////////////////////////////////////////////////////
    let mut new_nums: Vec<i32> = nums.iter().map(|x| x * x).collect();
    
    new_nums.sort();

    new_nums
    ///////////////////////////////////////////////////////////////////////////
}





fn _do_test(nums: &[i32], expected: &[i32]){
    let result = sorted_squares(nums.to_vec());
    assert!(
        result == expected,
        "\nInput = {nums:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: nums = [-4,-1,0,3,10], Output: [0,1,9,16,100]
    //Explanation: After squaring, the array becomes [16,1,0,9,100].
    //After sorting, it becomes [0,1,9,16,100].
    _do_test(&[-4, -1, 0, 3, 10], &[0, 1, 9, 16, 100]);
}

#[test]
fn example_2(){
    //Input: nums = [-7,-3,2,3,11], Output: [4,9,9,49,121]
    _do_test(&[-7, -3, 2, 3, 11], &[4, 9, 9, 49, 121]);
}