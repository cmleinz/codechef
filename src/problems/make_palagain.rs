// Problem

// Chef has a string AAA (containing lowercase Latin letters only) of length NNN where NNN is even. He can perform the following operation any number of times:

//     Swap AiA_iAi​ and Ai+2A_{i + 2}Ai+2​ for any 1≤i≤(N−2)1 \le i \le (N - 2)1≤i≤(N−2)

// Determine if Chef can convert string AAA to a palindromic string.

// Note: A string is called a palindrome if it reads the same backwards and forwards. For example, noon\texttt{noon}noon and level\texttt{level}level are palindromic strings but ebb\texttt{ebb}ebb is not.
// Input Format

//     The first line contains a single integer TTT — the number of test cases. Then the test cases follow.
//     The first line of each test case contains an integer NNN — the length of the string AAA.
//     The second line of each test case contains a string AAA of length NNN containing lowercase Latin letters only.

// Output Format

// For each test case, output YES if Chef can convert the string AAA to a palindromic string. Otherwise, output NO.

// You may print each character of YES and NO in either uppercase or lowercase (for example, yes, yEs, Yes will be considered identical).

// Thoughts
// Clearly the count for all characters must be even. 
// Futhermore we can think of the characters as being on two different "tracks" i.e even indexes can never be swapped with odd indexes.
// Need to create a hashmap for characters as follows:
// 'a': {
//     count_of_odd_idexes: 2,
//     count_of_even_idexes: 3,
// }
// For all characters both the count of odd indexes and the count of even indexes must be even.

use std::io::{stdin,stdout,Write};
use std::collections::HashMap;

struct IndexMap {
    even_index_count: i32,
    odd_index_count: i32,
}

pub fn solution() {
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line...");
    let n = input.trim().parse::<i32>().unwrap();
    for _ in 0..n {
        let mut len = String::new();
        stdin().read_line(&mut len).expect("Failed to read line...");
        let len = len.trim().parse::<i32>().unwrap(); 

        let mut palindrome = String::new();
        stdin().read_line(&mut palindrome).expect("Failed to read line...");
        let map = make_map(palindrome.trim());
        let mut passing = true;
        for val in map.values() {
            if val.even_index_count != val.odd_index_count {
                passing = false;
            }
        }
        match passing {
            true => println!("YES"),
            false => println!("NO"),
        }
    }
}

fn make_map(trimmed_str: &str) -> HashMap<char, IndexMap> {
    let mut mapping: HashMap<char, IndexMap> = HashMap::new();
    trimmed_str.char_indices().for_each(|(n, c)| {
        if n % 2 == 0 {
            // even branch
            let update = match mapping.get(&c) {
                Some(im) => {
                    IndexMap { even_index_count: im.even_index_count + 1, odd_index_count: im.odd_index_count }
                },
                None => {
                    IndexMap { even_index_count: 1, odd_index_count: 0 }
                }
            };
            mapping.insert(c, update);
        } else {
            // odd branch
            let update = match mapping.get(&c) {
                Some(im) => {
                    IndexMap { even_index_count: im.even_index_count, odd_index_count: im.odd_index_count + 1 }
                },
                None => {
                    IndexMap { even_index_count: 0, odd_index_count: 1 }
                }
            };
            mapping.insert(c, update);
        }
    });
    mapping
}