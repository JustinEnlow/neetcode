//You are given an array prices where prices[i] is the price of a given stock 
//on the ith day.

//You want to maximize your profit by choosing a single day to buy one stock 
//and choosing a different day in the future to sell that stock.

//Return the maximum profit you can achieve from this transaction. If you 
//cannot achieve any profit, return 0.

//Constraints:
//    1 <= prices.length <= 105
//    0 <= prices[i] <= 104





pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy_index = 0;
    let mut sell_index = 0;
    let mut max_profit = 0;

    while sell_index < prices.len(){
        if prices[buy_index] < prices[sell_index]{
            let profit = prices[sell_index] - prices[buy_index];
            max_profit = i32::max(max_profit, profit);
        }
        else{
            buy_index = sell_index;
        }
        sell_index += 1;
    }

    max_profit
}




fn _do_test(example: &str, prices: &[i32], expected: i32){
    let result = max_profit(prices.to_vec());
    assert!(
        result == expected,
        "\n{example:?}: input = {prices:?}, expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1:
    //Input: prices = [7,1,5,3,6,4]
    //Output: 5
    //Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
    //Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
    _do_test("ex_1", &[7, 1, 5, 3, 6, 4], 5);

    //Example 2:
    //Input: prices = [7,6,4,3,1]
    //Output: 0
    //Explanation: In this case, no transactions are done and the max profit = 0.
    _do_test("ex_2", &[7, 6, 4, 3, 1], 0);
}