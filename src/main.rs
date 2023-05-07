use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let contents =
        fs::read_to_string("dictionary.txt").expect("Something went wrong reading the file");
    let dictionary: HashSet<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut palingrams = HashSet::new();
    for rootword in &dictionary {
        // Sidestepping utf_8 strings with multi-byte chars in the dictionary
        if rootword.len() > 1 && (rootword.chars().count() == rootword.len()) {
            // left to right
            for i in 1..rootword.len() + 1 {
                let reversed = rootword[..i].chars().rev().collect::<String>();
                if dictionary.contains(&reversed) {
                    if is_palindrome(&rootword[i..]) {
                        palingrams.insert(format!("{} {}\n", rootword, reversed));
                    }
                }
            }
            // right to left
            for i in (0..rootword.len()).rev() {
                let reversed = rootword[i..].chars().rev().collect::<String>();
                if dictionary.contains(&reversed) {
                    if is_palindrome(&rootword[..i]) {
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
    let mut file = File::create("pairs.txt").expect("Couldn't create file");
    for pair in pairs {
        file.write(pair.as_bytes()).expect("Couldn't write file");
    }
}

fn is_palindrome(phrase: &str) -> bool {
    let reversed: String = phrase.chars().rev().collect();
    phrase == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_true() {
        assert!(is_palindrome("ere"));
    }
}
