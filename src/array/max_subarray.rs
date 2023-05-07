use std::cmp;

// ex1: [-2,1,-3,4,-1,2,1,-5,4]; res = 6
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let (mut tot ,mut current) = (nums[0], 0);
    for i in nums {
        current = cmp::max(i, current + i);
        tot = cmp::max(tot, current);
    }
    return tot;
}
