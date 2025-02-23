pub fn move_zeros(nums: &mut Vec<i32>) {
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

pub fn run() {
    let mut v = vec![0, 1, 0, 3, 12];
    move_zeros(&mut v);
    assert_eq!(v, vec![1, 3, 12, 0, 0], "move_zeros.rs");
    let mut v = vec![2, 0, 1];
    move_zeros(&mut v);
    assert_eq!(v, vec![2, 1, 0], "move_zeros.rs");
    let mut v = vec![2, 1];
    move_zeros(&mut v);
    assert_eq!(v, vec![2, 1], "move_zeros.rs");
    let mut v = vec![1, 2, 3, 1];
    move_zeros(&mut v);
    assert_eq!(v, vec![1, 2, 3, 1], "move_zeros.rs");
}
