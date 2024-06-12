pub fn find_median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (nums[mid - 1] + nums[mid]) as f64 / 2.0
    } else {
        nums[len / 2] as f64
    }
}
