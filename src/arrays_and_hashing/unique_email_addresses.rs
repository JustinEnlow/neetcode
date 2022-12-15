// Every valid email consists of a local name and a domain name, separated by 
// the '@' sign. Besides lowercase letters, the email may contain one or more 
// '.' or '+'.
//
// For example, in "alice@leetcode.com", "alice" is the local name, and 
// "leetcode.com" is the domain name.
// If you add periods '.' between some characters in the local name part of an 
// email address, mail sent there will be forwarded to the same address without 
// dots in the local name. Note that this rule does not apply to domain names.
//
// For example, "alice.z@leetcode.com" and "alicez@leetcode.com" forward to the 
// same email address.
// If you add a plus '+' in the local name, everything after the first plus 
// sign will be ignored. This allows certain emails to be filtered. Note that 
// this rule does not apply to domain names.
//
// For example, "m.y+name@email.com" will be forwarded to "my@email.com".
// It is possible to use both of these rules at the same time.
//
// Given an array of strings emails where we send one email to each emails[i], 
// return the number of different addresses that actually receive mails.
//
// Constraints:
//     1 <= emails.length <= 100
//     1 <= emails[i].length <= 100
//     emails[i] consist of lowercase English letters, '+', '.' and '@'.
//     Each emails[i] contains exactly one '@' character.
//     All local and domain names are non-empty.
//     Local names do not start with a '+' character.
//     Domain names end with the ".com" suffix.



pub fn num_unique_emails(_emails: Vec<String>) -> i32{
    0
}



fn _do_test(emails: Vec<String>, expected: i32){
    let e_mails = emails.clone();
    let result = num_unique_emails(emails);
    assert!(
        result == expected,
        "\nInput = {e_mails:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn example_1(){
    //Input: emails = ["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetcode.com","testemail+david@lee.tcode.com"]
    //Output: 2
    //Explanation: "testemail@leetcode.com" and "testemail@lee.tcode.com" actually receive mails.
    _do_test(
        vec![
            "test.email+alex@leetcode.com".to_string(), 
            "test.e.mail+bob.cathy@leetcode.com".to_string(), 
            "testemail+david@lee.tcode.com".to_string()
        ], 
        2
    );
}

#[test]
fn example_2(){
    //Input: emails = ["a@leetcode.com","b@leetcode.com","c@leetcode.com"]
    //Output: 3
    _do_test(
        vec![
            "a@leetcode.com".to_string(), 
            "b@leetcode.com".to_string(), 
            "c@leetcode.com".to_string()
        ], 
        3
    );
}