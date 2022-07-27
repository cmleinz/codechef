// Problem

// You have a grid with NNN rows and MMM columns. You have two types of tiles — one of dimensions 2×22 \times 22×2 and the other of dimensions 1×11 \times 11×1. You want to cover the grid using these two types of tiles in such a way that:

//     Each cell of the grid is covered by exactly one tile; and
//     The number of 1×11 \times 11×1 tiles used is minimized.

// Find the minimum number of 1×11 \times 11×1 tiles you have to use to fill the grid.
// Input Format

//     The first line of input will contain a single integer TTT, denoting the number of test cases.
//     Each test case consists of a single line containing two space-separated integers N,MN, MN,M.

// Output Format

// For each test case, print on a new line the minimum number of 1×11 \times 11×1 tiles needed to fill the grid.

use std::io::{stdin, stdout, Write};


pub fn solution() {
    let _ = stdout().flush();
    let mut test_str = String::new();
    let _ = stdin().read_line(&mut test_str).unwrap();
    let t = test_str.trim().parse::<usize>().unwrap();
    
    for _ in 0..t {
        let mut input_str = String::new();
        let _ = stdin().read_line(&mut input_str).unwrap();
        let nm = input_str.trim().split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let rows = nm[0];
        let cols = nm[1];
        match (rows % 2, cols % 2) {
            (0, 0) => println!("{}", 0),
            (1, 0) => println!("{}", cols),
            (0, 1) => println!("{}", rows),
            (1, 1) => println!("{}", rows + cols - 1),
            _ => println!("Failed"),
        }
    }    
}