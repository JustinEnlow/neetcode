// 58. Length of Last Word

// Given a string s consisting of words and spaces, return the length of the 
// last word in the string.

// A word is a maximal substring consisting of non-space characters only.

//Constraints:
//    1 <= s.length <= 104
//    s consists of only English letters and spaces ' '.
//    There will be at least one word in s.





pub fn length_of_last_word(s: String) -> i32{
    // "cheat" solution using built in functions
    //let words: Vec<&str> = s.split_whitespace().collect();
    //let last = words.last().unwrap();

    //last.len() as i32
    // or
    //s.split_whitespace().collect::<Vec<&str>>().last().unwrap().len() as i32



    let mut i = s.len() - 1;
    let mut length = 0;
    
    while s.chars().nth(i).unwrap() == ' '{
        i -= 1;
    }
    while i > 0 && s.chars().nth(i).unwrap() != ' '{
        length += 1;
        i -= 1;
    }

    length
}





fn _do_test(s: &str, expected: i32){
    let result = length_of_last_word(s.to_string());
    assert!(
        result == expected,
        "\nInput = {s:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: s = "Hello World", Output: 5
    //Explanation: The last word is "World" with length 5.
    _do_test("Hello World", 5);
}

#[test]
fn example_2(){
    //Example 2: Input: s = "   fly me   to   the moon  ", Output: 4
    //Explanation: The last word is "moon" with length 4.
    _do_test("   fly me   to   the moon  ", 4);
}

#[test]
fn example_3(){
    //Example 3: Input: s = "luffy is still joyboy", Output: 6
    //Explanation: The last word is "joyboy" with length 6.
    _do_test("luffy is still joyboy", 6);
}