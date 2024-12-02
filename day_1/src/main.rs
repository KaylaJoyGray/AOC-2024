use std::{fs};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() {
    let Ok(file) = fs::File::open("input") else {
        println!("Could not read input");
        return;
    };

    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();
    
    for line in BufReader::new(&file).lines() {
        let Ok(line) = line else {
            continue
        };

        line.split("   ").into_iter().enumerate().for_each(|(i, n)| {
            if i % 2 == 0 {
                nums1.push(n.trim().parse::<i32>().unwrap());
            } else {
                nums2.push(n.trim().parse::<i32>().unwrap());
            }
        });
    }

    nums1.sort();
    nums2.sort();

    let sum: i32 = nums1.iter().enumerate().map(|(index, n)| {
        n.abs_diff(nums2[index]) as i32
    }).sum();

    println!("Absolute difference: {}", sum);

    /*
     This time, you'll need to figure out exactly how often each number from the left list appears
     in the right list. Calculate a total similarity score by adding up each number in the left list
     after multiplying it by the number of times that number appears in the right list.
    */

    let mut map: HashMap<i32, i32> = HashMap::new();
    nums2.iter().for_each(|n| {
        map.entry(*n).and_modify(|e| *e += 1).or_insert(1);
    });

    nums1.iter_mut().for_each(|n| {
        *n *= map.get(n).unwrap_or(&0);
    });

    let similarity_score: i32 = nums1.iter().sum();

    println!("Similarity Score: {}", similarity_score);
}
