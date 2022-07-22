// Problem

// An electronics shop sells red and blue lamps. A red lamp costs XXX rupees and a blue lamp costs YYY rupees.

// Chef is going to buy exactly NNN lamps from this shop. Find the minimum amount of money Chef needs to pay such that at least KKK of the lamps bought are red.
// Input Format

//     The first line of input will contain a single integer TTT, denoting the number of test cases.
//     Each test case consists of a single line containing four space-separated integers N,K,X,YN, K, X, YN,K,X,Y.

// Output Format

// For each test case, output on a new line the minimum amount of money Chef needs to pay in order to buy NNN lamps such that at least KKK of the lamps bought are red.

use std::io::{stdin, stdout, Write};

pub fn solution() {
    let _ = stdout().flush();
    let mut n_tests_str = String::new();
    stdin().read_line(&mut n_tests_str).unwrap();

    let n: u32 = n_tests_str.trim().parse().unwrap();

    for _ in 0..n {
        // Test cases
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).unwrap();

        let mut cost = 0;
        let nkxy = input_str.trim().split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let cheapest_cost = std::cmp::min(nkxy[2], nkxy[3]);
        cost += nkxy[1] * nkxy[2];
        cost += cheapest_cost * (nkxy[0] - nkxy[1]);
        println!("{}", cost);
    }
}