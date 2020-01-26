use hangul::HangulExt;
use rand::seq::SliceRandom;
use strsim::normalized_levenshtein;

pub fn manufacture_file(content: &String) -> Vec<String> {
    let content: Vec<&str> = content.split("\n").collect();

    let mut result: Vec<String> = Vec::new();

    for i in content {
        &mut result.push(String::from(i));
    }

    result.insert(0, "".into());
    result.pop();

    result.shuffle(&mut rand::thread_rng());

    result
}

pub fn get_accuracy(text: &String, input: &String) -> usize {
    (normalized_levenshtein(text, input) * 100_f64) as usize
}

pub fn get_typing_speed(value: &String, elapsed_time: f64) -> usize {
    let mut typed = 0;
    for i in value.chars() {
        match i.is_syllable() {
            true => match i.is_open().unwrap() {
                true => typed += 2,
                false => typed += 3,
            },
            false => typed += 1,
        }
    }

    (typed as f64 / elapsed_time * 60_f64).round() as usize
}
