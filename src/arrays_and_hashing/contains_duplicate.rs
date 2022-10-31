//Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

//Constraints:
//    1 <= nums.length <= 105
//    -109 <= nums[i] <= 109





use std::collections::HashSet;



pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    ///////////////////////////////////////////////////////////////////////////
    //for (i, &outer_val) in nums.iter().enumerate(){
    //    for (j, &inner_val) in nums.iter().enumerate(){
    //        //if same index, skip
    //        if j == i{
    //            continue
    //        }
    //        else if inner_val == outer_val{
    //            return true;
    //        }
    //    }
    //}
    ////if we iterate through entire slice and havent returned true, we know there
    ////are no duplicates
    //false
    ///////////////////////////////////////////////////////////////////////////
    


    ///////////////////////////////////////////////////////////////////////////
    //for (i, &outer_val) in nums.iter().enumerate(){
    //    for j in (i + 1)..nums.len(){
    //        let inner_val = nums[j];
    //        println!("{}, {}", outer_val, inner_val);
    //        if inner_val == outer_val{
    //            return true;
    //        }
    //    }
    //}
    ////if we iterate through entire slice and havent returned true, we know there
    ////are no duplicates
    //false
    ///////////////////////////////////////////////////////////////////////////
    


    ///////////////////////////////////////////////////////////////////////////
    let mut hash_set = HashSet::new();

    for &value in nums.iter(){
        if hash_set.contains(&value){
            return true;
        }
        else{
            hash_set.insert(value);
        }
    }
    false
    ///////////////////////////////////////////////////////////////////////////
}





fn _do_test(example: &str, nums: &[i32], expected: bool){
    let result = contains_duplicate(nums.to_vec());
    assert!(
        result == expected,
        "\n{example:?}: input = {nums:?}, Expected = {expected:?} but got {result:?}\n"
    );
}

#[test]
fn tests(){
    //Example 1: Input: nums = [1,2,3,1], Output: true
    _do_test("ex_1", &[1, 2, 3, 1], true);
    
    //Example 2: Input: nums = [1,2,3,4], Output: false
    _do_test("ex_2", &[1, 2, 3, 4], false);
    
    //Example 3: Input: nums = [1,1,1,3,3,4,3,2,4,2], Output: true
    _do_test("ex_3", &[1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true);
}