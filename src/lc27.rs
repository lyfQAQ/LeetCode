struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut t = 0usize;
        for i in 0..nums.len() {
            if nums[i] == val {
                continue;
            }
            nums.swap(i, t);
            t += 1;
        }
        t as i32
    }
    pub fn remove_element_1(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x| *x != val);
        nums.len() as i32
    }
}