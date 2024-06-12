pub fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_ending_here = nums[0];
    let mut max_so_far = nums[0];

    for &num in nums.iter().skip(1) {
        max_ending_here = num.max(max_ending_here + num);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}
