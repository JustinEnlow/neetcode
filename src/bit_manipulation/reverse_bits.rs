// 190. Reverse Bits

// Reverse bits of a given 32 bits unsigned integer.

// Note:
//     Note that in some languages, such as Java, there is no unsigned integer 
//     type. In this case, both input and output will be given as a signed 
//     integer type. They should not affect your implementation, as the 
//     integer's internal binary representation is the same, whether it is 
//     signed or unsigned.

//     In Java, the compiler represents the signed integers using 2's 
//     complement notation. Therefore, in Example 2 above, the input represents 
//     the signed integer -3 and the output represents the signed integer 
//     -1073741825.

//Constraints:
//    The input must be a binary string of length 32


use core::ops::{BitOr, Shl, Shr, Rem};


pub fn reverse_bits(x: u32) -> u32 {
    //x.reverse_bits()  // cheat rust solution

    let mut x = x;
    let mut result = 0;

    for num in 0..32{
        let last = Rem::rem(x, 2); //x % 2;
        x = Shr::shr(x, 1); //x = x >> 1;
        result = BitOr::bitor(result, Shl::shl(last, 31 - num)/*last << (31 - num)*/);
    }

    result
}



fn _do_test(example: &str, n: u32, expected: u32){
    let result = reverse_bits(n);
    assert!(
        result == expected,
        "\n{example:?}: input = {n:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1:
    //Input: n = 00000010100101000001111010011100
    //Output:    964176192 (00111001011110000010100101000000)
    //Explanation: The input binary string 00000010100101000001111010011100 
    //represents the unsigned integer 43261596, so return 964176192 which its 
    //binary representation is 00111001011110000010100101000000.
    _do_test("ex_1", 0b00000010100101000001111010011100, 964176192);

    //Example 2:
    //Input: n = 11111111111111111111111111111101
    //Output:   3221225471 (10111111111111111111111111111111)
    //Explanation: The input binary string 11111111111111111111111111111101 
    //represents the unsigned integer 4294967293, so return 3221225471 which its 
    //binary representation is 10111111111111111111111111111111.
    _do_test("ex_2", 0b11111111111111111111111111111101, 3221225471);
}