use helpers::select_random::select_random_problem;
use strings::valid_anagram::is_anagram;

mod strings;
mod helpers;

fn main() {
    is_anagram("racecar".to_string(), "raccare".to_string());
    select_random_problem();
}
