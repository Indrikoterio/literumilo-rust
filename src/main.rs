// literumilo - This program is a spell checker for Esperanto.
// Besides checking spelling, it can separate words into their component morphemes.
// For example. 'forigitaj' will be divided as 'for.ig.it.aj'.
// Klivo 2020-05-08

#[macro_use]
mod macros;

use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

use vortaro::*;  // = dictionary
use entry::*;  // dictionary entry
use check_word::*;

mod suffix;
mod entry;
mod ending;
mod vortaro;
mod check_word;
mod morpheme_list;
mod scan_morphemes;

const HOW_TO_USE: &str = "\nLiterumilo   Rust version: 1.0\n\n\
    ----- (Esperanto sekvas.)\n\
    This program is a spell checker and morphological analyzer for Esperanto.\n\n\
    To list misspelled words from a file: ./literumilo file.txt\n\
    To divide words from a file into morphemes: ./literumilo -m file.txt\n\
    To check the spelling of a single word: ./literumilo ĉiutage\n\
    Accents can be represented by 'x': ./literumilo cxiutage\n\n\
    -----\n\
    Ĉi tiu programo estas literumilo kaj analizilo de morfemoj por Esperanto.\n\n\
    Por listigi misliterumitajn vortojn de dosiero: ./literumilo file.txt\n\
    Por dividi vortojn de dosiero laŭ morfemoj: ./literumilo -m file.txt\n\
    Por kontroli la literumadon de unu vorto: ./literumilo ĉiutage\n\
    Oni povas anstataŭigi supersignon per 'x': ./literumilo cxiutage\n\n\
    Klivo <indriko@yahoo.com> 2020";

/*
 * analyze_file()
 *
 * Analyze the text within a file. If the mode is False, check the spelling
 * of each word in the file, and return a list of unknown words.
 * If the mode is True, return the analyzed text with each known word
 * divided into morphemes (separated by periods).
 *
 * Params:
 *     file name
 *     morpheme mode - True = morphological analyzer, False = spell checker
 *     dictionary
 * Return:
 *     analyzed text, or list of misspelled words  (str)
 */
fn analyze_file(filename: &str, morpheme_mode: bool, dictionary: HashMap<String, Entry>) {

    let path = std::path::Path::new(filename);
    let exists = path.exists();
    if !exists {
        println!("File does not exist: {}", filename);
    }
    else {

        let path_display = path.display();
        // Open file for reading.
        let mut file = match File::open(&path) {
            Err(reason) => panic!("Could not open {}: {}", path_display, reason),
            Ok(file) => file,
        };

        // Read the file into a string.
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(reason) => panic!("Could not read {}: {}", path_display, reason),
            Ok(_) => (),
        }

        let mut bad_words = HashSet::new();

        let char_iter = s.chars();
        let mut in_word = false;
        let mut collected_chars = format!("");

        for ch in char_iter {
            if is_word_char!(ch) {
                in_word = true;
                collected_chars.push(ch);
            }
            else {
                if in_word {
                    let result = check_word(&collected_chars, &dictionary);
                    if morpheme_mode {
                        print!("{}", result.word);
                    }
                    else {
                        if !result.valid { bad_words.insert(format!("{}", &collected_chars)); }
                    }
                    collected_chars.clear();
                }
                in_word = false;
                if morpheme_mode { print!("{}", ch); }
            }
        }

        if in_word {
            let result = check_word(&collected_chars, &dictionary);
            if morpheme_mode {
                print!("{}", result.word);
            }
            else {
                if !result.valid { bad_words.insert(format!("{}", &collected_chars)); }
            }
            collected_chars.clear();
        }

        if !morpheme_mode {
            for word in bad_words { println!("{}", word); }
        }
    }
}  // analyze_file()


/*
 * main()
 *
 * The main function determines whether or not the given command line
 * parameter is a file name. If it is a file name, main calls analyze_file().
 *
 * If no file with the given name exists, the function calls check_word(),
 * assuming that the given parameter is an individual word.
 *
 * If the command line parameter is a file name, main() checks for
 * the 'morpheme_mode' flag (-m), and passes this parameter to
 * analyze_file(). When the morpheme_mode flag is set, analyze_file()
 * will output the entire text with Esperanto words divided by morpheme.
 * Otherwise it will output a list of misspelled words.
 */
fn main() {

    let args: Vec<String> = env::args().collect();
    let num_args = args.len() - 1;
    if num_args == 0 {
        println!("{}", HOW_TO_USE);
        process::exit(0);
    }

    // Get arguments.
    // 'Morpheme' mode means that the program will divide words in the input file
    // by morphemes. Eg. 'submara' becomes 'sub.mar.a'. Otherwise, the program
    // will output a list of presumably misspelled words.
    let mut morpheme_mode = false;
    let mut first_arg = "";
    let mut second_arg = "";

    if num_args > 0 { first_arg = &args[1]; }
    if num_args > 1 { second_arg = &args[2]; }

    let mut file_or_word = first_arg;
    if num_args >= 2 {
        if first_arg == "-m" {
            morpheme_mode = true;
            file_or_word = second_arg;
        }
    }

    // Get the Esperanto dictionary.
    let dictionary_source = include_str!("vortaro.tsv");
    let vortaro: HashMap<String, Entry> = make_dictionary(dictionary_source);

    let path = std::path::Path::new(file_or_word);
    let exists = path.exists();

    if exists {   // If there is a file...
        analyze_file(file_or_word, morpheme_mode, vortaro)
    }
    else {  // Must be a word.
        let word = literumilo::x_to_accent(file_or_word);
        let result = check_word(&word, &vortaro);
        if result.valid {
            println!("{} ✓", result.word);
        }
        else {
            println!("✘{}", file_or_word);
        }
    }
}

// Memory made safe,
// Generations turn to Rust,
// Silicon will sing.
//
// Klivo
