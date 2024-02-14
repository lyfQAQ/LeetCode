struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum_l = vec![0; n + 2];
        let mut sum_r = sum_l.clone();
        let mut i = 1;
        while i <= n {
            sum_l[i] = nums[i - 1] + sum_l[i - 1];
            i += 1;
        }
        i = n;
        while i > 0 {
            sum_r[i] = nums[i - 1] + sum_r[i + 1];
            i -= 1;
        }
        i = 1;
        let mut pos = -1;
        while i <= n {
            if sum_l[i] == sum_r[i] {
                pos = (i - 1) as i32;
                break;
            }
            i += 1;
        }
        pos
    }
    pub fn pivot_index_plus(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let n = nums.len();
        let mut partial = 0;
        let mut i = 0;
        let mut pos = -1;
        while i < n {
            if partial == sum - partial - nums[i] {
                pos = i as i32;
                break;
            }
            partial += nums[i];
            i += 1;
        }
        pos
    }
}

#[test]
fn test_fn() {
    let nums = vec![1,7,3,6,5,6];
    println!("{}", Solution::pivot_index_plus(nums));
}