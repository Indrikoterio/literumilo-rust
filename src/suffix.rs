// suffix.rs - contains functions to check the synthesis of suffixes.
// A compound word such as 'frenez-ul-ej-(o)' consists of 3 morphemes,
// (excluding the grammatical ending), which are stored in a vector called
// morpheme_list. Whether a suffix (-ul, -ej) is valid or not depends on
// the morphemes which come before it.
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
 * check_acx()
 *
 * Check whether suffix -aĉ is valid. Aĉ means bad (quality), unpleasant, ugly.
 * If this is the first morpheme in the word, treat it as an adjective. (aĉ-ulo)
 * Otherwise, it can follow a substantive, verb, adjective or participle.
 * (hund-aĉ-o, kri-aĉ-is, laŭt-aĉ-a)
 *
 * Params:
 *     index of morpheme
 *     morpheme list (vector of dictionary entries)
 * Return:
 *     true if synthesis is valid, false otherwise
 */
fn check_acx(index: usize, morpheme_list: &mut Morphemes) -> bool {

    let pos: POS;   // Part of Speech
    let meaning: Meaning;
    let transitivity: Transitivity;

    // If aĉ is the first element, it's OK. Treat it as an adjective.
    if index == 0 {
        if let Some(current_entry) = morpheme_list.get_mut(index) {
            current_entry.part_of_speech = POS::Adjective;
            return true;
        }
        else { return false; }
    }

    // Aĉ is not first.
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        pos = previous_entry.part_of_speech;
        meaning = previous_entry.meaning;
        transitivity = previous_entry.transitivity;
    }
    else { return false; }

    // Aĉ does not change the character of a word.
    // If 'kri-as' is intransitive, then 'kri-aĉ-as' is also intransitive.
    // It is necessary to transfer the POS (Verb, Substantive),
    // intransitivity (etc.) from the previous entry to the aĉ entry.
    if let Some(current_entry) = morpheme_list.get_mut(index) {
        if  pos <= POS::Adjective || pos == POS::Participle {
            current_entry.part_of_speech = pos;
            current_entry.meaning = meaning;
            current_entry.transitivity = transitivity;
            return true;
        }
    }

    return false;

}  // check_acx

/*
 * check_ad()
 *
 * Check suffix -ad.
 * Ad, when attached to a substantive (noun), indicates the associated action. Eg. 'martel-ad-o'
 * means 'hammering'. When attached to a verb, it indicates the abstract idea of a verb, or
 * repetition or long duration. Eg. 'frap-ad-o' means 'hitting', or 'repeated hitting'.
 * This suffix is attached to substantives, verbs or substantive-verbs. (kur-ad-is, funebr-ad-as)
 * Its part of speech changes to Verb, and it takes the transitivity of the morpheme it follows.
 * For a description of parameters see check_acx().
 */
fn check_ad(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return false; }

    let pos: POS;   // Part of Speech
    let transitivity: Transitivity;

    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        pos = previous_entry.part_of_speech;
        transitivity = previous_entry.transitivity;
    }
    else { return false; }

    if let Some(current_entry) = morpheme_list.get_mut(index) {
        if  pos <= POS::Verb {
            current_entry.part_of_speech = POS::Verb;
            current_entry.transitivity = transitivity;
            return true;
        }
    }
    return false;
}  // check_ad


/*
 * check_ajx()
 *
 * Check suffix -aĵ, meaning 'thing'. This suffix is attached to adjectives, prepositions
 * and participles. (blank-aĵ-o, krom-aĵ-o, perd-it-aĵ-o)
 * For a description of parameters see check_acx().
 */
fn check_ajx(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return true; }

    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        if pos <= POS::Adjective { return true; }
        match pos {
            POS::Preposition => return true,
            POS::Participle => return true,
            _ => (),
        }
        // Note: The following two lines seem to be irrelevant.
        let meaning = previous_entry.meaning;
        if is_animal(meaning) { return true; }   // bovaĵo, fiŝaĵo
    }
    return false;

}  // check_ajx


/*
 * check_an()
 *
 * Check suffix -an, meaning 'member of a group'. This suffix is attached to
 * substantives, which do not mean 'person'.
 * For a description of parameters see check_acx().
 */
