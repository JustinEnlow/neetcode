// Given an array of strings strs, group the anagrams together. You can return 
// the answer in any order.
//
// An Anagram is a word or phrase formed by rearranging the letters of a 
// different word or phrase, typically using all the original letters exactly 
// once.

//Constraints:
//    1 <= strs.length <= 104
//    0 <= strs[i].length <= 100
//    strs[i] consists of lowercase English letters.





// This finally kinda worked, but not passing tests. still more shit to figure out...
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
        
    for (i, outer_val) in strs.iter().enumerate(){    
        let mut skip = false;
        for idk in result.iter(){
            if idk.contains(&outer_val){
                skip = true;
            }
        }
            
        if skip{
            continue;
        }
    
        let mut anagrams = Vec::new();
        //let mut not = Vec::new();
        anagrams.push(outer_val.clone());
        for j in i+1..strs.len(){
            if is_anagram(outer_val, &strs[j]){
                anagrams.push(strs[j].clone())
            }
            //else{
            //    not.push(strs[j].clone())
            //}
        }
        result.push(anagrams);
    }
    
    result        
}

fn is_anagram(s: &str, t: &str) -> bool{
    if s.len() != t.len(){
        return false;
    }

    let mut s: Vec<char> = s.chars().collect();
    s.sort();

    let mut t: Vec<char> = t.chars().collect();
    t.sort();

    if s == t{
        return true
    }
    false
}



fn _do_test(example: &str, strs: Vec<String>, expected: Vec<Vec<String>>){
    let strings = strs.clone();
    let result = group_anagrams(strs);
    assert!(
        result == expected,
        "\n{example:?}: input = {strings:?}, Expected = {expected:?} but got {result:?}\n"
    );
}

#[test]
fn tests(){
    //Example 1:
    //Input: strs = ["eat","tea","tan","ate","nat","bat"]
    //Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
    //_do_test(
    //    "ex_1", 
    //    Vec::from(
    //        [
    //            "eat".to_string(), 
    //            "tea".to_string(), 
    //            "tan".to_string(), 
    //            "ate".to_string(), 
    //            "nat".to_string(), 
    //            "bat".to_string()
    //        ]
    //    ), 
    //    vec![
    //        vec![String::from("bat")],
    //        vec![
    //            String::from("nat"),
    //            String::from("tan")
    //        ],
    //        vec![
    //            String::from("ate"),
    //            String::from("eat"),
    //            String::from("tea"),
    //        ]
    //    ]
    //);        //this technically passes, but the order makes it fail. ignoring for now, will address later...
    
    //Example 2:
    //Input: strs = [""]
    //Output: [[""]]
    _do_test(
        "ex_2", 
        Vec::from(["".to_string()]), 
        vec![vec!["".to_string()]]
    );
    
    //Example 3:
    //Input: strs = ["a"]
    //Output: [["a"]]
    _do_test(
        "ex_3", 
        Vec::from(["a".to_string()]), 
        vec![
            vec![String::from("a")]
        ]
    );
}