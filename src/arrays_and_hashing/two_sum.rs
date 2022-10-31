// Given an array of integers nums and an integer target, return indices of the 
// two numbers such that they add up to target. You may assume that each input 
// would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

// Constraints:
//     2 <= nums.length <= 104
//     -109 <= nums[i] <= 109
//     -109 <= target <= 109
//     Only one valid answer exists.


use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    ///////////////////////// Solution 1 //////////////////////////////////////
    //let mut vec = Vec::new();
    //
    //for i in 0..nums.len(){
    //    for j in i+1..nums.len(){
    //        if nums[i] + nums[j] == target{
    //            vec.push(i as i32);
    //            vec.push(j as i32);
    //        }
    //    }
    //}
    //
    //vec
    ///////////////////////////////////////////////////////////////////////////
    


    ///////////////////////// Solution 2 //////////////////////////////////////
    let mut hashmap = HashMap::new();
    let mut vec = Vec::new();
    
    for (i, &val) in nums.iter().enumerate(){
        let remainder = target - val;
        match hashmap.get(&remainder){
            Some(&j) => {
                vec.push(j as i32);
                vec.push(i as i32);
            },
            None => {}
        }
        hashmap.insert(val, i);
    }
    
    vec
    ///////////////////////////////////////////////////////////////////////////
}





fn _do_test(example: &str, nums: &[i32], target: i32, expected: &[i32]){
    let result = two_sum(nums.to_vec(), target);
    assert!(
        result == expected,
        "\n{example:?}: input = {nums:?} and {target:?}, Expected = {expected:?} but got {result:?}\n"
    );
}

#[test]
fn tests(){
    // Example 1:
    //     Input: nums = [2,7,11,15], target = 9
    //     Output: [0,1]
    //     Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    _do_test("ex_1", &[2, 7, 11, 15], 9, &[0, 1]);
    
    // Example 2:
    //     Input: nums = [3,2,4], target = 6
    //     Output: [1,2]
    _do_test("ex_2", &[3, 2, 4], 6, &[1, 2]);

    // Example 3:
    //     Input: nums = [3,3], target = 6
    //     Output: [0,1]
    _do_test("ex_3", &[3, 3], 6, &[0, 1]);
}