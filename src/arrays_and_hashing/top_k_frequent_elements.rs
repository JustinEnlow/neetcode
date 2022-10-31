// Given an integer array nums and an integer k, return the k most frequent 
// elements. You may return the answer in any order.

//Constraints:
//    1 <= nums.length <= 105
//    -104 <= nums[i] <= 104
//    k is in the range [1, the number of unique elements in the array].
//    It is guaranteed that the answer is unique.





use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    
    for &num in nums.iter(){
        match hashmap.contains_key(&num){
            true => {
                hashmap.insert(
                    num, 
                    hashmap.get(&num).unwrap() + 1  // increment count
                );
            },
            false => {
                hashmap.insert(num, 1);
            }
        }
    }

    let mut vec = Vec::new();
    // assign key_pair of k most frequent value_pair to vec
    for _ in 0..k{
        let mut most_freq_val = 0;
        let mut most_freq_key = 0;
        for (&key, &val) in hashmap.iter(){
            if val > most_freq_val{
                most_freq_val = val;
                most_freq_key = key;
            }
        }
        hashmap.remove(&most_freq_key);

        vec.push(most_freq_key);
    }

    vec
}





fn _do_test(example: &str, nums: &[i32], k: i32, expected: &[i32]){
    let result = top_k_frequent(nums.to_vec(), k);
    assert!(
        result == expected.to_vec(),
        "\n{example:?}: input = {nums:?} and {k:?}, Expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1: Input: nums = [1,1,1,2,2,3], k = 2, Output: [1,2]
    _do_test("ex_1", &[1, 1, 1, 2, 2, 3], 2, &[1, 2]);

    //Example 2: Input: nums = [1], k = 1, Output: [1]
    _do_test("ex_2", &[1], 1, &[1]);
}