impl Solution {
    pub fn has_duplicate(mut nums: Vec<i32>) -> bool {
     if nums.is_empty(){
        return false;
     }
     nums.sort();
     let mut i = 0;
     while i < nums.len() -1 {
        if nums[i] == nums[i+1] {
            return true;
        }
        i+=1; 
     }
     return false;
    }
}
