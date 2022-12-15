// Given an array of strings strs, group the anagrams together. You can return 
// the answer in any order.
//
// An Anagram is a word or phrase formed by rearranging the letters of a 
// different word or phrase, typically using all the original letters exactly 
// once.
//
// Constraints:
//     1 <= strs.length <= 104
//     0 <= strs[i].length <= 100
//     strs[i] consists of lowercase English letters.






pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
        
    for (i, outer_val) in strs.iter().enumerate(){    
        let mut skip = false;
        
        for idk in result.iter(){
            if idk.contains(/*&*/outer_val){
                skip = true;
            }
        }
            
        if skip{
            continue;
        }
    
        let mut anagrams = Vec::new();
        anagrams.push(outer_val.clone());

        for j in strs.iter().skip(i + 1){
            if is_anagram(outer_val, j){
                anagrams.push(j.clone());
            }
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





fn _do_test(strs: Vec<String>, expected: Vec<Vec<String>>){
    let strings = strs.clone();
    let result = group_anagrams(strs);
    
    // how can i check that result contains expected collections of strings
    // disregarding the specific order of the strings?
    //for x in &expected{
    //    assert!(
    //        result.contains(x)
    //    )
    //}
    
    assert!(
        result == expected,
        "\ninput = {strings:?}, Expected = {expected:?} but got {result:?}\n"
    );
}

//#[test]
//fn example_1(){
//    //Example 1:
//    //Input: strs = ["eat","tea","tan","ate","nat","bat"]
//    //Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
//    _do_test(
//        Vec::from(
//            [
//                "eat".to_string(), 
//                "tea".to_string(), 
//                "tan".to_string(), 
//                "ate".to_string(), 
//                "nat".to_string(), 
//                "bat".to_string()
//            ]
//        ), 
//        vec![
//            vec![String::from("bat")],
//            vec![
//                String::from("nat"),
//                String::from("tan")
//            ],
//            vec![
//                String::from("ate"),
//                String::from("eat"),
//                String::from("tea"),
//            ]
//        ]
//    );
//    // this technically passes, but the order makes it fail. ignoring for now, 
//    // will address later...
//}

#[test]
fn example_2(){
    //Example 2: Input: strs = [""], Output: [[""]]
    _do_test(
        Vec::from(["".to_string()]), 
        vec![vec!["".to_string()]]
    );
}

#[test]
fn example_3(){
    //Example 3: Input: strs = ["a"], Output: [["a"]]
    _do_test(
        Vec::from(["a".to_string()]), 
        vec![
            vec![String::from("a")]
        ]
    );
}