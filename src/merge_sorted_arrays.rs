pub fn merge_sorted_arrays(nums1: &mut Vec<i32>, nums2: &mut Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);
    
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] <= nums2[j] {
            result.push(nums1[i]);
            i += 1;
        } else {
            result.push(nums2[j]);
            j += 1;
        }
    }
    
    result.extend_from_slice(&nums1[i..]);
    result.extend_from_slice(&nums2[j..]);
    
    result
}
