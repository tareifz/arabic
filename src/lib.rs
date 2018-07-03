pub fn is_diacritic(c: &char) -> bool {
    match *c {
        'َ' | 'ُ' | 'ِ' | 'ْ' |
        'ً' | 'ٌ' | 'ٍ' | 'ّ' => true,
        _ => false,
    }
}

pub fn remove_diacritics(s: &str) -> String {
    s.chars().filter(|c| {
        !is_diacritic(&c)
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_diacritic() {
        assert!(is_diacritic(&'ً'),
                "Error: is_diacritic() returned 'false' on Arabic diacritic!");
        assert!(!is_diacritic(&'أ'),
                "Error: is_diacritic() returned 'true' on Arabic letter!");
        assert!(!is_diacritic(&'h'),
                "Error: is_diacritic() returned 'true' on English Letter!");
        assert!(!is_diacritic(&','),
                "Error: is_diacritic() returned 'true' on Punctuation!");
        assert!(!is_diacritic(&'1'),
                "Error: is_diacritic() returned 'true' on Number!");
    }

    #[test]
    fn test_remove_diacritics() {
        assert_eq!(remove_diacritics("السَّلامُ عَلَيْكُم"), "السلام عليكم",
                   "Error: remove_diacritics() didn't remove diacritics from Arabic String slice!");
        assert_eq!(remove_diacritics(&"السَّلامُ عَلَيْكُم".to_string()), "السلام عليكم",
                   "Error: remove_diacritics() didn't remove diacritics from Arabic String!");
        assert_eq!(remove_diacritics("السلام عليكم"), "السلام عليكم",
                   "Error: remove_diacritics() did something wrong with String slice that doesn't contain diacritics!");
        assert_eq!(remove_diacritics("abcd"), "abcd",
                   "Error: remove_diacritics() did something wrong with English String slice!");
        assert_eq!(remove_diacritics("123السلام عليكم123-^%$"), "123السلام عليكم123-^%$",
                   "Error: remove_diacritics() did something wrong with String slice containing numbers, letters, signs!");
        assert_eq!(remove_diacritics(&String::from("")), "",
                   "Error: remove_diacritics() did something wrong with empty String!");
    }
}
