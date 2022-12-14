// 392. Is Subsequence
//
// Given two strings s and t, return true if s is a subsequence of t, or false 
// otherwise.
//
// A subsequence of a string is a new string that is formed from the original 
// string by deleting some (can be none) of the characters without disturbing 
// the relative positions of the remaining characters. (i.e., "ace" is a 
// subsequence of "abcde" while "aec" is not).
//
// Constraints:
//     0 <= s.length <= 100
//     0 <= t.length <= 104
//     s and t consist only of lowercase English letters.





pub fn is_subsequence(s: String, t: String) -> bool{
    let mut new_t = String::new();

    for s_char in s.chars(){
        for t_char in t.chars(){
            if t_char == s_char{
                new_t.push(t_char);
            }
        }
    }

    if new_t == s{
        return true;
    }
    
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
    //Input: s = "abc", t = "ahbgdc", Output: true
    _do_test("abc", "ahbgdc", true);
}

#[test]
fn example_2(){
    //Input: s = "axc", t = "ahbgdc", Output: false
    _do_test("axc", "ahbgdc", false);
}