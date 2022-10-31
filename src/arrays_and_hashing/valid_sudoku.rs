// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be 
// validated according to the following rules:
//     Each row must contain the digits 1-9 without repetition.
//     Each column must contain the digits 1-9 without repetition.
//     Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 
//     without repetition.
// 
// Note:
//     A Sudoku board (partially filled) could be valid but is not necessarily 
//     solvable.
//     Only the filled cells need to be validated according to the mentioned rules.

//Constraints:
//    board.length == 9
//    board[i].length == 9
//    board[i][j] is a digit 1-9 or '.'.





pub fn is_valid_sudoku(_board: Vec<Vec<char>>) -> bool {
    false
}








fn _do_test(example: &str, board: &[Vec<char>], expected: bool){
    let result = is_valid_sudoku(board.to_vec());
    assert!(
        result == expected,
        "\n{example:?}: Expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1:
    //Input: board = 
    //[["5","3",".",".","7",".",".",".","."]
    //,["6",".",".","1","9","5",".",".","."]
    //,[".","9","8",".",".",".",".","6","."]
    //,["8",".",".",".","6",".",".",".","3"]
    //,["4",".",".","8",".","3",".",".","1"]
    //,["7",".",".",".","2",".",".",".","6"]
    //,[".","6",".",".",".",".","2","8","."]
    //,[".",".",".","4","1","9",".",".","5"]
    //,[".",".",".",".","8",".",".","7","9"]]
    //Output: true
    _do_test(
        "ex_1", 
        &[
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ], 
        true
    );

    //Example 2:
    //Input: board = 
    //[["8","3",".",".","7",".",".",".","."]
    //,["6",".",".","1","9","5",".",".","."]
    //,[".","9","8",".",".",".",".","6","."]
    //,["8",".",".",".","6",".",".",".","3"]
    //,["4",".",".","8",".","3",".",".","1"]
    //,["7",".",".",".","2",".",".",".","6"]
    //,[".","6",".",".",".",".","2","8","."]
    //,[".",".",".","4","1","9",".",".","5"]
    //,[".",".",".",".","8",".",".","7","9"]]
    //Output: false
    //Explanation: Same as Example 1, except with the 5 in the top left corner 
    //being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it 
    //is invalid.
    _do_test(
        "ex_2", 
        &[
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ], 
    false
    );
}