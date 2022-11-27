// 15. 3sum

// Given an integer array nums, return all the triplets 
// [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and 
// nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.


//Constraints:
//    3 <= nums.length <= 3000
//    -105 <= nums[i] <= 105





pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    
    let mut result = Vec::new();
    
    nums.sort();
    
    for (i, &num) in nums.iter().enumerate(){
        if i > 0 && num == nums[i - 1]{continue;}
        
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        
        while left < right{
            let sum = num + nums[left] + nums[right];
            
            if sum > 0{
                right = right - 1;
            }
            else if sum < 0{
                left = left + 1;
            }
            else{
                result.push(vec![num, nums[left], nums[right]]);
                println!("{:?}", result);
                left = left + 1;

                while nums[left] == nums[left - 1] && left < right{
                    left = left + 1;
                }
            }
        }
    }
    
    result
}




fn _do_test(example: &str, nums: &[i32], expected: Vec<Vec<i32>>){
    let result = three_sum(nums.to_vec());
    assert!(
        result == expected,
        "\n{example:?}: input = {nums:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1: Input: nums = [-1,0,1,2,-1,-4], Output: [[-1,-1,2],[-1,0,1]]
    //Explanation: 
    //nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
    //nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
    //nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
    //The distinct triplets are [-1,0,1] and [-1,-1,2].
    //Notice that the order of the output and the order of the triplets does not matter.
    _do_test("ex_1", &[-1, 0, 1, 2, -1, -4], vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

    //Example 2: Input: nums = [0,1,1], Output: []
    //Explanation: The only possible triplet does not sum up to 0.
    _do_test("ex_2", &[0, 1, 1], vec![]);

    //Example 3: Input: nums = [0,0,0], Output: [[0,0,0]]
    //Explanation: The only possible triplet sums up to 0.
    _do_test("ex_3", &[0, 0, 0], vec![vec![0, 0, 0]]);
}