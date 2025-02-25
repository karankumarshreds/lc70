fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut result = 0.0;
    if k > nums.len() as i32 {
        return 0.0;
    }
    for i in 0..nums.len() as i32 - k + 1 {
        let mut sum = 0;
        for j in i..i+k {
            sum += nums[j as usize];
        }
        if result < sum as f64 / k as f64 {
            result = sum as f64 / k as f64;
        }
    }
    result
}

fn find_max_average_2(nums: Vec<i32>, k: i32) -> f64 {
    let mut result: f64 = f64::MIN;
    for w in nums.windows(k as usize) {
        let sum: i32 = w.iter().sum();
        result = result.max(sum as f64/k as f64);
    }
    result
}

pub fn run() {
    assert_eq!(find_max_average(vec![1,12,-5,-6,50,3], 4), 12.75, "find_max_average(vec![1,12,-5,-6,50,3], 4)");
    assert_eq!(find_max_average_2(vec![1,12,-5,-6,50,3], 4), 12.75, "find_max_average(vec![1,12,-5,-6,50,3], 4)");
}

