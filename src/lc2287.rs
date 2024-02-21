use std::io::Read;

struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut weight = [0; 26];
        // 做一个映射  "hlabcdefgijkmnopqrstuvwxyz"
        // h->a  l->b
        for (i, ch) in order.bytes().enumerate() {
            weight[(ch - b'a') as usize] = i + b'a' as usize;
        }
        let words = words
            .into_iter()
            .map(|s| {
                s.bytes()
                    .map(|c| weight[(c - b'a') as usize])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut sorted = words.clone();
        sorted.sort();
        sorted == words
    }
}
