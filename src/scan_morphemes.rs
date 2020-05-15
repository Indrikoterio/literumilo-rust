// scan_morphemes.rs - contains functions to check the synthesis of prefixes,
// participle endings, and morphemes with limited combinability. Suffixes are
// checked by another module.
//
// A compound word such as 'mis-kompren-it-(a)' consists of 3 morphemes,
// (excluding the grammatical ending), which are stored in a vector called
// morpheme_list. Whether a prefix (mis-) is valid or not depends on the 
// morphemes which come after it. Whether a participle ending (-it ) is valid
// or not depends on the morphemes which come before it.
//
// Note: Some of the functions below may modify the entries in morpheme_list.
//
// From Stackoverflow:
// Q: "How do I extract two mutable elements from a Vec in Rust?"
// A: "Rust can't guarantee at compile time that get_mut is not going to mutably
// borrow the same element twice, so get_mut borrows the entire vector mutably.
// Instead, use slices."
//
// Klivo 2020-05-08

use super::entry::*;
use super::morpheme_list::*;

/*
 * check_bo()
 *
 * Check prefix bo-, meaning 'in-law'. Eg. bo-patr-o (father-in-law).
 * 
 * For a description of parameters see check_acx().
 */
fn check_bo(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }
    let difference = morpheme_list.last_index() - index;

    if difference > 0 {  // Make sure at least one morpheme follows.
        if let Some(next_entry) = &morpheme_list.get(index + 1) {
            if next_entry.meaning == Meaning::Parenco { return true; }
        }
    }
    return false;

}  // check_bo


/*
 * check_cis()
 *
 * Check prefix cis-, meaning 'on the near side'.
 * Rare. Used for mountains and rivers. Eg. 'cisalpa' (cisalpine).
 * 
 * For a description of parameters see check_acx().
 */
fn check_cis(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }
    let difference = morpheme_list.last_index() - index;

    if difference > 0 {  // Make sure at least one morpheme follows.
        if let Some(next_entry) = &morpheme_list.get(index + 1) {
            let meaning = next_entry.meaning;
            match meaning {
                Meaning::Rivero => return true,
                Meaning::Monto => return true,
                Meaning::Montaro => return true,
                _ => return false,
            }
        }
    }
    return false;

}  // check_cis


/*
 * check_cxi()
 *
 * Check prefix ĉi-, meaning 'this here'. Eg. 'ĉi-vesper-e' (this evening, adv.).
 * Valid only for adjectives and adverbs.
 * 
 * For a description of parameters see check_acx().
 */
fn check_cxi(index: usize, morpheme_list: &Morphemes) -> bool {

    if index == 0 {
        let type_of_ending: POS = morpheme_list.type_of_ending();
        if type_of_ending == POS::Adjective || type_of_ending == POS::Adverb { return true }
    }
    return false;

}  // check_cxi



/*
 * check_eks()
 *
 * Check prefix eks-, meaning 'ex-'. Eg. 'eksprezidento' (expresident).
 * Valid only for people.
 * 
 * For a description of parameters see check_acx().
 */
fn check_eks(index: usize, morpheme_list: &Morphemes) -> bool {

    if index != 0 { return false; }

    let last = morpheme_list.last_index();
    let mut n = index + 1;
    while n <= last {
        if let Some(entry) = &morpheme_list.get(n) {
            if is_person(entry.meaning) { return true; }
        }
        n += 1
    }
    return false;

}  // check_eks

/*
 * check_ge()
 *
 * Check prefix ge-, meaning 'both sexes'. Valid for people and animals.
 * Eg. 'ge-student-oj' (male and female students), 'ge-frat-oj' (brothers and sisters).
 * 
 * For a description of parameters see check_acx().
 */
fn check_ge(index: usize, morpheme_list: &Morphemes) -> bool {

    if index != 0 { return false; }

    let last = morpheme_list.last_index();

    let mut n = index + 1;
    while n <= last {
        if let Some(entry) = &morpheme_list.get(n) {
            let meaning = entry.meaning;
            if is_person(meaning) || is_animal(meaning) { return true; }
        }
        n += 1;
    }
    return false;

}  // check_ge



