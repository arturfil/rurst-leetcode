use array::{max_product_subarray::max_product, max_subarray::max_sub_array};


mod strings;
mod heap;
mod array;
mod helpers;

fn main() {
    // select_random_problem();
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        max_sub_array(nums);
}

