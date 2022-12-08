use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

use regex::Regex;

fn word_count(words: &String) -> HashMap<String, u32> {
    let mut _map: HashMap<String, u32> = HashMap::new();
    let _regex_word = Regex::new(r#"(\w'\w|[0-9a-zA-Z])+"#).unwrap();

    for _word_match in _regex_word.find_iter(words) {
        let _word = _word_match.as_str().to_lowercase();
        _map.entry(_word).and_modify(|counter| *counter +=1).or_insert(1);
    }
    return _map;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let time_start = Instant::now();

    // Reading
    let file_contents = fs::read_to_string(file_path)
        .expect("Error reading file");
    let time_read = Instant::now();

    // Processing
    let count_map = word_count(&file_contents);
    let time_process = Instant::now();

    // Sorting
    let mut count_vector: Vec<(String, u32)> = count_map.into_iter().collect();
    count_vector.sort_by(|(_, count1), (_, count2)| count1.cmp(&count2).reverse());
    let time_sort = Instant::now();

    // Outputting
    for (word, count) in count_vector {
        println!("{}\t{}", word, count);
    }
    let time_end = Instant::now();

    println!("==============================");
    println!("Reading:\t{:?}", time_read.duration_since(time_start));
    println!("Processing:\t{:?}", time_process.duration_since(time_read));
    println!("Sorting:\t{:?}", time_sort.duration_since(time_process));
    println!("Outputting:\t{:?}", time_end.duration_since(time_sort));
    println!("------------------------------");
    println!("TOTAL:\t\t{:?}", time_end.duration_since(time_start));
    println!("==============================");

}