/*
 * check_kun
 *
 * Check kun-, meaning 'together with'. Eg. 'kun-ir-is', (went together).
 *
 * For a description of parameters see check_acx().
 */
fn check_kun(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }
    if  check_prepositional_prefix(index, morpheme_list) { return true; }

    let last = morpheme_list.last_index();
    let mut n = index + 1;
    while n <= last {
        if let Some(entry) = &morpheme_list.get(n) {
            if entry.part_of_speech == POS::Substantive { return true; }
        }
        n += 1;
    }
    return false;

}  // check_kun


/*
 * check_mal
 *
 * Check prefix mal-, meaning 'un'.
 * Eg. 'mal-feliĉ-a' (unhappy), 'mal-kompren-as' (misunderstands).
 *
 * For a description of parameters see check_acx().
 */
fn check_mal(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }

    let last = morpheme_list.last_index();
    let type_of_ending: POS = morpheme_list.type_of_ending();

    if last > 0 &&
       (type_of_ending == POS::Verb ||
        type_of_ending == POS::Adjective ||
        type_of_ending == POS::Adverb) {
        return true;
    }

    let mut n = index + 1;
    while n <= last {
        if let Some(entry) = &morpheme_list.get(n) {
            let pos = entry.part_of_speech;
            if pos == POS::Verb ||
               pos == POS::SubstantiveVerb ||
               pos == POS::Adjective { return true; }
        }
        n += 1;
    }
    return false;

}  // check_mal


/*
 * check_ne
 *
 * Check prefix ne-, meaning 'not'.
 * Eg. 'ne-far-ebla', (not doable), 'ne-taŭg-ul-o' (unsuitable person).
 *
 * For a description of parameters see check_acx().
 */
fn check_ne(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }

    let last = morpheme_list.last_index();
    let type_of_ending: POS = morpheme_list.type_of_ending();

    if last > 0 &&
       (type_of_ending == POS::Adjective || type_of_ending == POS::Adverb) {
        return true;
    }

    let mut n = index + 1;
    while n <= last {
        if let Some(entry) = &morpheme_list.get(n) {
            let pos = entry.part_of_speech;
            if pos == POS::Adjective { return true; }
            if pos == POS::Participle { return true; }
            let entry_str = &entry.word[..];
            if entry_str == "ad" { return true; }   // ne.uz.ad.o, ne.far.ad.o are valid.
            if entry_str == "ec" { return true; }
        }
        n += 1;
    }
    return false;

}  // check_ne


/*
 * check_po
 *
 * Check prefix po-, meaning 'apiece, at a rate of'. (It's complicated.)
 * Eg. 'po-pec-e', (by pieces).
 *
 * For a description of parameters see check_acx().
 */
fn check_po(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }

    let last = morpheme_list.last_index();
    let type_of_ending: POS = morpheme_list.type_of_ending();
    if last > 0 && type_of_ending == POS::Adverb { return true; }

    return false;

}  // check_po


/*
 * check_pra
 *
 * Check prefix pra-, meaning 'prehistoric, ancient, great'.
 * Eg. 'pra-ul-o', (ancestor), 'pra-nep-o' (great-grandson), 'pra-hom-o' (primitive man).
 *
 * For a description of parameters see check_acx().
 */
fn check_pra(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }

    let last = morpheme_list.last_index();
    let diff = last - index;

    if diff > 0 {
        if let Some(next_entry) = &morpheme_list.get(index + 1) {
            let pos = next_entry.part_of_speech;
            if pos == POS::Substantive || pos == POS::SubstantiveVerb { return true; }
        }
    }

    if diff > 1 {
        if let Some(next_entry) = &morpheme_list.get(index + 2) {
            let pos = next_entry.part_of_speech;
            if pos == POS::Participle { return true; }
        }
    }
    return false;

}  // check_pra


/*
 * check_pseuxdo
 *
 * Check prefix pseuxdo-, meaning 'false'.
 * Eg. 'pseŭdo-scienc-o', (pseudoscience),
 *
 * For a description of parameters see check_acx().
 */
