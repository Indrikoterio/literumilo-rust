// Define a Morpheme list. This list is used to analyse the synthesis of compound words.
//
// Why is morpheme_list a vector of dictionary entries, rather than a vector
// of references to dictionary entries? The algorithm uses this list to analyze
// word synthesis. Sometimes, it needs to modify an entry in the morpheme list.
//
// Klivo 2020-05-08

use super::entry::*;
use super::ending::*;

const  MAX_MORPHEMES: usize = 9; // The maximum number of morphemes in a compound word.

#[derive(Debug)]
pub struct Morphemes<'a> {
    last_index: usize,   // last index written to
    morpheme_list: Vec<Entry>,
    ending: Ending<'a>,
}

impl Morphemes<'_> {

    /*
     * new - Factory for a new morpheme list.
     *
     * Params:
     *    ending (grammatical ending of word)
     * Returns:
     *    morpheme list (Morphemes)
     */
    pub fn new(ending: Ending) -> Morphemes {

        let entry = Entry::empty();
        let mut morpheme_list: Vec<Entry>  = Vec::with_capacity(MAX_MORPHEMES);

        for _ in 0..MAX_MORPHEMES { morpheme_list.push(entry.clone()); }

        Morphemes {
            last_index: 0,
            morpheme_list: morpheme_list,
            ending: ending,
        }
    }  // end of new()


    /*
     * get - get one entry (borrow)
     *
     * Params:
     *     index of entry
     * Return:
     *     optional entry (morpheme)
     */
    pub fn get(&self, index: usize) -> Option<&Entry> {
        self.morpheme_list.get(index)
    }

    /*
     * get_mut - get one entry (mutable borrow)
     *
     * Params:
     *     index of entry
     * Return:
     *     optional entry (morpheme)
     */
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Entry> {
        self.morpheme_list.get_mut(index)
    }

    /*
     * put - put one entry
     *
     * Make a clone of the given entry, because we may
     * need to modify it later.
     *
     * Params:
     *     index of entry
     *     entry to insert into list
     */
    pub fn put(&mut self, index: usize, entry: &Entry) {
        self.last_index = index;
        self.morpheme_list[index] = entry.clone();
    }

    // Getter for last_index.
    pub fn last_index(&self) -> usize { self.last_index }

    // Getter for type (part of speech) of ending. Eg. Substantive, Verb...
    pub fn type_of_ending(&self) -> POS { self.ending.pos }

    /*
     * print - Prints collected entries from the morpheme list.
     *
     * Parameters:
     *     borrowed self
     */
    pub fn _print(&self) {
        for i in 0..=self.last_index {
            println!("-- {:?}", self.morpheme_list.get(i));
        }
    }

    /*
     * display_form
     *
     * This method takes the collected morphemes in morpheme_list
     * and returns a single string for display, with each morpheme
     * separated by a period, eg. 'for.ig.it.a'.
     *
     * Parameter:
     *     borrowed self
     * Return:
     *     string of morphemes
     */
    pub fn display_form(&self) -> String {
        let mut s = String::from("");
        if let Some(m) = self.morpheme_list.get(0) { s = format!("{}", m.word); }
        for i in 1..=self.last_index {
            if let Some(m) = self.morpheme_list.get(i) {
                s = format!("{}.{}", s, m.word);
            }
        }
        let s2 = format!("{}.{}", s, self.ending.ending);
        return s2;
    }

    /*
     * count_separators
     *
     * This method scans the collected morphemes in morpheme_list
     * to determine how many separators vowels there are. For example,
     * 'last.A.temp.e' has 1 separator vowel (A). (last.temp.e is a little
     * harder to pronounce.) This program will only allow one per word.
     *
     * Parameter:
     *     borrowed self
     * Return:
     *     count of separators
     */
    pub fn count_separators(&self) -> u32 {
        let mut count = 0;
        for index in 0 ..= self.last_index {
            if self.morpheme_list[index].flag == Flag::Separator {
                count += 1;
            }
        }
        return count;
    }

}  // end of impl Morphemes
