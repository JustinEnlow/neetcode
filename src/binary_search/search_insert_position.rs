// 35. Search Insert Position
//
// Given a sorted array of distinct integers and a target value, return the 
// index if the target is found. If not, return the index where it would be if 
// it were inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Constraints:
//     1 <= nums.length <= 10^4
//     -10^4 <= nums[i] <= 10^4
//     nums contains distinct values sorted in ascending order.
//     -10^4 <= target <= 10^4

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32{
    binary_search(&nums, target, 0, nums.len() - 1)
}

fn binary_search(nums: &[i32], target: i32, low: usize, high: usize) -> i32{
    if low > high{
        low as i32
    }
    else{
        let mid = (low + high) / 2;
        match target.cmp(&nums[mid]){
            std::cmp::Ordering::Equal => {
                mid as i32
            }
            std::cmp::Ordering::Greater => {
                binary_search(nums, target, mid + 1, high)
            }
            std::cmp::Ordering::Less => {
                binary_search(nums, target, low, mid - 1)
            }
        }
    }
}

fn _do_test(nums: &[i32], target: i32, expected: i32){
    let result = search_insert(nums.to_vec(), target);
    assert!(
        result == expected,
        "\nInput = {nums:?} and {target:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: nums = [1,3,5,6], target = 5, Output: 2
    _do_test(&[1, 3, 5, 6], 5, 2);
}

#[test]
fn example_2(){
    //Input: nums = [1,3,5,6], target = 2, Output: 1
    _do_test(&[1, 3, 5, 6], 2, 1);
}

#[test]
fn example_3(){
    //Input: nums = [1,3,5,6], target = 7, Output: 4
    _do_test(&[1, 3, 5, 6], 7, 4);
}