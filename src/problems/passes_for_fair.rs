// Problem

// There is a fair going on in Chefland. Chef wants to visit the fair along with his NNN friends. Chef manages to collect KKK passes for the fair. Will Chef be able to enter the fair with all his NNN friends?

// A person can enter the fair using one pass, and each pass can be used by only one person.
// Input Format

//     The first line of input will contain a single integer TTT, denoting the number of test cases.
//     Each test case consists of a single line containing two space-separated integers N,KN, KN,K.

// Output Format

// For each test case, print on a new line YES if Chef will be able to enter the fair with all his NNN friends and NO otherwise.

// You may print each character of the string in either uppercase or lowercase (for example, the strings yEs, yes, Yes, and YES will all be treated as identical).

use std::io::{stdin,stdout,Write};

pub fn solution() {
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line...");
    let n = input.trim().parse::<i32>().unwrap();
    for _ in 0..n {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line...");
        let user_in = input.split(" ").map(|i| i.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if user_in[0] < user_in[1] {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