fn check_an(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return true; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        let meaning = previous_entry.meaning;
        if  pos <= POS::SubstantiveVerb {
            if is_person(meaning) { return false; }
            else { return true;}
        }
    }
    return false;
}  // check_an


/*
 * check_ar()
 *
 * Check suffix -ar (meaning 'a group of something').
 * This suffix is attached to substantives and participles.
 * (hom-ar-o, aŭskult-ant-ar-o)
 *
 * For a description of parameters see check_acx().
 */
fn check_ar(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return true; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        if  pos <= POS::SubstantiveVerb || pos == POS::Participle {
            return true;
        }
    }
    return false;
}  // check_ar


/*
 * check_ebl()
 *
 * Check suffix -ebl, meaning 'capable of being verb-ed'.
 * This suffix is generally attached to transitive verbs.
 *
 * For a description of parameters see check_acx().
 */
fn check_ebl(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return true; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        if  pos == POS::Verb || pos == POS::SubstantiveVerb {
            if previous_entry.transitivity == Transitivity::Transitive { return true; }
        }
    }
    return false;
}  // check_ebl


/*
 * check_ec()
 *
 * Check suffix -ec, which expresses quality or state. (alt-ec-o)
 *
 * For a description of parameters see check_acx().
 */
fn check_ec(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return false; }
    let pos: POS;   // Part of Speech

    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        pos = previous_entry.part_of_speech;
    }
    else { return false; }

    if let Some(current_entry) = morpheme_list.get_mut(index) {
        if  pos <= POS::SubstantiveVerb ||
             pos == POS::Adjective ||
             pos == POS::Number ||
             pos == POS::Participle {
            current_entry.part_of_speech = POS::Substantive;
            return true;
        }
    }
    return false;

}  // check_ec


/*
 * check_eg_et()
 *
 * Check suffixes -eg and -et, which augment or diminish a word.
 * (laŭt-eg-a, ruĝ-et-a, kri-eg-is, hund-et-o)
 *
 * These two suffixes do not change the part of speech, meaning,
 * nor transitivity of the previous morpheme, so these are transferred.
 *
 * For a description of parameters see check_acx().
 */
fn check_eg_et(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return false; }
    let pos: POS;   // Part of Speech
    let meaning: Meaning;
    let transitivity: Transitivity;

    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        pos = previous_entry.part_of_speech;
        meaning = previous_entry.meaning;
        transitivity = previous_entry.transitivity;
    }
    else { return false; }

    if let Some(current_entry) = morpheme_list.get_mut(index) {
        if  pos <= POS::Adjective {
            current_entry.part_of_speech = pos;
            current_entry.meaning = meaning;
            current_entry.transitivity = transitivity;
            return true;
        }
    }
    return false;

}  // check_eg_et


/*
 * check_ej()
 *
 * Check suffix -ej, (meaning 'place'). Eg. manĝ-ej-o.
 *
 * This suffix should not be attached to a morpheme which already
 * has a meaning of 'place' (Loko).
 *
 * For a description of parameters see check_acx().
 */
fn check_ej(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return false; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        let meaning = previous_entry.meaning;
        if  pos <= POS::Adjective {
            if meaning == Meaning::Loko { return false; }
            return true;
        }
    }
    return false;

}  // check_ej


/*
 * check_em()
 *
 * Check suffix -em, (meaning 'tendency'). Eg. dorm-em-a
 *
 * For a description of parameters see check_acx().
 */
fn check_em(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return false; }

    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        if  pos <= POS::Adjective { return true; }
    }
    return false;

}  // check_em


/*
 * check_end_ind()
 *
 * Check suffixes -ind and -end.
 * -ind means, worthy to be (verb)-ed. Eg. vid-ind-a , worthy to be seen.
 * -end means, required to be (verb)-ed. Eg. pag-end-a, necessary to be paid.
 * These suffixes are normally only applied to transitive verbs. Mir-ind-a is
 * an exception.
 *
 * For a description of parameters see check_acx().
 */
fn check_end_ind(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return false; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        if  previous_entry.transitivity == Transitivity::Transitive { return true; }
    }
    return false;
}  // check_end_ind

