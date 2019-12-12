use strsim::normalized_levenshtein;

pub fn manufacture_file(content: &String) -> Vec<String> {
    let content: Vec<&str> = content.split("\n").collect();

    let mut result: Vec<String> = Vec::new();

    for i in content {
        &mut result.push(String::from(i));
    }

    result.insert(0, "".into());
    result.pop();

    result
}

pub fn check(text: &String, input: &String) -> String {
    format!("{}%", get_accuracy(&text, &input))
}

pub fn get_accuracy(text: &String, input: &String) -> usize {
    (normalized_levenshtein(text, input) * 100_f64) as usize
}
