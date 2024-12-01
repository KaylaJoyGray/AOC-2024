use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(file_name) = args.get(1) else {
        println!("Expected file name");
        return;
    };

    let Ok(file) = fs::read_to_string(file_name) else {
        println!("Invalid file name: {}", file_name);
        return;
    };

    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();

    file.split([' ', '\n']).filter_map(|s| { s.trim().parse::<i32>().ok() }).enumerate().for_each(|(index, n)| {
       if index % 2 == 0 {
            nums1.push(n);
       } else {
           nums2.push(n);
       }
    });

    nums1.sort();
    nums2.sort();
    
    assert_eq!(nums1.len(), nums2.len());

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
       match map.get(n) {
           None => { map.insert(*n, 1); }
           Some(v) => { map.insert(*n, v + 1);}
       }
    });

    nums1.iter_mut().for_each(|n| {
        *n *= map.get(n).unwrap_or(&0);
    });

    let similarity_score: i32 = nums1.iter().sum();

    println!("Similarity Score: {}", similarity_score);
}
