// check_word - (kontrolu_vorton)
// This file has functions which check the spelling of an Esperanto word.
// Klivo 2020-05-09

#![allow(dead_code)]

use std::collections::HashMap;

use literumilo::*;

use super::suffix::*;
use super::entry::*;
use super::ending::*;
use super::morpheme_list::*;
use super::scan_morphemes::*;

const  MAX_MORPHEMES: usize = 9; // The maximum number of morphemes in a compound word.

/*
 * AnalysisResult
 * Fields:
 *     'word' has the original word divided into morphemes, eg. 'mis.dir.it.a'.
 *     'valid' is true if the word is a valid Esperanto word. (correctly spelled)
 */
pub struct AnalysisResult {
    pub word: String,
    pub valid: bool,
}

impl AnalysisResult {

    /*
     * new()
     *
     * Params:
     *    original word
     *    word  (divided into morphemes)
     *    valid  (true or false)
     * Returns:
     *    analysis result
     */
    fn new(original: &str, word: &str, valid: bool) -> AnalysisResult {
        let word2 = restore_capitals(original, word);
        return AnalysisResult { word: word2, valid: valid };
    }
}


/*
 * check_synthesis  (kontrolu sintezon)
 *
 * This method checks the synthesis of suffixes when they are found,
 * and other morphemes (prefixes, roots) after the word has been
 * completely divided, by calling scan_morphemes().
 *
 * Params:
 *      rest of word
 *      dictionary
 *      index of morpheme (int)
 *      list of morphemes
 *      last_morpheme (t/f)
 * Return:
 *      true if valid, false otherwise
 */
fn check_synthesis(rest_of_word: &str, dictionary: &HashMap<String, Entry>, index: usize,
                            morpheme_list: &mut Morphemes<'_>, last_morpheme: bool) -> bool {
    let syn;
    let word;

    if let Some(entry) = morpheme_list.get(index) {
        syn = entry.synthesis;
        word = entry.word.clone();
    }
    else { return false; }

    if syn == Synthesis::Suffix && !check_suffix(&word, index, morpheme_list) { return false; }

    if !last_morpheme {
        // Divide the rest of the word into morphemes.
        if !find_morpheme(rest_of_word, dictionary, index + 1, morpheme_list) { return false; }
        return true;
    }

    if last_morpheme {
        // Check prefixes (and limited morphemes) after the word has been divided,
        // because the validity of a prefix depends on the morphemes which come after it.
        return scan_morphemes(morpheme_list);
    }

    return false;

}  // check_synthesis


/*
 * find_morpheme - trovu_radikon
 *
 * This function divides a (presumably) compound word into morphemes, while checking synthesis.
 * It is recursive.
 *
 * Params:
 *    rest_of_word - the remainder to be analyzed
 *    dictionary - a map of word data
 *    index of morpheme (indekso de radiko)
 *    morpheme_list - holds a vector of previously collected morphemes
 * Return:
 *    true for valid synthesis, false for invalid.
 */
fn find_morpheme(rest_of_word: &str, dictionary: &HashMap<String, Entry>,
                            index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index >= MAX_MORPHEMES { return false; }

    if index > 0 {
        if let Some(entry) = dictionary.get(rest_of_word) {
            // Do we allow this morpheme to join with others?
            if entry.synthesis != Synthesis::No {
                morpheme_list.put(index, entry);
                //println!("Rest of word: {}", rest_of_word);
                let valid = check_synthesis(&rest_of_word, dictionary, index, morpheme_list, true);
                if valid { return true; }
            }
        }
    }

    let length_of_word = rest_of_word.chars().count();
    let min_length = 2;  // minimum length of a morpheme
    let max_length = length_of_word - 1;

    // Try to find a valid morpheme, by dividing the rest of the word.
    for size in (min_length .. max_length).rev() {
        let morpheme: String = rest_of_word.chars().take(size).collect();
        if let Some(entry) = dictionary.get(&morpheme) {
            // Do we allow this morpheme to join with others?
            if entry.synthesis != Synthesis::No {
                let rest_of_word: String = rest_of_word.chars().skip(size).collect();
                morpheme_list.put(index, entry);
                let valid = check_synthesis(&rest_of_word, dictionary, index, morpheme_list, false);
                if valid {
                    return true;
               }
            }
        } // end if let
    } // end for

    // Sometimes there is a separator (a grammatical ending) between morphemes.
    // This is usually done to aid pronunciation. Instead of 'fingr.montri.', most would
    // write 'fingr.o.montr.i'. Other examples are: ĝust.a.temp.e, unu.a.foj.e, etc.
    // This algorithm will accept one separator per word. It must be 'o', 'a' or 'e'.

    if index == 0 || length_of_word < 3 { return false; }

    let separator: String = rest_of_word.chars().take(1).collect();
    if let Some(entry) = Entry::new_separator(&separator) {
        morpheme_list.put(index, &entry);
        let rest_of_word: String = rest_of_word.chars().skip(1).collect();
        let valid = check_synthesis(&rest_of_word, dictionary, index, morpheme_list, false);
        if valid { return true; }
    }

    return false;

}  // find_morpheme