/*
 * check_er()
 *
 * Check suffix -er
 * -er indicates part of a whole, eg. mon-er-o, a coin (piece of money).
 *
 * For a description of parameters see check_acx().
 */
fn check_er(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return false; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        let previous_string = &previous_entry.word[..];
        if previous_string == "sup" { return false; }   // 'sup.er' is a mistake for 'super'
        if  pos <= POS::SubstantiveVerb { return true; }
    }
    return false;
}  // check_er

/*
 * check_ik_ing_ism()
 *
 * Check suffixes -ik, -ing, -ism.
 * -ik indicates a science, technique, or art, eg. komput-ik-o (computer science).
 * -ing indicates a holder, eg. kandel-ing-o (candle holder).
 * -ism indicates a doctrine or customary behaviour, eg. alkohol-ism-o (alcoholism).
 * 
 * These suffixes are normally attached to substantives.
 *
 * For a description of parameters see check_acx().
 */
fn check_ik_ing_ism(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return false; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        if  pos <= POS::SubstantiveVerb { return true; }
    }
    return false;
}  // check_ik_ing_ism


/*
 * check_estr()
 *
 * Check suffix -estr, which indicates a leader, eg. urb-estr-o, (a mayor).
 * 
 * This suffix is attached to substantives. The new word becomes
 * a substantive, with a meaning of 'person'.
 *
 * For a description of parameters see check_acx().
 */
fn check_estr(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 {
        if let Some(current_entry) = morpheme_list.get_mut(index) {
            current_entry.part_of_speech = POS::Substantive;
            current_entry.meaning = Meaning::Persono;
            return true;
        }
        return false;
    }

    let pos: POS;
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        pos = previous_entry.part_of_speech;
    }
    else { return false; }

    if let Some(current_entry) = morpheme_list.get_mut(index) {
        if  pos <= POS::SubstantiveVerb {
            current_entry.part_of_speech = POS::Substantive;
            current_entry.meaning = Meaning::Persono;
            return true;
        }
    }

    return false;

}  // check_estr


/*
 * check_id()
 *
 * Check suffix -id, which means 'offspring of'.
 * This suffix is attached to animals, eg. kat-id-o (kitten).
 *
 * For a description of parameters see check_acx().
 */
fn check_id(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return true; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let meaning = previous_entry.meaning;
        if meaning == Meaning::Etno { return true; }
        if is_animal(meaning) { return true; }
    }
    return false;

}  // check_id


/*
 * check_ig_igx()
 *
 * Check suffixes -ig and -iĝ.
 * -ig is a causative suffix, eg. 'star-ig-is' (make to stand).
 * -iĝ indicates a change of state, eg. 'griz-iĝ-is' (became grey).
 * 
 * For a description of parameters see check_acx().
 */
fn check_ig_igx(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return false; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        if  pos <= POS::Adverb ||
            pos == POS::Preposition ||
            pos == POS::Prefix { return true; }
    }
    return false;
}  // check_ig_igx


/*
 * check_il()
 *
 * Check suffix -il, meaning 'tool'. Eg. ŝraŭb-il-o (screwdriver)
 * 
 * For a description of parameters see check_acx().
 */
fn check_il(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return true; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        let meaning = previous_entry.meaning;
        if  (pos == POS::Verb || pos == POS::SubstantiveVerb) &&
            meaning != Meaning::Ilo { return true; }
    }
    return false;
}  // check_il


/*
 * check_in()
 *
 * Check suffix -in, meaning 'female'. Eg. patr-in-o (mother).
 * 
 * For a description of parameters see check_acx().
 */
fn check_in(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return true; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let meaning = previous_entry.meaning;
        if  is_person(meaning) || is_animal(meaning) { return true; }
    }
    return false;
}  // check_in


/*
 * check_ist()
 *
 * Check suffix -ist, meaning 'a professional, supporter of a idea, doctrine, etc.'.
 * Eg. Esperant-ist-o (Esperantisto)
 * 
 * For a description of parameters see check_acx().
 */
fn check_ist(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return false; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        let meaning = previous_entry.meaning;
        if  pos <= POS::Verb && !is_person(meaning) { return true; }
    }
    return false;
}  // check_ist


