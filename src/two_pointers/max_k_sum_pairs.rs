use std::collections::HashMap;
use std::cmp::min;

fn max_k_sum_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut nums = nums.iter();
        let mut left = nums.next();
        let mut right = nums.next_back();
        let mut count = 0;
        while left.is_some() && right.is_some() {
            let l = left.unwrap();
            let r = right.unwrap();
            if l + r == k {
                count += 1;
                left = nums.next();
                right = nums.next_back();
            } else if l + r > k {
                right = nums.next_back();
            } else {
                left = nums.next();
            }
        }
        count as i32
}

fn max_k_sum_pairs_2(nums: Vec<i32>, k: i32) -> i32 {
    let mut counts = HashMap::new();
        let mut pairs  = 0;
        for &num in &nums {
            let compliment = k - num;

            if let Some(count) = counts.get_mut(&compliment) {
                if *count > 0 {
                    *count -= 1;
                    pairs += 1;
                    continue;
                }
            }
            counts.entry(num).and_modify(|c| *c += 1).or_insert(1);
        }
        pairs
}

fn max_k_sum_pairs_3(nums: Vec<i32>, k: i32) -> i32 {
    let mut tmp: Vec<i32> = nums.iter().cloned().filter(|&num| num < k).collect();

        if tmp.is_empty() {
            return 0;
        }

        let max = *tmp.iter().max().unwrap();
        tmp.retain(|&num| num >= k - max);

        if tmp.is_empty() {
            return 0;
        }

        let mut copy: HashMap<i32, i32> = HashMap::new();
        for &num in &tmp {
            *copy.entry(num).or_insert(0) += 1;
        }

        let mut pairs = 0;
        for (&key, &value) in copy.clone().iter() {
            if key < k - key {
                let pair_value = *copy.get(&(k - key)).unwrap_or(&0);
                pairs += min(value, pair_value);
            } else if key == k - key {
                pairs += value / 2;
            }
        }

        pairs
}

pub fn run() {
    let v = vec![1, 2, 3, 4, 5];
    let k = 5;
    assert_eq!(max_k_sum_pairs(v, k), 2, "max_k_sum_pairs.rs");
    let v = vec![1, 2, 3, 4, 5];
    let k = 5;
    assert_eq!(max_k_sum_pairs_2(v, k), 2, "max_k_sum_pairs.rs");
    let v = vec![1, 2, 3, 4, 5];
    let k = 5;
    assert_eq!(max_k_sum_pairs_3(v, k), 2, "max_k_sum_pairs.rs");
}
