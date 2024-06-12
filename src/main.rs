mod palindrome;
mod first_occurrence;
mod shortest_word;
mod is_prime;
mod find_median;
mod longest_common_prefix;
mod kth_smallest;
mod max_depth;
mod reverse_string;
mod merge_sorted_arrays;
mod max_subarray_sum;

fn main() {
    let string1 = "radar";
    let string2 = "hello";
    println!("Is '{}' a palindrome? {}", string1, palindrome::is_palindrome(string1));
    println!("Is '{}' a palindrome? {}", string2, palindrome::is_palindrome(string2));

    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    let target = 4;
    println!("First occurrence of {}: {:?}", target, first_occurrence::first_occurrence(&nums, target));

    let words = "The quick brown fox jumps over the lazy dog";
    println!("Shortest word: {:?}", shortest_word::shortest_word(words));

    let num1 = 17;
    let num2 = 15;
    println!("Is {} prime? {}", num1, is_prime::is_prime(num1));
    println!("Is {} prime? {}", num2, is_prime::is_prime(num2));

    let nums_sorted = vec![1, 2, 3, 4, 5];
    println!("Median of {:?}: {}", nums_sorted, find_median::find_median(&nums_sorted));

    let strings = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("Longest common prefix: {:?}", longest_common_prefix::longest_common_prefix(&strings));

    let mut nums_kth = vec![4, 1, 3, 2, 5];
    let k = 3;
    println!("{}th smallest element in {:?}: {:?}", k, nums_kth, kth_smallest::kth_smallest(&mut nums_kth.clone(), k));

    // Create your binary tree here if needed

    let original_str = "Hello, World!";
    let reversed_str = reverse_string::reverse_string(original_str);
    println!("Original: {}", original_str);
    println!("Reversed: {}", reversed_str);

    let mut nums1 = vec![1, 3, 5, 7, 9];
    let mut nums2 = vec![2, 4, 6, 8, 10];
    let merged_nums = merge_sorted_arrays::merge_sorted_arrays(&mut nums1, &mut nums2);
    println!("Merged sorted arrays: {:?}", merged_nums);

    let nums_subarray = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum: {}", max_subarray_sum::max_subarray_sum(&nums_subarray));
}