fn check_pseuxdo(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }

    let last = morpheme_list.last_index();
    let diff = last - index;

    if diff > 0 {
        if let Some(next_entry) = &morpheme_list.get(index + 1) {
            let pos = next_entry.part_of_speech;
            match pos {
                POS::Substantive => return true,
                POS::SubstantiveVerb => return true,
                POS::Adjective => return true,
                _ => return false,
            }
        }
    }
    return false;

}  // check_pseuxdo


/*
 * check_sen
 *
 * Check sen-, meaning 'without'.
 * Eg. 'sen-interes-a' (without interest, uninteresting),
 * 'sen-hom-ej-o', (a place without people).
 *
 * For a description of parameters see check_acx().
 */
fn check_sen(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }
    if  check_prepositional_prefix(index, morpheme_list) { return true; }

    let last = morpheme_list.last_index();

    if let Some(last_entry) = &morpheme_list.get(last) {
        let w = &last_entry.word[..];
        let valid = match w {
            "ul" => true,
            "aĵ" => true,
            "ej" => true,
            _ => false,
        };
        return valid;
    }

    return false;

}  // check_sen


/*
 * check_sin
 *
 * Check sin-, meaning 'self'. Because this is reflexive, it is only attached
 * to transitive verbs.
 * Eg. 'sin-kritik-em-a' (tending to criticise his/herself),
 * 'sin-re-vok-a', (recursive, calling itself).
 *
 * For a description of parameters see check_acx().
 */
fn check_sin(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index != 0 { return false; }

    let last = morpheme_list.last_index();

    let mut n = index + 1;
    while n <= last {
        if let Some(next_entry) = &morpheme_list.get(n) {
            if next_entry.transitivity == Transitivity::Transitive { return true; }
        }
        n += 1;
    }
    return false;

}  // check_sin


/*
 * check_sub_super_sur
 *
 * Check prefixes sub- (under) and sur- (on).
 * Eg. 'sub-mar-a' (under sea), 'sur-tabl-e', (on the table).
 *
 * For a description of parameters see check_acx().
 */
fn check_sub_super_sur(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if  check_prepositional_prefix(index, morpheme_list) { return true; }

    let last = morpheme_list.last_index();
    let type_of_ending: POS = morpheme_list.type_of_ending();

    if last > 0 &&
       (type_of_ending == POS::Substantive ||
        type_of_ending == POS::SubstantiveVerb) {
        return true;
    }
    return false;

}  // check_sub_super_sur


/*
 * check_prepositional_prefix
 *
 * (Reference. PIV 373)
 * Eg. 'kun-ir-is', (went together), 'antaŭ-dir-is' (said before).
 *
 * For a description of parameters see check_acx().
 */
fn check_prepositional_prefix(index: usize, morpheme_list: &mut Morphemes) -> bool {

    let last = morpheme_list.last_index();
    let type_of_ending: POS = morpheme_list.type_of_ending();

    if last > 0 &&
       (type_of_ending == POS::Adjective || type_of_ending == POS::Adverb) {
        return true;
    }

    let mut n = index + 1;
    while n <= last {
        if let Some(entry) = &morpheme_list.get(n) {
            let pos = entry.part_of_speech;
            if pos == POS::Verb || pos == POS::SubstantiveVerb { return true; }
        }
        n += 1;
    }
    return false;
}  // check_prepositional_prefix


/*
 * check_adverbial_prefix
 *
 * Check adverbial prefixes. Eg. 'for-ir-is', (went away), 'mis-dir-is' (misspoke).
 * Adverbs modify a verb root.
 *
 * For a description of parameters see check_acx().
 */
fn check_adverbial_prefix(index: usize, morpheme_list: &mut Morphemes) -> bool {

    let last = morpheme_list.last_index();
    let type_of_ending: POS = morpheme_list.type_of_ending();

    if last > 0 && type_of_ending == POS::Verb { return true; }

    let mut n = index + 1;
    while n <= last {
        if let Some(entry) = &morpheme_list.get(n) {
            let pos = entry.part_of_speech;
            if pos == POS::Verb || pos == POS::SubstantiveVerb { return true; }
        }
        n += 1;
    }
    return false;
}  // check_adverbial_prefix


