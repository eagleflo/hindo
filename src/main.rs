use std::collections::HashMap;
use std::env;
use std::fs;

fn is_kanji(c: &char) -> bool {
    (*c >= '\u{4e00}' && *c <= '\u{9fff}') || // CJK Unified Ideographs
        (*c >= '\u{f900}' && *c <= '\u{faff}') // CJK Compatibility Ideographs
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let text = fs::read_to_string(file_path).expect("Not able to read file");

    let mut kanji: HashMap<char, u32> = HashMap::new();
    for character in text.chars() {
        if is_kanji(&character) {
            *kanji.entry(character).or_insert(0) += 1;
        }
    }

    let mut result = Vec::from_iter(kanji);
    result.sort_by(|(_, a), (_, b)| b.cmp(a));
    for (character, count) in result {
        println!("{character}: {count}");
    }
}
