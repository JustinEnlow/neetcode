// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
//
// An Anagram is a word or phrase formed by rearranging the letters of a 
// different word or phrase, typically using all the original letters exactly once.
//
// Constraints:
//     1 <= s.length, t.length <= 5 * 104
//     s and t consist of lowercase English letters.


use std::collections::HashMap;


pub fn is_anagram(s: String, t: String) -> bool {
    ///////////////////////////////////////////////////////////////////////////
    //quicker, but more memory used
    //if the words have different lengths, they def cant be anagrams
    if s.len() != t.len(){
        return false;
    }
    let hashmap_1 = build_hashmap_from_str(&s);
    let hashmap_2 = build_hashmap_from_str(&t);
    if hashmap_1 == hashmap_2{
        return true;
    }
    false
    ///////////////////////////////////////////////////////////////////////////
    


    ///////////////////////////////////////////////////////////////////////////
    //slower, but less memory use?
    //if s.len() != t.len(){
    //    return false;
    //}
    //
    //let mut s: Vec<char> = s.chars().collect();
    //s.sort();
    //
    //let mut t: Vec<char> = t.chars().collect();
    //t.sort();
    //
    //if s == t{
    //    return true
    //}
    //false
    ///////////////////////////////////////////////////////////////////////////
}

fn build_hashmap_from_str(s: &str) -> HashMap<char, i32>{
    let mut hashmap = HashMap::new();
    for char in s.chars(){
        if !hashmap.contains_key(&char){
            hashmap.insert(char, 1);
        }
        else{
            let new_value = hashmap.get(&char).unwrap() + 1;
            hashmap.insert(char, new_value);
        }
    }
    hashmap
}





fn _do_test(s: &str, t: &str, expected: bool){
    let result = is_anagram(s.to_string(), t.to_string());
    assert!(
        result == expected,
        "\ninput = {s:?} and {t:?}, Expected = {expected:?} but got {result:?}\n"
    );
}

#[test]
fn example_1(){
    //Input: s = "anagram", t = "nagaram", Output: true
    _do_test("anagram", "nagaram", true);
}

#[test]
fn example_2(){
    //Input: s = "rat", t = "car", Output: false
    _do_test("rat", "car", false);
}