/*
 * check_word
 *
 * This function tests whether a word is correctly spelled.
 *
 * Params:
 *    word - the word to test
 *    dictionary - a map of word data
 * Return:
 *     AnalysisResult
 */
pub fn check_word(original_word: &str, dictionary: &HashMap<String, Entry>) -> AnalysisResult {

    let length_of_word = original_word.chars().count();

    if length_of_word == 1 {   // Single letters are OK.
        let chrs:Vec<char> = original_word.chars().collect();
        if is_word_char!(chrs[0]) {
            return AnalysisResult::new(original_word, original_word, true);
        }
        else {
            return AnalysisResult::new(original_word, original_word, false);
        }
    }

    // Check for abbreviations, such as n-r.oj, s-in.oj
    // The second character must be a hyphen.
    if length_of_word > 2 {
        let chrs:Vec<char> = original_word.chars().collect();
        let second_char = chrs[1];
        if is_hyphen!(second_char) {
            let word = original_word.to_lowercase();
            if let Some(_entry) = dictionary.get(&word) {
                return AnalysisResult::new(original_word, &word, true);
            }
            else {
                return AnalysisResult::new(original_word, &word, false);
            }
        }
    }

    let original_word = remove_hyphens(&original_word);
    let word = original_word.to_lowercase();
    let length_of_word = word.chars().count();

    // Exceptions.
    // A few words cause difficulties for the algorithm, especially accusative pronouns.
    // For example, the pronoun 'vin' means 'you' (accusative), but it is also the root for 'wine' (vino).
    // I want the pronoun to divided as 'vi.n' and the beverage to be 'vin.o' (not vi.n.o). The dictionary
    // has 'vin' as a key, but the keys in a dictionary must be unique. To solve this problem, some
    // pronouns (etc.) will be excluded from the dictionary, and handled as exceptions here.

    if length_of_word < 5 {
        let w;
        match &word[..] {
            "ĝin" => w = String::from("ĝi.n"),
            "lin" => w = String::from("li.n"),
            "min" => w = String::from("mi.n"),
            "sin" => w = String::from("si.n"),
            "vin" => w = String::from("vi.n"),
            "lian" => w = String::from("li.an"),
            "cian" => w = String::from("ci.an"),
            _ => w = String::from(""),
        }
        if w.len() > 0 {
            return AnalysisResult::new(&original_word, &w, true);
        }
    }

    // First, check the dictionary for words which have no grammatical ending, eg. 'ne', 'dum', 'post'.
    if let Some(entry) = dictionary.get(&word) {
        if entry.without_ending == WithoutEnding::Yes {
            return AnalysisResult::new(&original_word, &entry.word, true);
        }
    }

    let word_iter = word.chars();

    // Most words have a grammatical ending, eg. elefant-ojn, trov-is.
    // Attempt to remove a grammatical ending, then search for the root, (elefant, trov).
    if let Some(ending) = Ending::new(&word) {
        let length = length_of_word - ending.length;
        let word_without_ending: String = word_iter.take(length).collect();

        // Try to find the root in the dictionary.
        if let Some(entry) = dictionary.get(&word_without_ending) {
            if entry.with_ending == WithEnding::Yes {
                let w = format!("{}.{}", &entry.word, ending.ending);
                return AnalysisResult::new(&original_word, &w, true);
            }
        }

        // The root was not found. Maybe it's a compound word.
        // Analyze.

        // The morpheme list needs the ending for later analysis.
        let mut morpheme_list = Morphemes::new(ending);

        let valid: bool = find_morpheme(&word_without_ending, dictionary, 0, &mut morpheme_list);

        if valid {
            return AnalysisResult::new(&original_word, &morpheme_list.display_form(), true);
        }
        else {
            return AnalysisResult::new(&original_word, &word, false);
        }

    }
    else {
        return AnalysisResult::new(&original_word, &word, false);
    }  // No ending.

}  // check_word