/*
 * check_first
 *
 * A long prefix, such as 'antaŭ' (before) or 'inter', is valid if it is the first
 * morpheme in the list.
 *
 * For a description of parameters see check_acx().
 */
fn check_first(index: usize, _morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return true; }
    return false;
}


/*
 * check_prefix - Checks synthesis of a prefix.
 *
 * Params:
 *    prefix as string
 *    index of morpheme in morpheme list
 *    morpheme list (vector of dictionary entries)
 * Return:
 *    true for valid synthesis, false otherwise
 */
fn check_prefix(_prefix: &str, index: usize, morpheme_list: &mut Morphemes) -> bool {

    // Check technical prefixes such as 'hiper' and 'mega'.
    // These are only valid at the front of a word.
    if let Some(entry) = &morpheme_list.get(index) {
        if entry.part_of_speech == POS::TechPrefix {
            if index == 0 { return true; }
            return false;
        }
    }

    match _prefix {
        "al" => return check_prepositional_prefix(index, morpheme_list),
        "anstataŭ" => return check_first(index, morpheme_list),
        "antaŭ" => return check_first(index, morpheme_list),
        "apud" => return check_prepositional_prefix(index, morpheme_list),
        "bo" => return check_bo(index, morpheme_list),
        "cis" => return check_cis(index, morpheme_list),
        "ĉe" => return check_prepositional_prefix(index, morpheme_list),
        "ĉi" => return check_cxi(index, morpheme_list),
        "ĉirkaŭ" => return check_first(index, morpheme_list),
        "de" => return check_prepositional_prefix(index, morpheme_list),
        "dis" => return check_adverbial_prefix(index, morpheme_list),
        "dum" => return check_prepositional_prefix(index, morpheme_list),
        "ek" => return check_adverbial_prefix(index, morpheme_list),
        "eks" => return check_eks(index, morpheme_list),
        "ekster" => return check_first(index, morpheme_list),
        "el" => return check_prepositional_prefix(index, morpheme_list),
        "en" => return check_prepositional_prefix(index, morpheme_list),
        "for" => return check_adverbial_prefix(index, morpheme_list),
        "ge" => return check_ge(index, morpheme_list),
        "ĝis" => return check_prepositional_prefix(index, morpheme_list),
        "inter" => return check_first(index, morpheme_list),
        "kontraŭ" => return check_first(index, morpheme_list),
        "krom" => return check_first(index, morpheme_list),
        "kun" => return check_kun(index, morpheme_list),
        "laŭ" => return check_prepositional_prefix(index, morpheme_list),
        "mal" => return check_mal(index, morpheme_list),
        "mis" => return check_adverbial_prefix(index, morpheme_list),
        "ne" => return check_ne(index, morpheme_list),
        "per" => return check_prepositional_prefix(index, morpheme_list),
        "pli" => return check_adverbial_prefix(index, morpheme_list),
        "po" => return check_po(index, morpheme_list),
        "por" => return check_prepositional_prefix(index, morpheme_list),
        "post" => return check_prepositional_prefix(index, morpheme_list),
        "pra" => return check_pra(index, morpheme_list),
        "preter" => return check_prepositional_prefix(index, morpheme_list),
        "pri" => return check_prepositional_prefix(index, morpheme_list),
        "pro" => return check_prepositional_prefix(index, morpheme_list),
        "pseŭdo" => return check_pseuxdo(index, morpheme_list),
        "re" => return check_adverbial_prefix(index, morpheme_list),
        "retro" => return check_first(index, morpheme_list),
        "sen" => return check_sen(index, morpheme_list),
        "sin" => return check_sin(index, morpheme_list),
        "sub" => return check_sub_super_sur(index, morpheme_list),
        "super" => return check_sub_super_sur(index, morpheme_list),
        "sur" => return check_sub_super_sur(index, morpheme_list),
        "tra" => return check_prepositional_prefix(index, morpheme_list),
        "trans" => return check_prepositional_prefix(index, morpheme_list),
        _ => return false,
    }

}  // check_prefix


