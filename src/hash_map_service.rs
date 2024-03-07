use std::collections::HashMap;

pub fn get_value(key: &str, scores: &HashMap<String, i32>) -> i32 {
    let value = scores.get(key).copied().unwrap_or(0);
    value
}

pub fn insert_value(key: &str, value: i32, scores: &mut HashMap<String, i32>) {
    scores.insert(String::from(key), value);
}

pub fn remove_value(key: &str, scores: &mut HashMap<String, i32>) {
    scores.remove(key);
}

pub fn count_words_in_text(text: &String) -> HashMap<String, i32> {
    let mut words_map: HashMap<String, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = words_map.entry(String::from(word)).or_insert(0);
        *count += 1;
    }

    words_map
}

pub fn min_max_scores(scores: &HashMap<String, i32>) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for (_, value) in scores {
        if *value < min {
            min = *value;
        }
        if *value > max {
            max = *value;
        }
    }
    (min, max)
}

pub fn avg_and_total_score(scores: &HashMap<String, i32>) -> (f32, i32) {
    let mut total = 0;
    for (_, value) in scores {
        total += value;
    }
    let average = total as f32 / scores.len() as f32;
    (average, total)
}

pub fn print_scores(scores: &HashMap<String, i32>) {
    let mut total: i32 = 0;
    for (key, value) in scores {
        total += value;
        println!("{}: {}", key, value);
    }
    let avg = total as f32 / scores.len() as f32;
    println!("--------------------");
    println!("Average: {}", avg);
    println!("Total: {}", total);
}

pub fn print_hash_map(hash_map: &HashMap<String, i32>) {
    for (key, value) in hash_map {
        println!("{}: {}", key, value);
    }
}
