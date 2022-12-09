// 191. Number of 1 bits

// Write a function that takes an unsigned integer and returns the number of 
// '1' bits it has (also known as the Hamming weight).
// Note:
//     Note that in some languages, such as Java, there is no unsigned integer 
//     type. In this case, the input will be given as a signed integer type. It should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
//     In Java, the compiler represents the signed integers using 2's 
//     complement notation. Therefore, in Example 3, the input represents the signed integer. -3.

//Constraints:
//    The input must be a binary string of length 32.



use core::ops::BitAnd;
// use core::ops::Rem; // can use Rem::rem() in place of % operator



pub fn hamming_weight (n: u32) -> i32 {
    /////////////////// My ignorant solution //////////////////////////////////
    //let mut one_count = 0;
    //
    //let n_bit_string = format!("{:032b}", n);
    //
    //for char in n_bit_string.chars(){
    //    println!("{}", char);
    //    match char{
    //        '1' => {
    //            one_count = one_count + 1;
    //        },
    //        _ => {}
    //    }
    //}
    //
    //one_count

    ////////////////////// Solution 1 from neetcode ///////////////////////////
    //let mut m = n; // only needed because n in function sig is not mutable. this is defined by leetcode, so we are working around it
    //let mut one_count = 0;
    //while m > 0{
    //    one_count = one_count + m % 2; // if last bit is 1, % 2 will return 1, if last bit is 0, % 2 will return 0
    //    m = m >> 1; // shift right so last bit is discarded
    //}
    //
    //one_count as i32

    ///////////////////// Solution 2 from neetcode ////////////////////////////
    let mut m = n;
    let mut one_count = 0;
    while m > 0{
        m = BitAnd::bitand(m, m - 1);    //m = m & (m - 1);
        one_count = one_count + 1;
    }

    one_count as i32
}





fn _do_test(n: u32, expected: i32){
    let result = hamming_weight(n);
    assert!(
        result == expected,
        "\ninput = {n:032b}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1:
    //Input: n = 00000000000000000000000000001011
    //Output: 3
    //Explanation: The input binary string 00000000000000000000000000001011 has a total of three '1' bits.
    _do_test(0b00000000000000000000000000001011, 3);
}

#[test]
fn example_2(){
    //Example 2:
    //Input: n = 00000000000000000000000010000000
    //Output: 1
    //Explanation: The input binary string 00000000000000000000000010000000 has a total of one '1' bit.
    _do_test(0b00000000000000000000000010000000, 1);
}

#[test]
fn example_3(){
    //Example 3:
    //Input: n = 11111111111111111111111111111101
    //Output: 31
    //Explanation: The input binary string 11111111111111111111111111111101 has a total of thirty one '1' bits.
    _do_test(0b11111111111111111111111111111101, 31);
}