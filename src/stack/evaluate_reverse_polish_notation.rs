// Evaluate the value of an arithmetic expression in Reverse Polish Notation.
// Valid operators are +, -, *, and /. Each operand may be an integer or 
// another expression.
// Note that division between two integers should truncate toward zero.
// It is guaranteed that the given RPN expression is always valid. That means 
// the expression would always evaluate to a result, and there will not be any 
// division by zero operation.

//Constraints:
//    1 <= tokens.length <= 104
//    tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the range [-200, 200].





pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for string in tokens.iter(){
        match string.as_str(){
            "+" => {
                let second_num = stack.pop().unwrap();
                let first_num = stack.pop().unwrap();

                stack.push(first_num + second_num);
            },
            "-" => {
                let second_num = stack.pop().unwrap();
                let first_num = stack.pop().unwrap();

                stack.push(first_num - second_num);
            },
            "*" => {
                let second_num = stack.pop().unwrap();
                let first_num = stack.pop().unwrap();

                stack.push(first_num * second_num);
            },
            "/" => {
                let second_num = stack.pop().unwrap();
                let first_num = stack.pop().unwrap();
                
                stack.push(first_num / second_num);
            },
            _ => {
                stack.push(string.parse::<i32>().unwrap());
            }
        }
    }

    stack.pop().unwrap()
}





fn _do_test(example: &str, tokens: &[String], expected: i32){
    let result = eval_rpn(tokens.to_vec());
    assert!(
        result == expected,
        "\n{example:?}: input = {tokens:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1:
    //Input: tokens = ["2","1","+","3","*"]
    //Output: 9
    //Explanation: ((2 + 1) * 3) = 9
    _do_test(
        "ex_1", 
        &["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()], 
        9
    );

    //Example 2:
    //Input: tokens = ["4","13","5","/","+"]
    //Output: 6
    //Explanation: (4 + (13 / 5)) = 6
    _do_test(
        "ex_2", 
        &["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()], 
        6
    );

    //Example 3:
    //Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
    //Output: 22
    //Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
    //= ((10 * (6 / (12 * -11))) + 17) + 5
    //= ((10 * (6 / -132)) + 17) + 5
    //= ((10 * 0) + 17) + 5
    //= (0 + 17) + 5
    //= 17 + 5
    //= 22
    _do_test(
        "ex_3", 
        &["10".to_string(), "6".to_string(), "9".to_string(), "3".to_string(), "+".to_string(), 
            "-11".to_string(), "*".to_string(), "/".to_string(), "*".to_string(), "17".to_string(), 
            "+".to_string(), "5".to_string(), "+".to_string()], 
        22
    );
}