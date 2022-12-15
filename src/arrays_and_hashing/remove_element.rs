// Given an integer array nums and an integer val, remove all occurrences of 
// val in nums in-place. The relative order of the elements may be changed.
//
// Since it is impossible to change the length of the array in some languages, 
// you must instead have the result be placed in the first part of the array 
// nums. More formally, if there are k elements after removing the duplicates, 
// then the first k elements of nums should hold the final result. It does not 
// matter what you leave beyond the first k elements.
//
// Return k after placing the final result in the first k slots of nums.
//
// Do not allocate extra space for another array. You must do this by modifying 
// the input array in-place with O(1) extra memory.
//
// Constraints:
//     0 <= nums.length <= 100
//     0 <= nums[i] <= 50
//     0 <= val <= 100





pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32{
    let mut k = nums.len();
    let mut i = 0;
    
    while i < k{
        if nums[i] == val{
            k -= 1;
            
            let curr_num = nums[i];
            //move all vec elements down one index and place nums[i] at end of vec
            for j in i..nums.len(){
                if j + 1 < nums.len(){
                    nums[j] = nums[j + 1];
                }
                else{
                    nums[j] = curr_num;
                }
            }
        }
        else{
            i += 1;
        }
    }

    k as i32
}





// Custom Judge:
//
// The judge will test your solution with the following code:
//
// int[] nums = [...]; // Input array
// int val = ...; // Value to remove
// int[] expectedNums = [...]; // The expected answer with correct length.
//                             // It is sorted with no values equaling val.
//
// int k = removeElement(nums, val); // Calls your implementation
//
// assert k == expectedNums.length;
// sort(nums, 0, k); // Sort the first k elements of nums
// for (int i = 0; i < actualLength; i++) {
//     assert nums[i] == expectedNums[i];
// }
// If all assertions pass, then your solution will be accepted.
fn _do_test(nums: &mut Vec<i32>, val: i32, expected: i32, expected_nums: Vec<i32>){
    let nums_input = nums.clone();
    let result = remove_element(nums, val);
    assert!(
        result == expected,
        "\nInput = {nums_input:?} and {val:?}, expected {expected:?} but got {result:?}\n"
    );
    for i in 0..expected{
        assert!(
            nums[i as usize] == expected_nums[i as usize],
            "\nnums[{i:?}]: {:?} should equal new_nums[{i:?}]: {:?}\n", nums[i as usize], expected_nums[i as usize]
        );
    }
}

#[test]
fn example_1(){
    //Input: nums = [3,2,2,3], val = 3
    //Output: 2, nums = [2,2,_,_]
    //Explanation: Your function should return k = 2, with the first two 
    //elements of nums being 2.
    //It does not matter what you leave beyond the returned k (hence they are 
    //underscores).
    _do_test(&mut vec![3, 2, 2, 3], 3, 2, vec![2, 2, 3, 3]);
}

#[test]
fn example_2(){
    //Input: nums = [0,1,2,2,3,0,4,2], val = 2
    //Output: 5, nums = [0,1,4,0,3,_,_,_]
    //Explanation: Your function should return k = 5, with the first five 
    //elements of nums containing 0, 0, 1, 3, and 4.
    //Note that the five elements can be returned in any order.
    //It does not matter what you leave beyond the returned k (hence they are 
    //underscores).
    _do_test(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5, vec![0, 1, 3, 0, 4, 2, 2, 2]);
}