use array::container_with_most_water::max_area;
use helpers::select_random::select_random_problem;

mod strings;
mod heap;
mod array;
mod helpers;

fn main() {
    select_random_problem();
    let heights: Vec<i32> = vec![2,3,5,6,2,3];
        max_area(heights);
}

