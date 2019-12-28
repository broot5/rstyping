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

pub fn get_typing_speed(typed: usize, elapsed_time: f64) -> usize {
    (typed as f64 / elapsed_time * 60_f64).round() as usize
}