/*
 * check_obl_on_op()
 *
 * Check suffixes -obl, -on, -op. These are attached to numbers.
 * Eg. du-obl-e (double), du-on-o (a half), du-op-o (a pair).
 * 
 * For a description of parameters see check_acx().
 */
fn check_obl_on_op(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return false; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        if  previous_entry.part_of_speech == POS::Number { return true; }
    }
    return false;
}  // check_obl_on_op


/*
 * check_uj()
 *
 * Check suffix -uj, meaning 'a container, a fruit bearing tree, a country'.
 * Eg. cindr-uj-o (ashtray), pom-uj-o (apple tree), Angl-uj-o (England).
 * 
 * For a description of parameters see check_acx().
 */
fn check_uj(index: usize, morpheme_list: &mut Morphemes) -> bool {
    if index == 0 { return false; }
    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        let pos = previous_entry.part_of_speech;
        let meaning = previous_entry.meaning;
        if  pos <= POS::SubstantiveVerb && meaning != Meaning::Arbo { return true; }
    }
    return false;
}  // check_uj


/*
 * check_ul()
 *
 * Check suffix -ul, meaning 'a person'. Eg. povr-ul-o (poor person).
 * Note: This suffix will be allowed to attach to a participle ending,
 * eg., 'frap.it.ul.o', although this may be redundant.
 * 
 * For a description of parameters see check_acx().
 */
fn check_ul(index: usize, morpheme_list: &mut Morphemes) -> bool {

    if index == 0 { return true; }
    let pos;
    let meaning;

    if let Some(previous_entry) = morpheme_list.get(index - 1) {
        pos = previous_entry.part_of_speech;
        meaning = previous_entry.meaning;
    }
    else { return false; }
    if  pos == POS::Participle { return true;}
    if  pos <= POS::Adjective && !is_person(meaning) { return true; }
    if  pos == POS::Preposition { return true;}
    return false;
}  // check_ul


/*
 * check_suffix - Checks synthesis of suffixes.
 *
 * Params:
 *    suffix as string
 *    index of morpheme in morpheme list
 *    morpheme list (vector of dictionary entries)
 * Return:
 *    true for valid synthesis, false otherwise
 */
pub fn check_suffix(_suffix: &str, index: usize, morpheme_list: &mut Morphemes) -> bool {

    match _suffix {
        "aĉ" => return check_acx(index, morpheme_list),
        "ad" => return check_ad(index, morpheme_list),
        "aĵ" => return check_ajx(index, morpheme_list),
        "an" => return check_an(index, morpheme_list),
        "ar" => return check_ar(index, morpheme_list),
        "ebl" => return check_ebl(index, morpheme_list),
        "ec" => return check_ec(index, morpheme_list),
        "eg" => return check_eg_et(index, morpheme_list),
        "et" => return check_eg_et(index, morpheme_list),
        "ej" => return check_ej(index, morpheme_list),
        "em" => return check_em(index, morpheme_list),
        "end" => return check_end_ind(index, morpheme_list),
        "ind" => return check_end_ind(index, morpheme_list),
        "er" => return check_er(index, morpheme_list),
        "ik" => return check_ik_ing_ism(index, morpheme_list),
        "ing" => return check_ik_ing_ism(index, morpheme_list),
        "ism" => return check_ik_ing_ism(index, morpheme_list),
        "estr" => return check_estr(index, morpheme_list),
        "id" => return check_id(index, morpheme_list),
        "ig" => return check_ig_igx(index, morpheme_list),
        "iĝ" => return check_ig_igx(index, morpheme_list),
        "il" => return check_il(index, morpheme_list),
        "in" => return check_in(index, morpheme_list),
        "ist" => return check_ist(index, morpheme_list),
        "obl" => return check_obl_on_op(index, morpheme_list),
        "on" => return check_obl_on_op(index, morpheme_list),
        "op" => return check_obl_on_op(index, morpheme_list),
        "uj" => return check_uj(index, morpheme_list),
        "ul" => return check_ul(index, morpheme_list),
        _ => return false,
    }
}  // check_suffix
