//You are given a string s and an integer k. You can choose any character of 
//the string and change it to any other uppercase English character. You can 
//perform this operation at most k times.

//Return the length of the longest substring containing the same letter you can
// get after performing the above operations.

//Constraints:
//    1 <= s.length <= 105
//    s consists of only uppercase English letters.
//    0 <= k <= s.length





pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut start = 0;
    let mut longest_substring = 0;
    let mut k_uses = 0;

    for (i, char) in s.chars().enumerate(){
        let start_char = s.chars().nth(start).unwrap();
        let potential_longest = (i + 1) - start;
        
        match (char == start_char, k_uses < k){
            (true, _) => {
                longest_substring = usize::max(longest_substring, potential_longest);
            },
            (false, true) => {
                longest_substring = usize::max(longest_substring, potential_longest);
                k_uses += 1;
            },
            (false, false) => {
                start = i;
                k_uses = 0;
            }
        }
    }

    longest_substring as i32
}





fn _do_test(example: &str, s: &str, k: i32, expected: i32){
    let result = character_replacement(s.to_string(), k);
    assert!(
        result == expected,
        "\n{example:?}: input = {s:?} and {k:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1:
    //Input: s = "ABAB", k = 2
    //Output: 4
    //Explanation: Replace the two 'A's with two 'B's or vice versa.
    _do_test("ex_1", "ABAB", 2, 4);

    //Example 2:
    //Input: s = "AABABBA", k = 1
    //Output: 4
    //Explanation: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
    //The substring "BBBB" has the longest repeating letters, which is 4.
    _do_test("ex_2", "AABABBA", 1, 4);
}