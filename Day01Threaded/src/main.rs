use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use rayon::prelude::*;
use rayon::iter::IntoParallelRefIterator;

fn main() -> io::Result<()> {
    let start = Instant::now();

    // Create vectors to store the numbers
    let mut left_column: Vec<u32> = Vec::new();
    let mut right_column: Vec<u32> = Vec::new();

    // Open and read the file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Process each line
    for line in reader.lines() {
        let line = line?;
        // Split the line by whitespace and collect numbers
        let numbers: Vec<&str> = line.split_whitespace().collect();
        
        if numbers.len() == 2 {
            // Parse the numbers and add them to respective vectors
            if let Ok(left) = numbers[0].parse::<u32>() {
                left_column.push(left);
            }
            if let Ok(right) = numbers[1].parse::<u32>() {
                right_column.push(right);
            }
        }
    }

    left_column.par_sort_unstable();
    right_column.par_sort_unstable();

    // Day 1 part 1
    let dists: Vec<f64> = (0..left_column.len())
        .into_par_iter()
        .map(|i| (left_column[i] as f64 - right_column[i] as f64).abs())
        .collect();

    // Day 1 part 2
    let similarity_score: u32 = left_column
        .par_iter()
        .map(|&left_num| {
            let count = right_column
                .par_iter()
                .filter(|&&x| x == left_num)
                .count() as u32;
            left_num * count
        })
        .sum();

    println!("Total similarity score: {}", similarity_score);
    println!("Total dist: {}", dists.par_iter().sum::<f64>());

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}