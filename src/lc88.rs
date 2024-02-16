struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut one = m - 1;
        let mut two = n - 1;
        let mut s = (m + n) as usize;
        let mut t = 0i32;
        for i in (0..s).rev() {
            if one < 0 || two < 0 {
                t = i as i32;
                break;
            }
            if nums1[one as usize] >= nums2[two as usize] {
                nums1[i] = nums1[one as usize];
                one -= 1;
            } else {
                nums1[i] = nums2[two as usize];
                two -= 1;
            }
        }
        if one < 0 {
            while t >= 0 {
                nums1[t as usize] = nums2[two as usize];
                two -= 1;
                t -= 1;
            }
        }
        if two < 0 {
            while t >= 0 {
                nums1[t as usize] = nums1[one as usize];
                one -= 1;
                t -= 1;
            }
        }
    }
}

#[test]
fn test_solve() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    println!("{:?}", Solution::merge(&mut nums1, 3, &mut nums2, 3))
}
