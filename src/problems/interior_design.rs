// Problem

// Chef decided to redecorate his house, and now needs to decide between two different styles of interior design.

// For the first style, tiling the floor will cost X1X_1X1​ rupees and painting the walls will cost Y1Y_1Y1​ rupees.

// For the second style, tiling the floor will cost X2X_2X2​ rupees and painting the walls will cost Y2Y_2Y2​ rupees.

// Chef will choose whichever style has the lower total cost. How much will Chef pay for his interior design?
// Input Format

//     The first line of input will contain a single integer TTT, denoting the number of test cases.
//     Each test case consists of a single line of input, containing 444 space-separated integers X1,Y1,X2,Y2X_1, Y_1, X_2, Y_2X1​,Y1​,X2​,Y2​ as described in the statement.

// Output Format

// For each test case, output on a new line the amount Chef will pay for interior design.

use std::io::{stdin, stdout, Write};
use std::cmp;

pub fn solution() {
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line...");
    let n = input.trim().parse::<i32>().unwrap();
    for _ in 0..n {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        let i = user_input.split(" ").map(|s| s.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        println!("{}", cmp::min(i[0] + i[1], i[2] + i[3]));
    }
}