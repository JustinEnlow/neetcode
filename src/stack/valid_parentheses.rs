//Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
//An input string is valid if:
//    Open brackets must be closed by the same type of brackets.
//    Open brackets must be closed in the correct order.
//    Every close bracket has a corresponding open bracket of the same type.

//Constraints:
//    1 <= s.length <= 104
//    s consists of parentheses only '()[]{}'.





pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for char in s.chars(){
        match char{
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}' | ')' | ']' => {
                match stack.pop(){
                    None => return false,
                    Some(val) => {
                        if val != char{
                            return false;
                        }
                    }
                }
            },
            _ => {}
        }
    }

    stack.is_empty()
}





fn _do_test(example: &str, s: &str, expected: bool){
    let result = is_valid(s.to_string());
    assert!(
        result == expected,
        "\n{example:?}: input = {s:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1: Input: s = "()", Output: true
    _do_test("ex_1", "()", true);

    //Example 2: Input: s = "()[]{}", Output: true
    _do_test("ex_2", "()[]{}", true);

    //Example 3: Input: s = "(]", Output: false
    _do_test("ex_3", "(]", false);
}