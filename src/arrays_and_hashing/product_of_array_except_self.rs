// Given an integer array nums, return an array answer such that answer[i] is 
// equal to the product of all the elements of nums except nums[i].
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
// You must write an algorithm that runs in O(n) time and without using the division operation.
//
// Constraints:
//     2 <= nums.length <= 105
//     -30 <= nums[i] <= 30
//     The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer. 
//
// Follow up: Can you solve the problem in O(1) extra space complexity? (The 
// output array does not count as extra space for space complexity analysis.)





// is this solution O(n)
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut vec = Vec::new();
    
    for i in 0..nums.len(){
        let mut first_run = true;
        let mut product = 0;
        
        for (j, &val) in nums.iter().enumerate(){
            if i == j{
                continue;
            }
            
            if first_run{
                product = val;
                first_run = false;
            }
            else{
                product *= val; //product = product * val;
            }
        }

        vec.push(product);
    }

    vec
}





fn _do_test(nums: &[i32], expected: &[i32]){
    let result = product_except_self(nums.to_vec());
    assert!(
        result == expected.to_vec(),
        "\ninput = {nums:?}, Expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: nums = [1,2,3,4], Output: [24,12,8,6]
    _do_test(&[1, 2, 3, 4], &[24, 12, 8, 6]);
}

#[test]
fn example_2(){
    //Input: nums = [-1,1,0,-3,3], Output: [0,0,9,0,0]
    _do_test(&[-1, 1, 0, -3, 3], &[0, 0, 9, 0, 0]);
}