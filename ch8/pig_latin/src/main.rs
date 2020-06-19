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

fn pigconsonant(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Option::None => String::new(),
        Option::Some(first) => format!("{}-{}{}", String::from_iter(chars), first, "ay"),
    }
}

fn pigword(word: &str) -> String {
    match first_char(word) {
        CharType::Consonant(_) => pigconsonant(word),
        CharType::Vowel(_) => format!("{}-hay", word),
        CharType::Blank => String::new(),
    }
}

fn piglatin(sentence: &str) -> String {
    let pigwords: Vec<String> = sentence.split(' ').map(|word| pigword(word)).collect();
    pigwords.join(" ")
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
    #[test]
    #[ignore]
    fn punctuation() {
        assert_eq!("ust-ray, is-hay ard-hay!", piglatin("rust, is hard!"));
    }
}
