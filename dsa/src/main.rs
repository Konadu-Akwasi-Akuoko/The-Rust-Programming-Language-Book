mod sliding_window_1;

use sliding_window_1::Solution;

fn main() {
    let nums: Vec<i32> = vec![1, 5, 4, 2, 9, 9, 9];
    let k: i32 = 3;

    let result = Solution::maximum_subarray_sum(nums, k);
    println!("{:#?}", result)
}
