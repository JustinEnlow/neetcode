// 338. counting bits
//
// Given an integer n, return an array ans of length n + 1 such that for each 
// i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.
//
// Constraints:
//     0 <= n <= 105



use core::ops::BitAnd;



pub fn count_bits(n: i32) -> Vec<i32> {
    let mut ans = Vec::new();

    for mut x in 0..=n{
        let mut one_count = 0;
        
        while x > 0{
            x = BitAnd::bitand(x, x - 1);    //m = m & (m - 1);
            one_count += 1; //one_count = one_count + 1;
        }

        ans.push(one_count as i32);
    }

    ans
}



fn _do_test(n: i32, expected: &[i32]){
    let result = count_bits(n);
    assert!(
        result == expected,
        "\ninput = {n:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: n = 2, Output: [0,1,1]
    //Explanation: 0 --> 0, 1 --> 1, 2 --> 10
    _do_test(2, &[0, 1, 1]);
}

#[test]
fn example_2(){
    //Input: n = 5, Output: [0,1,1,2,1,2]
    //Explanation: 0 --> 0, 1 --> 1, 2 --> 10, 3 --> 11, 4 --> 100, 5 --> 101
    _do_test(5, &[0, 1, 1, 2, 1, 2]);
}