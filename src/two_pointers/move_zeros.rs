pub fn run(nums: &mut Vec<i32>) {
    let mut curr = 0;
    let mut next = 0;
    while next < nums.len() {
        if nums[curr] != 0 {
            curr += 1;
        } else if nums[next] != 0 {
            let temp = nums[curr];
            nums[curr] = nums[next];
            nums[next] = temp;
            curr += 1;
        }
        next += 1;
    }
}
