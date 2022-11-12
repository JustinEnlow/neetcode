//Given a 1-indexed array of integers numbers that is already sorted in 
//non-decreasing order, find two numbers such that they add up to a specific 
//target number. Let these two numbers be numbers[index1] and numbers[index2] 
//where 1 <= index1 < index2 <= numbers.length.

//Return the indices of the two numbers, index1 and index2, added by one as an 
//integer array [index1, index2] of length 2.

//The tests are generated such that there is exactly one solution. You may not 
//use the same element twice.

//Your solution must use only constant extra space.

//Constraints:
//    2 <= numbers.length <= 3 * 104
//    -1000 <= numbers[i] <= 1000
//    numbers is sorted in non-decreasing order.
//    -1000 <= target <= 1000
//    The tests are generated such that there is exactly one solution.





pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = 1;

    let mut vec = Vec::new();

    loop{
        if right < numbers.len(){
            if numbers[right] + numbers[left] == target{
                vec.push((left as i32) + 1);
                vec.push((right as i32) + 1);
                break;
            }
            else{
                right += 1;
            }
        }
        else/*if right == numbers.len()*/{
            left += 1;
            right = left + 1;
        }

        if left == numbers.len(){
            break;
        }
    }

    vec
}




fn _do_test(example: &str, numbers: &[i32], target: i32, expected: &[i32]){
    let result = two_sum(numbers.to_vec(), target);
    assert!(
        result == expected.to_vec(),
        "\n{example:?}: input = {numbers:?} and {target:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1:
    //Input: numbers = [2,7,11,15], target = 9
    //Output: [1,2]
    //Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
    _do_test("ex_1", &[2, 7, 11, 15], 9, &[1, 2]);

    //Example 2:
    //Input: numbers = [2,3,4], target = 6
    //Output: [1,3]
    //Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
    _do_test("ex_2", &[2, 3, 4], 6, &[1, 3]);

    //Example 3:
    //Input: numbers = [-1,0], target = -1
    //Output: [1,2]
    //Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
    _do_test("ex_3", &[-1, 0], -1, &[1, 2]);
}