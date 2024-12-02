use std::{fs};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() {
    let (mut nums1, nums2) = read_input();
    println!("Absolute difference: {}", calc_abs_dist(&nums1, &nums2));
    println!("Similarity score: {}", calc_similarity_score(&mut nums1, &nums2));
}

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let file = fs::File::open("input").expect("Could not read input");

    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();

    for line in BufReader::new(&file).lines() {
        let Ok(line) = line else {
            continue
        };

        line.split("   ").into_iter().enumerate().for_each(|(i, n)| {
            let n = n.trim().parse::<i32>().unwrap();
            if i % 2 == 0 {
                nums1.push(n);
            } else {
                nums2.push(n);
            }
        });
    }

    nums1.sort();
    nums2.sort();
    
    (nums1, nums2)
}

fn calc_abs_dist(nums1: &[i32], nums2: &[i32]) -> i32 {
    nums1.iter().enumerate().map(|(index, n)| {
        n.abs_diff(nums2[index]) as i32
    }).sum()
}

fn calc_similarity_score(nums1: &mut [i32], nums2: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    nums2.iter().for_each(|n| {
        map.entry(*n).and_modify(|e| *e += 1).or_insert(1);
    });

    nums1.iter_mut().for_each(|n| {
        *n *= map.get(n).unwrap_or(&0);
    });

    nums1.iter().sum()
}
