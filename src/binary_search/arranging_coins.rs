// 441. Arranging Coins
//
// You have n coins and you want to build a staircase with these coins. The 
// staircase consists of k rows where the ith row has exactly i coins. The last 
// row of the staircase may be incomplete.
//
// Given the integer n, return the number of complete rows of the staircase you 
// will build.
//
// Constraints:
//     1 <= n <= 231 - 1





pub fn arrange_coins(n: i32) -> i32{
    let mut n = n;
    let mut num_rows = 0;
    
    while n != 1{
        n = n / 2;
        num_rows += 1;
    }

    num_rows
}





fn _do_test(n: i32, expected: i32){
    let result = arrange_coins(n);
    assert!(
        result == expected,
        "\nInput = {n:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: n = 5, Output: 2
    //Explanation: Because the 3rd row is incomplete, we return 2.
    _do_test(5, 2);
}

#[test]
fn example_2(){
    //Input: n = 8, Output: 3
    //Explanation: Because the 4th row is incomplete, we return 3.
    _do_test(8, 3);
}