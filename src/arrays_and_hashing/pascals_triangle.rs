// Given an integer numRows, return the first numRows of Pascal's triangle.
//
// In Pascal's triangle, each number is the sum of the two numbers directly 
// above it as shown:
//
// Constraints:
// 1 <= numRows <= 30



pub fn generate(num_rows: i32) -> Vec<Vec<i32>>{
    let mut container = Vec::new();

    for row in 0..num_rows{
        if row == 0{
            container.push(vec![1]);
        }
        else{
            let last_row = (row - 1) as usize;
            
            let mut row_vec = Vec::new();

            for column in 0..=row{
                if column == 0{ // start of row is always 1
                    row_vec.push(1);
                }
                else if column < row{// 
                    row_vec.push(
                        container[last_row][(column - 1) as usize] 
                        + container[last_row][column as usize]
                    );
                }
                else{   // end of row is always 1
                    row_vec.push(1);
                }
            }

            container.push(row_vec);
        }
    }

    container
}



fn _do_test(num_rows: i32, expected: Vec<Vec<i32>>){
    let result = generate(num_rows);
    assert!(
        result == expected,
        "\nInput = {num_rows:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: numRows = 5, Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
    _do_test(
        5, vec![
            vec![1], 
            vec![1, 1], 
            vec![1, 2, 1], 
            vec![1, 3, 3, 1], 
            vec![1, 4, 6, 4, 1]
        ]
    );
}

#[test]
fn example_2(){
    //Input: numRows = 1, Output: [[1]]
    _do_test(1, vec![vec![1]]);
}