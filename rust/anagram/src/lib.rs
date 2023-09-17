use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let mut word_sorted: Vec<char> = word.to_lowercase().chars().collect();
    word_sorted.sort_unstable();

    for &anagram in possible_anagrams {
        // The solution cannot contain the input word
        if word.to_lowercase() == anagram.to_lowercase() {
            continue;
        }
        let mut anagram_sorted: Vec<char> = anagram.to_lowercase().chars().collect();
        anagram_sorted.sort_unstable();

        if word_sorted == anagram_sorted {
            result.insert(anagram);
        }
    }

    result
}
