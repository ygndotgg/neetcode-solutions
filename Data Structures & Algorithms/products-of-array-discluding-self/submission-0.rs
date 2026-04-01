impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
     let mut n = nums.len();
     let mut left = vec![0;n];
     let mut right = vec![0;n];
     let mut result = vec![0;n];
     left[0] = 1;
     right[n-1] = 1;
     for i in 1..n{
        left[i] = nums[i-1] * left[i-1];
     }
     for i in (0..n-1).rev() {
        right[i] = nums[i+1] * right[i+1];
     }
     println!("Left{:?}\nRight{:?}",left,right);
     for i in 0..n {
        result[i] = left[i] * right[i];
     }
     result
    }
}
