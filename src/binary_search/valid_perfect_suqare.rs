// 367. Valid Perfect Square

// Given a positive integer num, write a function which returns True if num is 
// a perfect square else False.

// Follow up: Do not use any built-in library function such as sqrt.

// Constraints:
//     1 <= num <= 2^31 - 1



pub fn is_perfect_square(num: i32) -> bool{
    let factors = determine_factors(num);
    for factor in factors{
        if factor * factor == num{
            return true;
        }
    }

    false
}

fn determine_factors(num: i32) -> Vec<i32>{
    let mut factors = Vec::new();
    for x in 1..=num{
        if num % x == 0{
            factors.push(x);
        }
    }

    factors
}



fn _do_test(num: i32, expected: bool){
    let result = is_perfect_square(num);
    assert!(
        result == expected,
        "\nInput = {num:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: num = 16, Output: true
    _do_test(16, true);
}

#[test]
fn example_2(){
    //Input: num = 14, Output: false
    _do_test(14, false);
}