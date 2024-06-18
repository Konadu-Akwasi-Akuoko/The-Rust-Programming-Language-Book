/**
 * You are given an integer array nums and an integer k. Find the maximum subarray sum of all the subarrays of nums that meet the following conditions:

The length of the subarray is k, and
All the elements of the subarray are distinct.
Return the maximum subarray sum of all the subarrays that meet the conditions. If no subarray meets the conditions, return 0.

A subarray is a contiguous non-empty sequence of elements within an array.



Example 1:

Input: nums = [1,5,4,2,9,9,9], k = 3
Output: 15
Explanation: The subarrays of nums with length 3 are:
- [1,5,4] which meets the requirements and has a sum of 10.
- [5,4,2] which meets the requirements and has a sum of 11.
- [4,2,9] which meets the requirements and has a sum of 15.
- [2,9,9] which does not meet the requirements because the element 9 is repeated.
- [9,9,9] which does not meet the requirements because the element 9 is repeated.
We return 15 because it is the maximum subarray sum of all the subarrays that meet the conditions
Example 2:

Input: nums = [4,4,4], k = 3
Output: 0
Explanation: The subarrays of nums with length 3 are:
- [4,4,4] which does not meet the requirements because the element 4 is repeated.
We return 0 because no subarrays meet the conditions.

 */
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut sum_of_current_sub_array: i32 = 0;
        let mut sum_of_arrays: Vec<i32> = Vec::new();
        let mut sub_array: Vec<i32> = Vec::new();
        let new_k: usize = k as usize;

        for i in 0..nums.len() {
            sum_of_current_sub_array += nums[i];
            sub_array.push(nums[i]);

            if i == new_k - 1 {
                let hash_set: HashSet<_> = sub_array.clone().into_iter().collect();
                if hash_set.len() == new_k {
                    sum_of_arrays.push(sum_of_current_sub_array)
                } else {
                    sum_of_arrays.push(0)
                }
            } else if i >= new_k {
                sum_of_current_sub_array -= nums[i - new_k];
                sub_array.remove(0);

                let hash_set: HashSet<_> = sub_array.clone().into_iter().collect();
                if hash_set.len() == new_k {
                    sum_of_arrays.push(sum_of_current_sub_array)
                } else {
                    sum_of_arrays.push(0)
                }
            }
        }

        let max_sum: i64 = sum_of_arrays
            .iter()
            .map(|&num| num as i64)
            .max()
            .unwrap_or(0);
        return max_sum;
    }
}
