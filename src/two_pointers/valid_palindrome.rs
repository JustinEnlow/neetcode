// A phrase is a palindrome if, after converting all uppercase letters into 
// lowercase letters and removing all non-alphanumeric characters, it reads the 
// same forward and backward. Alphanumeric characters include letters and numbers.

// Given a string s, return true if it is a palindrome, or false otherwise.

//Constraints:
//    1 <= s.length <= 2 * 105
//    s consists only of printable ASCII characters.





pub fn is_palindrome(s: String) -> bool {
    if s.is_empty(){
        return true;
    }

    //remove non alphanumeric characters
    let mut alphanum_string = String::new();
    for char in s.chars(){
        match char::is_alphanumeric(char){
            true => {
                alphanum_string.push(char);
            },
            false => {}
        }
    }
    //println!("{}", new_string);

    //convert any capital to lower
    let lower = alphanum_string.to_lowercase();

    //construct new string by reversing old string
    let mut rev_string = String::new();
    for char in lower.chars().rev(){
        rev_string.push(char);
    }
    //println!("{}", &rev_string);

    //if same, return true
    if rev_string == lower{
        return true;
    }
    
    false
}





fn _do_test(example: &str, s: &str, expected: bool){
    let result = is_palindrome(s.to_string());
    assert!(
        result == expected,
        "\n{example:?}: input = {s:?}, Expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1:
    //Input: s = "A man, a plan, a canal: Panama"
    //Output: true
    //Explanation: "amanaplanacanalpanama" is a palindrome.
    _do_test("ex_1", "A man, a plan, a canal: Panama", true);

    //Example 2:
    //Input: s = "race a car"
    //Output: false
    //Explanation: "raceacar" is not a palindrome.
    _do_test("ex_2", "race a car", false);

    //Example 3:
    //Input: s = " "
    //Output: true
    //Explanation: s is an empty string "" after removing non-alphanumeric characters.
    //Since an empty string reads the same forward and backward, it is a palindrome.
    _do_test("ex_3", " ", true);
}