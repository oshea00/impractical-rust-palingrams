#![allow(dead_code)]
use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let contents =
        fs::read_to_string("dictionary.txt").expect("dictionary.txt s/b in current directory.");
    let dictionary: HashSet<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut palingrams = HashSet::new();
    for rootword in &dictionary {
        if rootword.chars().count() > 1 {
            // left to right
            for i in 1..rootword.chars().count() + 1 {
                let reversed = reverse(&left(rootword, i));
                if dictionary.contains(&reversed) {
                    if is_palindrome(&right(rootword, i)) {
                        palingrams.insert(format!("{} {}\n", rootword, reversed));
                    }
                }
            }
            // right to left
            for i in (0..rootword.chars().count()).rev() {
                let reversed = reverse(&right(rootword, i));
                if dictionary.contains(&reversed) {
                    if is_palindrome(&left(rootword, i)) {
                        palingrams.insert(format!("{} {}\n", reversed, rootword));
                    }
                }
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    write_pairs(palingrams);
}

fn write_pairs(pairs: HashSet<String>) {
    let mut file = File::create("pairs.txt").expect("pairs.txt created.");
    for pair in pairs {
        file.write(pair.as_bytes()).expect("pairs.txt is saved.");
    }
}

fn is_palindrome(phrase: &str) -> bool {
    let reversed: String = phrase.chars().rev().collect();
    phrase == reversed
}

fn substring(str: &str, start: usize, end: usize) -> String {
    str.graphemes(true)
        .skip(start)
        .take(end - start)
        .collect::<String>()
}

fn right(str: &str, start: usize) -> String {
    str.graphemes(true).skip(start).collect::<String>()
}

fn left(str: &str, end: usize) -> String {
    str.graphemes(true).take(end).collect::<String>()
}

fn reverse(str: &str) -> String {
    str.graphemes(true).rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_true() {
        assert!(is_palindrome("ere"));
    }

    #[test]
    fn can_left() {
        assert_eq!("fiancé", left("fiancé", 6));
    }

    #[test]
    fn can_right() {
        assert_eq!("", right("fiancé", 6));
    }
    #[test]

    fn can_substring() {
        assert_eq!("y̆", substring("fiancy̆", 5, 6));
    }

    #[test]
    fn can_reverse() {
        assert_eq!("fiancé", reverse("écnaif"));
    }
}
