// 14. Longest Common Prefix

// Write a function to find the longest common prefix string amongst an array 
// of strings.

// If there is no common prefix, return an empty string "".

//Constraints:
//    1 <= strs.length <= 200
//    0 <= strs[i].length <= 200
//    strs[i] consists of only lowercase English letters.





pub fn longest_common_prefix(_strs: Vec<String>) -> String{
    "".to_string()
}





fn _do_test(strs: &[String], expected: &str){
    let result = longest_common_prefix(strs.to_vec());
    assert!(
        &result == expected,
        "\nInput = {strs:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: strs = ["flower","flow","flight"], Output: "fl"
    _do_test(
        &[
            "flower".to_string(), 
            "flow".to_string(), 
            "flight".to_string()
        ], 
        "fl"
    );
}

#[test]
fn example_2(){
    //Example 2: Input: strs = ["dog","racecar","car"], Output: ""
    //Explanation: There is no common prefix among the input strings.
    _do_test(
        &[
            "dog".to_string(), 
            "racecar".to_string(), 
            "car".to_string()
        ], 
        ""
    );
}