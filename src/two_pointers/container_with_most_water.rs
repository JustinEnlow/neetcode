// You are given an integer array height of length n. There are n vertical 
// lines drawn such that the two endpoints of the ith line are (i, 0) and 
// (i, height[i]).
// Find two lines that together with the x-axis form a container, such that the 
// container contains the most water.
// Return the maximum amount of water a container can store.
// Notice that you may not slant the container.

//Constraints:
//    n == height.length
//    2 <= n <= 105
//    0 <= height[i] <= 104





pub fn max_area(height: Vec<i32>) -> i32{
    let mut largest_area = 0;
    let mut left = 0;
    let mut right = 1;

    let last_index = height.len() - 1;

    while left < last_index{
        let usable_height = if height[left] < height[right]{
            height[left]
        }
        else{
            height[right]
        };
        let width = right - left;

        let potential_largest = usable_height * width as i32;

        largest_area = if potential_largest > largest_area{
            potential_largest
        }
        else{
            largest_area
        };

        if right < last_index{
            right += 1;
        }
        else{
            left += 1;
            right = left + 1;
        }
    }

    largest_area
}





fn _do_test(height: &[i32], expected: i32){
    let result = max_area(height.to_vec());
    assert!(
        result == expected,
        "\nInput = {height:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: height = [1,8,6,2,5,4,8,3,7], Output: 49
    //Explanation: The above vertical lines are represented by array 
    //[1,8,6,2,5,4,8,3,7]. In this case, the max area of water 
    //(blue section) the container can contain is 49.
    _do_test(&[1, 8, 6, 2, 5, 4, 8, 3, 7], 49);
}

#[test]
fn example_2(){
    //Example 2: Input: height = [1,1], Output: 1
    _do_test(&[1, 1], 1);
}