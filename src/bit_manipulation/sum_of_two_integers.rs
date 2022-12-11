// 371. Sum of Two Integers

// Given two integers a and b, return the sum of the two integers without using 
// the operators + and -.

//Constraints:
//    -1000 <= a, b <= 1000





pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    loop{   // loop until no carries left
        let carry = (a & b) << 1;   //(a bitwise_and b) left_shift 1
        a ^= b; //a = a ^ b;  //xor
        b = carry;

        if b == 0{break;}
    }

    a
}





fn _do_test(a: i32, b: i32, expected: i32){
    let result = get_sum(a, b);
    assert!(
        result == expected,
        "\nInput = {a:?} and {b:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: a = 1, b = 2, Output: 3
    _do_test(1, 2, 3);
}

#[test]
fn example_2(){
    //Example 2: Input: a = 2, b = 3, Output: 5
    _do_test(2, 3, 5);
}