pub fn kth_smallest(nums: &mut [i32], k: usize) -> Option<i32> {
    nums.sort();
    nums.get(k - 1).cloned()
}
