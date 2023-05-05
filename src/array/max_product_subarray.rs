use std::cmp;
// ex1: nums = [2,3,-2,4] -> would be 2 * 3 = 6
// ex2: 
//
pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut tot = nums[0];
    let mut max= nums[1];
    let mut min = nums[1];
    
    for num in nums {
        let prev_max = max;
        max = *[num, num * prev_max, num * min].iter().max().unwrap();
        min = *[num, num * prev_max, num * min].iter().min().unwrap();
        tot = cmp::max(tot, max);   
    }
    
    return tot;
}
