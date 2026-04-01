use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
      let mut hs = HashMap::new();
      for word in strs {
        let mut keys = vec![0;26];
        for w in word.as_bytes() {
            keys[(w - b'a') as usize] +=1;
        }
        hs.entry(keys).or_insert(Vec::new()).push(word);
      }
      hs.into_values().collect()
    }
}
