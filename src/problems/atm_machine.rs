// Problem
// Read problems statements Bengali , Mandarin chinese , Russian and Vietnamese as well.

// There is an ATM machine. Initially, it contains a total of KKK units of money. NNN people (numbered 111 through NNN) want to withdraw money; for each valid iii, the iii-th person wants to withdraw AiA_iAi​ units of money.

// The people come in and try to withdraw money one by one, in the increasing order of their indices. Whenever someone tries to withdraw money, if the machine has at least the required amount of money, it will give out the required amount. Otherwise, it will throw an error and not give out anything; in that case, this person will return home directly without trying to do anything else.

// For each person, determine whether they will get the required amount of money or not.
// Input

//     The first line of the input contains a single integer TTT denoting the number of test cases. The description of TTT test cases follows.
//     The first line of each test case contains two space-separated integers NNN and KKK.
//     The second line contains NNN space-separated integers A1,A2,…,ANA_1, A_2, \dots, A_NA1​,A2​,…,AN​.

// Output

// For each test case, print a single line containing a string with length NNN. For each valid iii, the iii-th character of this string should be '1' if the iii-th person will successfully withdraw their money or '0' otherwise.

use std::io::{stdin, stdout, Write};

pub fn solution() {
    let mut test_str = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut test_str).unwrap();
    let n: u32 = test_str.trim().parse().unwrap();
    for _ in 0..n {
        // First input is N K
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).unwrap();
        let n_k = input_str.trim().split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // Second input is A1...AN
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).unwrap();
        let a_n = input_str.trim().split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut out_str = String::new();
        let mut k = n_k[1];
        for a in a_n {
            if k - a >= 0 {
                k -= a;
                out_str.push('1');
            } else {
                out_str.push('0');
            }
        }
        println!("{}", out_str);
    }
}