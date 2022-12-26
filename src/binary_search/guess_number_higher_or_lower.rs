// 347. Guess Number Higher or Lower
//
// We are playing the Guess Game. The game is as follows:
// I pick a number from 1 to n. You have to guess which number I picked.
// Every time you guess wrong, I will tell you whether the number I picked is 
// higher or lower than your guess.
//
// You call a pre-defined API int guess(int num), which returns three possible 
// results:
//     -1: Your guess is higher than the number I picked (i.e. num > pick).
//     1: Your guess is lower than the number I picked (i.e. num < pick).
//     0: your guess is equal to the number I picked (i.e. num == pick).
//     Return the number that I picked.
//
// Constraints:
//     1 <= n <= 231 - 1
//     1 <= pick <= n





/*
Forward declaration of guess API.
@param  num   your guess
@return     -1 if num is higher than the picked number
             1 if num is lower than the picked number
             otherwise return 0
unsafe fn guess(num: i32) -> i32 {}
*/
//leetcode specified function signature
//unsafe fn guessNumber(n: i32) -> i32{}






//fn _do_test(){}
//
//#[test]
//fn example_1(){
//    //Input: n = 10, pick = 6, Output: 6
//    _do_test();
//}
//
//#[test]
//fn example_2(){
//    //Input: n = 1, pick = 1, Output: 1
//}
//
//#[test]
//fn example_3(){
//    //Input: n = 2, pick = 1, Output: 1
//}