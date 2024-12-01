use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
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

    left_column.sort();
    right_column.sort();

    // Print the first few elements to verify
    println!("First 5 elements of left column: {:?}", &left_column[..5]);
    println!("First 5 elements of right column: {:?}", &right_column[..5]);
    
    // Print the lengths to verify all data was read
    println!("Left column length: {}", left_column.len());
    println!("Right column length: {}", right_column.len());

    let mut dists: Vec<f64> = Vec::new();

    // Day 1 part 1
    for i in 0..left_column.len() {
        let dist: f64 = (left_column[i] as f64 - right_column[i] as f64).abs();

        dists.push(dist);
        println!("{}", dist);
    }
    let mut similarity_score: u32 = 0;

    // Day 1 part 2
    for &left_num in &left_column {
        let count = right_column.iter().filter(|&&x| x == left_num).count() as u32;
        similarity_score += left_num * count;
    }

    println!("Total similarity score: {}", similarity_score);

    println!("Total dist: {}", dists.iter().sum::<f64>());
    Ok(())
}