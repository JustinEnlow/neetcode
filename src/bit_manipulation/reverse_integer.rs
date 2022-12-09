// 7. Reverse Integer

// Given a signed 32-bit integer x, return x with its digits reversed. If 
// reversing x causes the value to go outside the signed 32-bit integer range 
// [-231, 231 - 1], then return 0.

// Assume the environment does not allow you to store 64-bit integers (signed 
// or unsigned).

//Constraints:
//    -231 <= x <= 231 - 1 



pub fn reverse(x: i32) -> i32{
    x
}



fn _do_test(x: i32, expected: i32){
    let result = reverse(x);
    assert!(
        result == expected,
        "\nInput = {x:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: x = 123, Output: 321
    _do_test(123, 321);
}

#[test]
fn example_2(){
    //Example 2: Input: x = -123, Output: -321
    _do_test(-123, -321);
}

#[test]
fn example_3(){
    //Example 3: Input: x = 120, Output: 21
    _do_test(120, 021);
}