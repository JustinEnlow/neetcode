// 33. Search in Rotated Sorted Array

// There is an integer array nums sorted in ascending order (with distinct 
// values).

// Prior to being passed to your function, nums is possibly rotated at an 
// unknown pivot index k (1 <= k < nums.length) such that the resulting array 
// is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] 
// (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 
// and become [4,5,6,7,0,1,2].

// Given the array nums after the possible rotation and an integer target, 
// return the index of target if it is in nums, or -1 if it is not in nums.

// You must write an algorithm with O(log n) runtime complexity.


//Constraints:
//    1 <= nums.length <= 5000
//    -104 <= nums[i] <= 104
//    All values of nums are unique.
//    nums is an ascending array that is possibly rotated.
//    -104 <= target <= 104





pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    //for (i, &num) in nums.iter().enumerate(){
    //    if num == target{
    //        return i as i32;
    //    }
    //}

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right{
        let middle = (left + right) / 2;

        if nums[middle] == target{
            return middle as i32;
        }

        if nums[left] <= nums[middle]{
            if nums[left] <= target && target <= nums[middle]{
                right = middle - 1;
            }
            else{
                left = middle + 1;
            }
        }
        else if nums[middle] <= target && target <= nums[right]{
            left = middle + 1;
        }
        else{
            right = middle - 1;
        }
        //else{
        //    if nums[middle] <= target && target <= nums[right]{
        //        left = middle + 1;
        //    }
        //    else{
        //        right = middle - 1;
        //    }
        //}
    }

    -1
}





fn _do_test(nums: &[i32], target: i32, expected: i32){
    let result = search(nums.to_vec(), target);
    assert!(
        result == expected,
        "\nInput = {nums:?} and {target:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: nums = [4,5,6,7,0,1,2], target = 0, Output: 4
    _do_test(&[4, 5, 6, 7, 0, 1, 2], 0, 4);
}

#[test]
fn example_2(){
    //Example 2: Input: nums = [4,5,6,7,0,1,2], target = 3, Output: -1
    _do_test(&[4, 5, 6, 7, 0, 1, 2], 3, -1);
}

#[test]
fn example_3(){
    //Example 3: Input: nums = [1], target = 0, Output: -1
    _do_test(&[1], 0, -1);
}