use code_fuzzy_match::fuzzy_match;
use rand::{random, thread_rng, Rng};

fn random_string(min_length: usize, max_length: usize) -> String {
    let length = thread_rng().gen_range(min_length..=max_length);
    let mut chars = Vec::new();
    if thread_rng().gen_range(0..10) == 0 {
        for _ in 0..length {
            chars.push(random::<char>());
        }
    } else if thread_rng().gen_range(0..10) < 8 {
        for _ in 0..length {
            chars.push(thread_rng().gen_range(' '..='~'));
        }
    } else {
        for _ in 0..length {
            chars.push(thread_rng().gen_range('0'..='z'));
        }
    }
    chars.into_iter().collect()
}

fn main() {
    loop {
        let query = random_string(1, 10);

        for _ in 0..10 {
            let target = random_string(1, 30);
            let _ = fuzzy_match(&target, &query);
        }
    }
}
