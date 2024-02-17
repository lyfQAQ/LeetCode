struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
        // let mut slow = 0;
        // for fast in 0..nums.len() {
        //     if nums[fast] != nums[slow] {
        //         slow += 1;
        //         nums[slow] = nums[fast];
        //     }
        // }
        // (slow + 1) as i32
    }
}