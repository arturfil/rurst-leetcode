use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let arr_len = height.len(); 
    let (mut left, mut right) = (0, arr_len - 1);
    let mut max_area = 0;    

    while left < right {
        let base: usize = right - left;
        let min_height: i32 = cmp::min(height[left], height[right]) as i32;
        let area = (base * min_height as usize) as i32;
        max_area = cmp::max(area, max_area);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area as i32
}

