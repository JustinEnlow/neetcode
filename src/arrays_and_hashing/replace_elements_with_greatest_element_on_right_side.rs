// 1299. Replace Elements with Greatest Element on Right Side

// Given an array arr, replace every element in that array with the greatest 
// element among the elements to its right, and replace the last element with -1.

//After doing so, return the array.

//Constraints:
//    1 <= arr.length <= 104
//    1 <= arr[i] <= 105





pub fn replace_elements(arr: Vec<i32>) -> Vec<i32>{
    // figure out how to do this with for loop. was having trouble mutating values in arr when borrowing arr for the loop
    let mut arr = arr;
    
    let mut i = 0;
    loop{
        if i == arr.len() - 1{
            arr[i] = -1;
            break;
        }

        let mut index_of_largest = 0;
        let mut largest = 0;
        //for j in i + 1..arr.len(){
        for (j, _) in arr.iter().enumerate().skip(i + 1){
            if arr[j] > largest{
                largest = arr[j];
                index_of_largest = j;
            }
        }

        arr[i] = arr[index_of_largest];

        i += 1;
    }
    
    arr
}





fn _do_test(arr: &[i32], expected: &[i32]){
    let result = replace_elements(arr.to_vec());
    assert!(
        result == expected,
        "\nInput = {arr:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Example 1: Input: arr = [17,18,5,4,6,1], Output: [18,6,6,6,1,-1]
    //Explanation: 
    //- index 0 --> the greatest element to the right of index 0 is index 1 (18).
    //- index 1 --> the greatest element to the right of index 1 is index 4 (6).
    //- index 2 --> the greatest element to the right of index 2 is index 4 (6).
    //- index 3 --> the greatest element to the right of index 3 is index 4 (6).
    //- index 4 --> the greatest element to the right of index 4 is index 5 (1).
    //- index 5 --> there are no elements to the right of index 5, so we put -1.
    _do_test(&[17, 18, 5, 4, 6, 1], &[18, 6, 6, 6, 1, -1]);
}

#[test]
fn example_2(){
    //Example 2: Input: arr = [400], Output: [-1]
    //Explanation: There are no elements to the right of index 0.
    _do_test(&[400], &[-1]);
}