/*
 * check_limited_synthesis
 *
 * Some morphemes, because they are short, can cause problems for the spell checker.
 * To solve the problem, many short morphemes, have limited combinability.
 *
 * A limited verb-morpheme may combine only with a suffix and/or a prefix.
 * A limited animal-morpheme may combine only with vir-, -in, -id ktp.
 * A limited relationship-morpheme (father, brother) may combine only with pra-, ge- or -in.
 * A limited ethnicity-morpheme may combine with -in, -land, etc.
 *
 * Params:
 *    morpheme as string
 *    index of morpheme in morpheme list
 *    morpheme list (vector of dictionary entries)
 * Return:
 *    true for valid synthesis, false otherwise
 */
fn check_limited_synthesis(_morpheme: &str, index: usize, morpheme_list: &mut Morphemes) -> bool {

    let last = morpheme_list.last_index();
    let pos;
    let meaning;

    if let Some(entry) = &morpheme_list.get(index) {
        pos = entry.part_of_speech;
        meaning = entry.meaning;
    }
    else { return false; }

    // If a verb is short it should have only limited combinability (LM).
    // Such verbs can only combine with prefixes, suffixes, and participle
    // endings.
    if pos == POS::Verb || pos == POS::SubstantiveVerb {
        if index > 0 {
            if let Some(prev) = &morpheme_list.get(index - 1) {
                if prev.synthesis != Synthesis::Prefix { return false; }
            }
        }
        if index < last {
            if let Some(next) = &morpheme_list.get(index + 1) {
                if next.synthesis != Synthesis::Suffix &&
                   next.synthesis != Synthesis::Participle { return false; }
            }
        }
    }
    else if meaning == Meaning::Parenco {
        if index > 0 {
            if let Some(prev) = &morpheme_list.get(index - 1) {
                let m = &prev.word[..];
                match m {
                    "bo" => (),
                    "ge" => (),
                    "pra" => (),
                    _ => return false,
                };
            }
        }
        if index < last {
            if let Some(next) = &morpheme_list.get(index + 1) {
                let m = &next.word[..];
                if m != "in" { return false; }
            }
        }
    }
    else if is_animal(meaning) {
        if index > 0 {
            if let Some(prev) = &morpheme_list.get(index - 1) {
                let m = &prev.word[..];
                if m != "vir" { return false; }
            }
        }
        if index < last {
            if let Some(next) = &morpheme_list.get(index + 1) {
                let m = &next.word[..];
                match m {
                    "in" => (),
                    "id" => (),
                    "aĵ" => (),
                    "ov" => (),
                    _ => return false,
                };
            }
        }
    }
    else if meaning == Meaning::Etno {
        if index > 0 {
            if let Some(prev) = &morpheme_list.get(index - 1) {
                if prev.word != "ge" { return false; }
            }
        }
        if index < last {
            if let Some(next) = &morpheme_list.get(index + 1) {
                let m = &next.word[..];
                match m {
                    "in" => (),
                    "id" => (),
                    "land" => (),
                    "stil" => (),
                     _ => return false,
                };
            }
        }
    }
    return true;
}  // check_limited_synthesis


/*
 * valid_separator - Checks synthesis of a separators. (nask-O-tag-o, du-A-foj-e)
 *
 * Params:
 *    part of speech of separator (O = substantive, A = adjective, I = verb)
 *    index of morpheme in morpheme list
 *    morpheme list (vector of dictionary entries)
 * Return:
 *    true for valid synthesis, false otherwise
 */
