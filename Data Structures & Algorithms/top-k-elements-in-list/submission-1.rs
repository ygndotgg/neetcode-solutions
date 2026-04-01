use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
     let mut hs = HashMap::new();
     for i in nums {
        *hs.entry(i).or_insert(0) +=1;
     }
     let mut ke:Vec<(i32,i32)> = hs.into_iter().collect();
     ke.sort_by(|(_,b1),(_,b2)|b2.cmp(b1));
     ke.iter().take(k as usize).map(|x|x.0).collect()
    }
}
