// 128. Longest Consecutive Sequence

// Given an unsorted array of integers nums, return the length of the longest 
// consecutive elements sequence.
// You must write an algorithm that runs in O(n) time.

//Constraints:
//    0 <= nums.length <= 105
//    -109 <= nums[i] <= 109


use std::collections::HashSet;


pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    //let mut set = HashSet::new();
    //for num in nums{
    //    if !set.contains(&num){
    //        set.insert(num);
    //    }
    //}
    let set: HashSet<i32> = nums.into_iter().collect();

    let mut longest = 0;

    for &x in &set{
        if !set.contains(&(x - 1)){ // ensures x is the start of a sequence
            let mut length = 1;
            
            while set.contains(&(x + length)){
                length = length.saturating_add(1); // length += 1
            }

            longest = if longest > length{
                longest
            }
            else{
                length
            };
        }
    }
    
    longest
}





fn _do_test(nums: &[i32], expected: i32){
    let result = longest_consecutive(nums.to_vec());
    assert!(
        result == expected,
        "\ninput = {nums:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    // Example 1: Input: nums = [100,4,200,1,3,2], Output: 4
    // Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
    _do_test(&[100, 4, 200, 1, 3, 2], 4);
}

#[test]
fn example_2(){
    // Example 2: Input: nums = [0,3,7,2,5,8,4,6,0,1], Output: 9
    _do_test(&[0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9);
}