use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::time::Duration;
use rand::{Rng};

#[derive(Serialize, Deserialize, Debug)]
struct SimpleWord {
    lang_code: String,
    word: String,
    #[serde(default)]
    other_pos: Vec<String>,
    pos: String,
}

impl PartialEq for SimpleWord {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}

fn from_wiktionary() -> Vec<SimpleWord> {
    let lines = BufReader::new(File::open("data/de-extract.json").unwrap()).lines();
    let name = "name".to_string();
    lines
        .into_iter()
        .filter_map(|l| l.ok().and_then(|l| serde_json::from_str(&l).ok()))
        .filter(|x: &SimpleWord| {
            x.lang_code.as_str() == "de" && !x.other_pos.contains(&name) && x.pos != "abbrev"
        })
        .collect::<Vec<_>>()
}

fn from_wortliste<P : AsRef<Path>>(path: P) -> Vec<SimpleWord> {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    lines.into_iter().filter_map(|l| l.ok().map(|x| SimpleWord {
        word: x,
        lang_code: "de".to_string(),
        other_pos: vec![],
        pos: String::new()
    })).collect::<_>()
}

fn main() {

    // let words = from_wortliste("data/german/german.dic");
    let words = from_wiktionary();

    get_and_print_result(
        &words,
        strict_ascending,
        "Wörter mit alphabetisch sortierten Buchstaben:\n",
    );
    get_and_print_result(
        &words,
        duplicate_ascending,
        "Wörter mit alphabetisch sortierten Buchstaben (inklusive Duplikaten):\n",
    );

    get_and_print_result(
        &words,
        strict_descending,
        "Wörter mit invers alphabetisch sortierten Buchstaben:\n",
    );
    get_and_print_result(
        &words,
        duplicate_descending,
        "Wörter mit invers alphabetisch sortierten Buchstaben (inklusive Duplikaten):\n",
    );
}

fn get_and_print_result<F: Fn(&[char]) -> bool>(
    words: &Vec<SimpleWord>,
    comparison: F,
    text: &str,
) {
    let mut result = words
        .into_iter()
        .filter(|w| analyze_in_order(&w.word, &comparison))
        .collect::<Vec<&SimpleWord>>();
    result.sort_by(|a, b| b.word.len().cmp(&a.word.len()));
    result.dedup();
    thread::sleep(Duration::from_millis(200));
    println!("\n\n{}", text);
    thread::sleep(Duration::from_millis(200));
    println!("Ergebnisse: {}", result.len());
        let mut r = result[0..10]
            .iter()
            .enumerate()
            .map(|(i, x)| format!("{}: {} ({})", i + 1, x.word, x.word.len())).collect::<Vec<_>>();
    r[0] = String::from("1. ???");
    for r in r {
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(0..250)));
        println!("{}", r);
    }
}

fn strict_ascending(input: &[char]) -> bool {
    input[0] < input[1]
}

fn strict_descending(input: &[char]) -> bool {
    !duplicate_ascending(input)
}

fn duplicate_ascending(input: &[char]) -> bool {
    input[0] <= input[1]
}

fn duplicate_descending(input: &[char]) -> bool {
    !strict_ascending(input)
}


fn analyze_in_order<F: Fn(&[char]) -> bool>(word: &str, comparison: F) -> bool {
    if !word.chars().all(|c| c.is_alphabetic()) {
        return false;
    }
    let ch = word.to_ascii_lowercase().chars().collect::<Vec<char>>();
    for w in ch.windows(2) {
        if w.len() < 2 || !comparison(w) {
            return false;
        }
    }
    return true;
}
