// 153. Find Minimum in Rotated Sorted Array

// Suppose an array of length n sorted in ascending order is rotated between 1 
// and n times. For example, the array nums = [0,1,2,4,5,6,7] might become:
//    [4,5,6,7,0,1,2] if it was rotated 4 times.
//    [0,1,2,4,5,6,7] if it was rotated 7 times.

// Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results 
// in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].

// Given the sorted rotated array nums of unique elements, return the minimum 
// element of this array.

//You must write an algorithm that runs in O(log n) time.


//Constraints:
//    n == nums.length
//    1 <= n <= 5000
//    -5000 <= nums[i] <= 5000
//    All the integers of nums are unique.
//    nums is sorted and rotated between 1 and n times.





pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    let mut min = nums[0];

    while left < right{
        let middle = (left + right) / 2;

        min = if nums[middle] < min{
            nums[middle]
        }
        else{
            min
        };

        if nums[middle] > nums[right]{
            left = middle + 1;
        }
        else{
            right = middle;
        }
    }

    nums[left]
} 





fn _do_test(nums: &[i32], expected: i32){
    let result = find_min(nums.to_vec());
    assert!(
        result == expected,
        "\nInput = {nums:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: nums = [3,4,5,1,2], Output: 1
    //Explanation: The original array was [1,2,3,4,5] rotated 3 times.
    _do_test(&[3, 4, 5, 1, 2], 1);
}

#[test]
fn example_2(){
    //Example 2: Input: nums = [4,5,6,7,0,1,2], Output: 0
    //Explanation: The original array was [0,1,2,4,5,6,7] and it was rotated 4 times.
    _do_test(&[4, 5, 6, 7, 0, 1, 2], 0);
}

#[test]
fn example_3(){
    //Example 3: Input: nums = [11,13,15,17], Output: 11
    //Explanation: The original array was [11,13,15,17] and it was rotated 4 times. 
    _do_test(&[11, 13, 15, 17], 11);
}