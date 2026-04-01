impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
     if nums.is_empty() {
        return 0;
     }
     nums.sort();
     let mut count =1;
     let mut max_count =1;

     for i in 1..nums.len(){
        if nums[i] == nums[i-1] {
            continue;
        }
        if nums[i] == nums[i-1]+1{
            count +=1;
            
           
        }
        else {
count =1;
        }
        max_count = max_count.max(count);
        
     }
     max_count
    }
}
