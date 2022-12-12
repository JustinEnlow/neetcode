// 268. Missing Number
//
// Given an array nums containing n distinct numbers in the range [0, n], 
// return the only number in the range that is missing from the array.
//
// Constraints:
//     n == nums.length
//     1 <= n <= 104
//     0 <= nums[i] <= n
//     All the numbers of nums are unique.
//
// Follow up: Could you implement a solution using only O(1) extra space 
// complexity and O(n) runtime complexity?



//use std::collections::HashSet;



pub fn missing_number(nums: Vec<i32>) -> i32 {
    // my solution. does not employ bit manipulation tho
    //let set: HashSet<i32> = nums.into_iter().collect();
    //
    //let mut missing = 0;
    //
    //for &num in &set{
    //    if set.contains(&(num - 1)) && !set.contains(&(num + 1)){
    //        missing = num + 1;
    //    }
    //}
    //
    //missing

    /////////////////////////////////////////////////////////////
    let mut range_sum = 0;
    for i in 0..nums.len() + 1{
        range_sum += i; //range_sum = range_sum + i;
    }

    let mut num_sum = 0;
    for num in nums{
        num_sum += num; //num_sum = num_sum + num;
    }
    
    range_sum as i32 - num_sum
}





fn _do_test(nums: &[i32], expected: i32){
    let result = missing_number(nums.to_vec());
    assert!(
        result == expected,
        "\nInput = {nums:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: nums = [3,0,1], Output: 2
    //Explanation: n = 3 since there are 3 numbers, so all numbers are in the 
    //range [0,3]. 2 is the missing number in the range since it does not 
    //appear in nums.
    _do_test(&[3, 0, 1], 2);
}

#[test]
fn example_2(){
    //Example 2: Input: nums = [0,1], Output: 2
    //Explanation: n = 2 since there are 2 numbers, so all numbers are in the 
    //range [0,2]. 2 is the missing number in the range since it does not 
    //appear in nums.
    _do_test(&[0, 1], 2);
}

#[test]
fn example_3(){
    //Example 3: Input: nums = [9,6,4,2,3,5,7,0,1], Output: 8
    //Explanation: n = 9 since there are 9 numbers, so all numbers are in the 
    //range [0,9]. 8 is the missing number in the range since it does not 
    //appear in nums.
    _do_test(&[9, 6, 4, 2, 3, 5, 7, 0, 1], 8);
}