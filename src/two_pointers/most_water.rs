use std::cmp::{min,max};

fn container_with_most_water(height: Vec<i32>) -> i32 {
  let mut max_area = 0;
        let mut l = 0 as usize;
        let mut r = height.len() - 1 as usize;
        while l < r {
            let area = (r - l) * min(height[r], height[l]) as usize;
            max_area = max(area, max_area);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
    max_area as i32
}

pub fn run() {
    let heights = vec![1,8,6,2,5,4,8,3,7];
    let area = container_with_most_water(heights);
    assert_eq!(area, 49);
    let heights = vec![1,1];
    let area = container_with_most_water(heights);
    assert_eq!(area, 1);
}
