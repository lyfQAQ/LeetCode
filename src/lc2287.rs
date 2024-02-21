struct Solution;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let count = |s: String| {
            s.bytes().fold([0; 26], |mut acc, b| {
                acc[(b - b'a') as usize] += 1;
                acc
            })
        };

        let cnt_s = count(s);
        let cnt_t = count(target);
        cnt_t
            .iter()
            .zip(cnt_s.iter())
            .filter_map(|(&tc, &sc)| if tc > 0 { Some(sc / tc) } else { None })
            .min()
            .unwrap_or(0)
    }
}
