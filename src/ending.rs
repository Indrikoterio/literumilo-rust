// Define the grammatical endings (finaĵoj) of Esperanto words.
// Klivo 2020-05-08

#![allow(dead_code)]

use super::entry::*;

#[derive(Debug)]
pub struct Ending<'a> {
    pub ending: &'a str,
    pub length: usize,
    pub pos: POS,	// Part of Speech: substantive (noun), adjective, verb, etc.
}

const SUB_O: Ending = Ending { ending: "o", length: 1, pos: POS::Substantive, };
const SUB_ON: Ending = Ending { ending: "on", length: 2, pos: POS::Substantive, };
const SUB_OJ: Ending = Ending { ending: "oj", length: 2, pos: POS::Substantive, };
const SUB_OJN: Ending = Ending { ending: "ojn", length: 3, pos: POS::Substantive, };
const VERB_IS : Ending = Ending { ending: "is", length: 2, pos: POS::Verb, };
const VERB_AS : Ending = Ending { ending: "as", length: 2, pos: POS::Verb, };
const VERB_OS : Ending = Ending { ending: "os", length: 2, pos: POS::Verb, };
const VERB_I : Ending = Ending { ending: "i", length: 1, pos: POS::Verb, };
const VERB_U : Ending = Ending { ending: "u", length: 1, pos: POS::Verb, };
const VERB_US: Ending = Ending { ending: "us", length: 2, pos: POS::Verb, };
const ADJ_A: Ending = Ending { ending: "a", length: 1, pos: POS::Adjective, };
const ADJ_AN: Ending = Ending { ending: "an", length: 2, pos: POS::Adjective, };
const ADJ_AJ: Ending = Ending { ending: "aj", length: 2, pos: POS::Adjective, };
const ADJ_AJN: Ending = Ending { ending: "ajn", length: 3, pos: POS::Adjective, };
const ADV_E: Ending = Ending { ending: "e", length: 1, pos: POS::Adverb, };
const ADV_EN: Ending = Ending { ending: "en", length: 2, pos: POS::Adverb, };


impl Ending<'_> {

    // new - Checks to see if the original word has a valid grammatical ending.
    // Returns and Option containing an Ending struct, or None.

    pub fn new(original_word: &str) -> Option<Ending> {

        let length = original_word.chars().count();
        let mut characters = original_word.chars().rev();

        if let Some(last) = characters.next() {
            if length < 3 { return None; }
            if last == 'o' {
                return Some(SUB_O);
            }
            else if last == 's' {
                if length < 4 { return None; }
                if let Some(second_last) = characters.next() {
                    match second_last {
                        'a' => return Some(VERB_AS),
                        'i' => return Some(VERB_IS),
                        'o' => return Some(VERB_OS),
                        'u' => return Some(VERB_US),
                        _ => return None,
                    }
                }
            }
            else if last == 'n' {
                if length < 4 { return None; }
                if let Some(second_last) = characters.next() {
                    if second_last == 'o' { return Some(SUB_ON); }
                    else if second_last == 'a' { return Some(ADJ_AN); }
                    else if second_last == 'e' { return Some(ADV_EN); }
                    else if second_last == 'j' {
                        if length < 5 { return None; }
                        if let Some(third_last) = characters.next() {
                            if third_last == 'o' { return Some(SUB_OJN); }
                            else if third_last == 'a' { return Some(ADJ_AJN); }
                        }
                        return None;
                    }
                    return None;
                }
            }
            else if last == 'j' {
                if length < 4 { return None; }
                if let Some(second_last) = characters.next() {
                   if second_last == 'o' { return Some(SUB_OJ); }
                   else if second_last == 'a' { return Some(ADJ_AJ); }
                   return None;
                }
            }
            match last {
                'a' => return Some(ADJ_A),
                'e' => return Some(ADV_E),
                'i' => return Some(VERB_I),
                'u' => return Some(VERB_U),
                _ => return None,
            }
        }
        else { None }
    }
}
