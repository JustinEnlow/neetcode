// Given two strings s and t of lengths m and n respectively, return the 
// minimum window substring of s such that every character in t 
// (including duplicates) is included in the window. If there is no such 
// substring, return the empty string "".
// The testcases will be generated such that the answer is unique.

//Constraints:
//    m == s.length
//    n == t.length
//    1 <= m, n <= 105
//    s and t consist of uppercase and lowercase English letters.

//Follow up: Could you find an algorithm that runs in O(m + n) time?





pub fn min_window(s: String, t: String) -> String{
    if t.len() > s.len(){
        return "".to_string();
    }

    let last_index = s.len() - 1;

    let mut window_start = 0;
    let mut window_end = 1;

    let mut shortest_substring = &s[..];
    let mut min_range = last_index - window_start;

    loop{
        if window_start >= last_index{break;}

        let substring = &s[window_start..=window_end];

        if contains_t_chars(substring, &t){
            if window_end - window_start <= min_range{
                shortest_substring = &s[window_start..=window_end];
                min_range = window_end - window_start;
            }
            window_start += 1;
        }
        else{
            if !(window_end + 1 > last_index){
                window_end += 1;
            }
            else{
                break;
            }
        }
    }

    shortest_substring.to_string()
}

fn contains_t_chars(substring: &str, t: &str) -> bool{
    for char in t.chars(){
        if !substring.contains(char){
            return false;
        }
    }

    true
}





fn _do_test(s: &str, t: &str, expected: &str){
    let result = min_window(s.to_string(), t.to_string());
    assert!(
        result == expected.to_string(),
        "\nInput = {s:?} and {t:?}, expected = {expected:?} but \
        got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: s = "ADOBECODEBANC", t = "ABC", Output: "BANC"
    //Explanation: The minimum window substring "BANC" includes 'A', 'B', and 
    //'C' from string t.
    _do_test("ADOBECODEBANC", "ABC", "BANC");
}

#[test]
fn example_2(){
    //Example 2: Input: s = "a", t = "a", Output: "a"
    //Explanation: The entire string s is the minimum window.
    _do_test("a", "a", "a");
}

#[test]
fn example_3(){
    //Example 3: Input: s = "a", t = "aa", Output: ""
    //Explanation: Both 'a's from t must be included in the window.
    //Since the largest window of s only has one 'a', return empty string.
    _do_test("a", "aa", "");
}