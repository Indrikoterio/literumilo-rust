// vortaro - This module creates an Esperanto dictionary (vortaro) from a data file.
// Klivo 2020-05-08

use std::collections::HashMap;

use literumilo::*;
use super::entry::*;

/*
 * make_dictionary
 *
 * This function takes rows of tab-separated dictionary data and produces a hash map
 * which is indexed by morpheme.
 *
 * A typical row of data is:
 * divid	VERBO	N	T	N	KF	NLM	1	R
 *
 * The columns are:
 * morpheme, part of speech, meaning, transitivity, without-ending, with-ending, combinability, rarity, flag.
 *
 * morpheme - eg. 'divid', 'elefant', 'amik'
 * part of speech - SUBST (substantive), VERBO, ADJEKTIVO, etc.
 * meaning - eg. ANIMALO, URBO, PERSONO
 * transitivity - N/T
 * without-ending - SF = Sen Finaĵo (without ending), N = Ne (no)
 * with-ending - KF = Kun Finaĵo (with ending), N = Ne (no)
 * combinability - LM (limited), NLM (not limited), P (as prefix), S (as suffix)
 * rarity - 0 = very common, 4 = rare
 * flag - R (root/ morpheme), K (compound), X (eXclude from dictionary)
 *
 * Params:
 *    strings of dictionary data
 * Return:
 *    hash map of dictionary data
 */
pub fn make_dictionary(data: &str) -> HashMap<String, Entry> {

    let mut esperanto_dictionary: HashMap<String, Entry> = HashMap::new();

    // Read the dictionary data into _dict_map.
    for line in data.lines() {

        if line.len() == 0 { continue; }

        let ch = line.chars().next().unwrap();
        if ch == '#' { continue; }    // Must be a comment line. Skip it.

        let split_line: Vec<&str> = line.split_whitespace().collect();
        // If there are not enough fields, it must be bad data, so skip it.
        if split_line.len() < 9 {
            println!(">>>>>>> {}", line);
            continue;
        }

        // Make a key
        let _vorto = String::from(split_line[0]);
        let _vorto2 = x_to_accent(&_vorto);
        let _key = _vorto2.replace(".", "").to_lowercase();

        if let Some(entry) = Entry::new(split_line) {
            esperanto_dictionary.insert(_key.clone(), entry);
        }
    }
    return esperanto_dictionary;

}  // make_dictionary
