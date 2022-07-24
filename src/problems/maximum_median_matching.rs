// Problem

// NNN boys and NNN girls attend a dance class, where NNN is odd. For today's practice session, they need to form NNN boy-girl pairs.

// The iii-th boy has a happiness value of AiA_iAi​ and the iii-th girl has a happiness value of BiB_iBi​. A pair consisting of the iii-th boy and the jjj-th girl has a happiness value of Ai+BjA_i + B_jAi​+Bj​.

// Let the happiness values of the NNN pairs be C1,C2,…,CNC_1, C_2, \ldots, C_NC1​,C2​,…,CN​. The dance instructor would like it if many of the pairs have a high happiness value, and passes the task to you — find the maximum possible value of the median of CCC, if the boy-girl pairs are chosen optimally.

// Note: The median of a odd-sized list of integers is the middle number when they are sorted. For example, the median of [1][1][1] is 111, the median of [1,5,2][1, 5, 2][1,5,2] is 222, and the median of [30,5,5,56,3][30, 5, 5, 56, 3][30,5,5,56,3] is 555.
// Input Format

//     The first line of input will contain a single integer TTT, denoting the number of test cases.
//     Each test case consists of three lines of input.
//         The first line of each test case contains a single integer NNN.
//         The second line of each test case contains NNN space-separated integers A1,A2,…,ANA_1, A_2, \ldots, A_NA1​,A2​,…,AN​ — the happiness values of the boys.
//         The third line of each test case contains NNN space-separated integers B1,B2,…,BNB_1, B_2, \ldots, B_NB1​,B2​,…,BN​ — the happiness values of the girls.

// Output Format

// For each test case, output on a new line the maximum possible median happiness of the NNN pairs.

use std::io::{stdin, stdout, Write};
use std::convert::TryInto;

pub fn solution() {
    let _ = stdout().flush();
    let mut n_test_str = String::new();
    stdin().read_line(&mut n_test_str).unwrap();
    let n = n_test_str.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        let mut sum_vec: Vec<i32> = Vec::new();

        let mut num = String::new();
        stdin().read_line(&mut num).unwrap();
        let num = num.trim().parse::<usize>().unwrap();

        let mut boys = String::new();
        stdin().read_line(&mut boys).unwrap();
        let mut boys_vec = boys.trim().split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        boys_vec.sort();

        let mut girls = String::new();
        stdin().read_line(&mut girls).unwrap();
        let mut girls_vec = girls.trim().split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        girls_vec.sort();

        let median_index: usize = num / 2 + 1;
        let mut first_half_boys = boys_vec.iter().take(median_index).map(|i| i.to_owned()).collect::<Vec<i32>>();
        let mut last_half_boys = boys_vec.iter().skip(median_index).take(usize::max_value()).map(|i| i.to_owned()).collect::<Vec<i32>>();
        last_half_boys.reverse();
        

        // let mut first_half_girls = girls_vec.iter().take(median_index).map(|i| i.to_owned()).collect::<Vec<i32>>();
        // let mut last_half_girls = girls_vec.iter().skip(median_index).take(usize::max_value()).map(|i| i.to_owned()).collect::<Vec<i32>>();

        first_half_boys.append(&mut last_half_boys);

        for i in 0..num {
            sum_vec.push(girls_vec[i] + first_half_boys[i]);
        }

        // Output median
        sum_vec.sort();
        println!("{}", sum_vec[median_index]);
    }
}