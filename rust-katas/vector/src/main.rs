/*
Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

fn median(v: &Vec<i32>) -> i32 {
    let mut v = v.clone();
    v.sort();
    
    return match v.get(v.len() / 2) {
        Some(median) => *median,
        None => 0
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut current_mode: (i32, i32) = (0, 0); // (mode, mode_count)

    for i in v.iter() {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;
        if *count > current_mode.1 {
            current_mode = (*i, *count);
        }
    }
    
    current_mode.0
}

fn main() {
    let v = vec![2, 1, 3, 4, 3, 5, 7, 9];
    
    let median = median(&v);
    let mode = mode(&v);

    println!("Median: {}", median);
    println!("Mode:   {}", mode);
}
