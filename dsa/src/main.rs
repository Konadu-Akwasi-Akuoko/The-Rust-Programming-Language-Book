mod sliding_window_1;

use sliding_window_1::Solution;

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let k: i32 = 3;

    let result = Solution::maximum_subarray_sum(nums, k);
}
