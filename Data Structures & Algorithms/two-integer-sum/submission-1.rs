impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ne:Vec<(i32,usize)> = nums.into_iter().enumerate().map(|(a,b)|{
        (b,a)
    }).collect();
    ne.sort();
    let mut i = 0;
    let mut j = ne.len() -1;
    while i < j {
        let cur = ne[i].0 + ne[j].0;
        if cur == target {
            return vec![
                ne[i].1.min(ne[j].1) as i32,
                ne[i].1.max(ne[j].1) as i32
            ]
        }
        if cur < target {
            i+=1;
        }
        else {
            j-=1;
        }
    }
    vec![]
    }
}
