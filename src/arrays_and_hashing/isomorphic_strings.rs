// 205. Isomorphic Strings
//
// Given two strings s and t, determine if they are isomorphic.
//
// Two strings s and t are isomorphic if the characters in s can be replaced to 
// get t.
//
// All occurrences of a character must be replaced with another character while 
// preserving the order of characters. No two characters may map to the same 
// character, but a character may map to itself.
//
// Constraints:
//     1 <= s.length <= 5 * 104
//     t.length == s.length
//     s and t consist of any valid ascii character.


pub fn is_isomorphic(_s: String, _t: String) -> bool{
    false
}


fn _do_test(s: &str, t: &str, expected: bool){
    let result = is_isomorphic(s.to_string(), t.to_string());
    assert!(
        result == expected,
        "\nInput = {s:?} and {t:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: s = "egg", t = "add"
    //Output: true
    _do_test("egg", "add", true);
}

#[test]
fn example_2(){
    //Input: s = "foo", t = "bar"
    //Output: false
    _do_test("foo", "bar", false);
}

#[test]
fn example_3(){
    //Input: s = "paper", t = "title"
    //Output: true
    _do_test("paper", "title", true);
}