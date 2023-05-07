use array::{max_product_subarray::max_product, max_subarray::max_sub_array};
use helpers::select_random::select_random_problem;
use strings::first_unique_char::first_uniq_char;


mod strings;
mod heap;
mod array;
mod helpers;

fn main() {
    select_random_problem();
    let test_str = String::from("loveleetcode");
    first_uniq_char(test_str);

}


