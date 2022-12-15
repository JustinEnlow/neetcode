// 680. Valid Palindrome II
//
// Given a string s, return true if the s can be palindrome after deleting at 
// most one character from it.
//
// Constraints:
//     1 <= s.length <= 105
//     s consists of lowercase English letters.


use crate::two_pointers::valid_palindrome;


pub fn valid_palindrome(s: String) -> bool{
    if valid_palindrome::is_palindrome(s.clone()){
        return true;
    }

    for (i, _) in s.chars().enumerate(){
        let mut new_s = String::new();
        
        if i + 1 < s.len(){
            new_s.push_str(&s[..i]);
            new_s.push_str(&s[i+1..]);
        }

        if valid_palindrome::is_palindrome(new_s.clone()) && !new_s.is_empty(){
            return true;
        }
    }

    false
}





fn _do_test(s: &str, expected: bool){
    let result = valid_palindrome(s.to_string());
    assert!(
        result == expected,
        "\nInput = {s:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: s = "aba", Output: true
    _do_test("aba", true);
}

#[test]
fn example_2(){
    //Input: s = "abca", Output: true
    //Explanation: You could delete the character 'c'.
    _do_test("abca", true);
}

#[test]
fn example_3(){
    //Input: s = "abc", Output: false
    _do_test("abc", false);
}