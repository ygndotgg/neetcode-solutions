use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
      let  (mut a,mut b) = (HashMap::new(),HashMap::new());
      for i in s.chars(){
        *a.entry(i).or_insert(0) +=1;
      }
      for j in t.chars(){
        *b.entry(j).or_insert(0) +=1;
      }
      a == b
    }
}
