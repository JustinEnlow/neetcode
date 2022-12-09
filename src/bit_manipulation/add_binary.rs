// 67. Add Binary

// Given two binary strings a and b, return their sum as a binary string.

//Constraints:
//    1 <= a.length, b.length <= 104
//    a and b consist only of '0' or '1' characters.
//    Each string does not contain leading zeros except for the zero itself. 





pub fn add_binary(a: String, _b: String) -> String{
    a
}





fn _do_test(a: &str, b: &str, expected: &str){
    let result = add_binary(a.to_string(), b.to_string());
    assert!(
        &result == expected,
        "\nInput = {a:?} and {b:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: a = "11", b = "1", Output: "100"
    _do_test("11", "1", "100");
}

#[test]
fn example_2(){
    //Example 2: Input: a = "1010", b = "1011", Output: "10101"
    _do_test("1010", "1011", "10101");
}