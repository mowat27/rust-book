use std::iter::FromIterator;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    println!("{}", piglatin("Hello, world!"));
}

enum CharType {
    Vowel(char),
    Consonant(char),
    Blank,
}

impl CharType {
    fn new(ch: &char) -> CharType {
        for vowel in VOWELS.iter() {
            if ch == vowel {
                return CharType::Vowel(*ch);
            }
        }
        CharType::Consonant(*ch)
    }
}

fn first_char(word: &str) -> CharType {
    let chars = Vec::from_iter(word.chars());
    match chars.first() {
        Option::Some(ch) => CharType::new(&ch),
        Option::None => CharType::Blank,
    }
}

fn pigvowel(word: &str) -> String {
    let mut result = String::from(word);
    result.push_str("-hay");
    result
}

fn pigconsonant(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Option::None => String::new(),
        Option::Some(first) => {
            let mut result = String::from_iter(chars);
            result.push_str(&String::from_iter(vec!['-', first, 'a', 'y'].iter()));
            result
        }
    }
}

fn pigword(word: &str) -> String {
    match first_char(word) {
        CharType::Consonant(_) => pigconsonant(word),
        CharType::Vowel(_) => pigvowel(word),
        CharType::Blank => String::new(),
    }
}

fn piglatin(sentence: &str) -> String {
    let mut result = String::new();
    for word in sentence.split(" ") {
        result.push_str(&pigword(word));
        result.push_str(" ");
    }
    String::from(result.trim_end_matches(' '))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consonant() {
        assert_eq!("irst-fay", pigword("first"));
    }

    #[test]
    fn vowel() {
        assert_eq!("apple-hay", pigword("apple"));
    }

    #[test]
    fn blank() {
        assert_eq!("", pigword(""));
    }

    #[test]
    fn sentence() {
        assert_eq!("ust-ray is-hay ard-hay", piglatin("rust is hard"));
    }
}
