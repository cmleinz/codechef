// Problem

// Chef is playing a videogame, and is getting close to the end. He decides to finish the rest of the game in a single session.

// There are XXX levels remaining in the game, and each level takes Chef YYY minutes to complete. To protect against eye strain, Chef also decides that every time he completes 333 levels, he will take a ZZZ minute break from playing. Note that there is no need to take this break if the game has been completed.

// How much time (in minutes) will it take Chef to complete the game?
// Input Format

//     The first line of input will contain a single integer TTT, denoting the number of test cases.
//     The first and only line of input will contain three space-separated integers XXX, YYY, and ZZZ.

// Output Format

// For each test case, output on a new line the answer — the length of Chef's gaming session.

use std::io::{stdin, stdout, Write};

pub fn solution() {
    let _ = stdout().flush();
    let mut test_str = String::new();

    stdin().read_line(&mut test_str).unwrap();

    let n = test_str.trim().parse::<usize>().unwrap();
    for _ in 0..n {
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).unwrap(); 
        let xyz = input_str.trim().split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let eye_time = if xyz[0] % 3 == 0 {
            ((xyz[0] / 3) - 1) * xyz[2]
        } else {
            (xyz[0] / 3) * xyz[2]
        };
        let total_time = xyz[0] * xyz[1] + eye_time;
        println!("{}", total_time);
    }
}