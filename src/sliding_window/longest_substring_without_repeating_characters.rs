//Given a string s, find the length of the longest substring without repeating 
//characters.

//Constraints:
//    0 <= s.length <= 5 * 104
//    s consists of English letters, digits, symbols and spaces.





use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hashmap = HashMap::new();
    let mut start = 0;
    let mut longest_substring = 0;
    
    for (i, char) in s.chars().enumerate(){
        //match hashmap.get(&char){
        //    Some(&val) => {
        //        start = if val > start{val}
        //                else{start};
        //    },
        //    None => {}
        //}
        if let Some(&val) = hashmap.get(&char){
            start = if val > start{val}else{start};
        }
        
        let potential_longest = (i + 1) - start;
        longest_substring = usize::max(longest_substring, potential_longest);
    
        hashmap.insert(char, i + 1);
    }
    
    longest_substring as i32
}





fn _do_test(s: &str, expected: i32){
    let result = length_of_longest_substring(s.to_string());
    assert!(
        result == expected,
        "\ninput = {s:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: s = "abcabcbb", Output: 3
    //Explanation: The answer is "abc", with the length of 3.
    _do_test("abcabcbb", 3);
}

#[test]
fn example_2(){
    //Example 2: Input: s = "bbbbb", Output: 1
    //Explanation: The answer is "b", with the length of 1.
    _do_test("bbbbb", 1);
}

#[test]
fn example_3(){
    //Example 3: Input: s = "pwwkew", Output: 3
    //Explanation: The answer is "wke", with the length of 3.
    //Notice that the answer must be a substring, "pwke" is a subsequence and 
    //not a substring.
    _do_test("pwwkew", 3);
}