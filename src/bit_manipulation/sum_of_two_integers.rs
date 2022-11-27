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
        a = a ^ b;  //xor
        b = carry;

        if b == 0{break;}
    }

    a
}





fn _do_test(example: &str, a: i32, b: i32, expected: i32){
    let result = get_sum(a, b);
    assert!(
        result == expected,
        "\n{example:?}: Input = {a:?} and {b:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1: Input: a = 1, b = 2, Output: 3
    _do_test("ex_1", 1, 2, 3);

    //Example 2: Input: a = 2, b = 3, Output: 5
    _do_test("ex_2", 2, 3, 5);
}