fn valid_separator(pos: POS, index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return false; }

    // Get part of speech of previous morpheme.
    let previous_pos;
    if let Some(previous) = &morpheme_list.get(index - 1) {
        previous_pos = previous.part_of_speech;
    }
    else { return false; }

    // type of ending, eg;  nask-o-tag-O  <-- Substantive, du-a-foj-E  <-- Adverb.
    let type_of_ending = morpheme_list.type_of_ending();

    // Reconsider this:
    // ''blu.a.ĉiel.o" is an error. An adjective is not joined directly to a substantive morpheme.
    if pos == POS::Substantive && previous_pos > POS::Adjective { return false; }
    // "blu.a.blank.o" is an error. Adjective joined to substantive is invalid.
    else if pos == POS::Adjective || pos == POS::Adverb {
        if type_of_ending != POS::Adjective && type_of_ending != POS::Adverb { return false; }
        if previous_pos > POS::Adverb { return false; }
    }
    return true;

}  // valid_separator


/*
 * check_participle()
 *
 * Check participle suffixes, Eg. 'naĝ-ant-a' (swimming), 'forges-it-a' (forgotten).
 * Note: Passive participle endings can only be attached to transitive verbs.
 * Also, the participle ending is not necessarily the last morpheme in a word.
 * Words such as 'forges.it.aĵ.o' are sometimes found.
 * 
 * For a description of parameters see check_acx().
 */
fn check_participle(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return false; }

    let participle_string;	// Eg: "ant", "at", "int", etc.
    let previous_string;	// The previous morpheme (string).
    let previous_pos;	// Part of speech of previous morpheme.
    let previous_trans;
    let mut next_string = "";	// The next morpheme (string), if there is one.
    let last = morpheme_list.last_index();

    if let Some(entry) = morpheme_list.get(index) { participle_string = &entry.word; }
    else { return false; }

    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        previous_string = &previous_entry.word[..];
        previous_pos = previous_entry.part_of_speech;
        previous_trans = previous_entry.transitivity;
    }
    else { return false; }

    if index < last {
        if let Some(next_entry) = morpheme_list.get(index + 1) {
            next_string = &next_entry.word;
        }
        else { return false; }
    }

    if previous_pos == POS::Verb || previous_pos == POS::SubstantiveVerb {
        if participle_string.len() == 2 {  // -it, -at, and -ot are passive participle endings.
           if previous_trans != Transitivity::Transitive { return false; }
        }
        if index < last {
            match next_string {
                "aĵ" => return true,
                "ul" => return true,
                "in" => return true,
                "ec" => return true,
                "ar" => return true,
                _ => return false,
            }
        }
        return true;
    }

    match previous_string {
        "antaŭ" => return true,
        "anstataŭ" => return true,
        "ĉirkaŭ" => return true,
        "kontraŭ" => return true,
        "super" => return true,
         _ => return false,
    };

}  // check_participle


/*
 * scan_morphemes - Scan the list of morphemes to check the validity of prefixes, etc.
 * This is done after the word is completely divided into morphemes.
 *
 * Params:
 *    morpheme list (vector of dictionary entries)
 * Return:
 *    true for valid synthesis, false otherwise
 */
pub fn scan_morphemes(morpheme_list: &mut Morphemes) -> bool {

    let last = morpheme_list.last_index();
    if morpheme_list.count_separators() > 1 { return false; }  // Only allow one.

    for index in 0..=last {

        let syn;
        let pos;
        let morpheme;

        if let Some(entry) = &morpheme_list.get(index) {

            syn = entry.synthesis;
            pos = entry.part_of_speech;
            morpheme = entry.word.clone();

            // Separator between morphemes (fingr-o-montri)
            if entry.flag == Flag::Separator {
                if !valid_separator(pos, index, morpheme_list) { return false; };
            }
        }
        else { return false; }  // Failure to get morpheme from vector.

        if syn == Synthesis::Prefix {
            if index == last { return false; }   // A prefix can't be the last morpheme.
            if !check_prefix(&morpheme, index, morpheme_list) { return false; }
        }
        else if syn == Synthesis::Participle {
            if !check_participle(index, morpheme_list) { return false; }
        }
        else if syn == Synthesis::Limited {
            if !check_limited_synthesis(&morpheme, index, morpheme_list) { return false; };
        }
    }
    return true;  // All OK.

} // scan_morphemes
