use strings::valid_anagram::is_anagram;

mod strings;

fn main() {
    is_anagram("racecar".to_string(), "raccare".to_string());
}
