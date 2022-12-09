// 392. Is Subsequence

// Given two strings s and t, return true if s is a subsequence of t, or false 
// otherwise.

// A subsequence of a string is a new string that is formed from the original 
// string by deleting some (can be none) of the characters without disturbing 
// the relative positions of the remaining characters. (i.e., "ace" is a 
// subsequence of "abcde" while "aec" is not).

//Constraints:
//    0 <= s.length <= 100
//    0 <= t.length <= 104
//    s and t consist only of lowercase English letters.





pub fn is_subsequence(_s: String, _t: String) -> bool{
    false
}





fn _do_test(s: &str, t: &str, expected: bool){
    let result = is_subsequence(s.to_string(), t.to_string());
    assert!(
        result == expected,
        "\nInput = {s:?} and {t:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: s = "abc", t = "ahbgdc", Output: true
    _do_test("abc", "ahbgdc", true);
}

#[test]
fn example_2(){
    //Example 2: Input: s = "axc", t = "ahbgdc", Output: false
    _do_test("axc", "ahbgdc", false);
}