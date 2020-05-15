// This library has macros and functions for dealing with Esperanto's accented characters.
// Klivo (indriko@yahoo.com) 2020-05-08

#[macro_use]
mod macros;

/*
 * x_to_accent()
 *
 * Converts cx to ĉ, sx to ŝ, etc., for an entire string.
 *
 * Params:
 *    original word (&str)
 * Return:
 *    new word (String)
 */
pub fn x_to_accent(word: &str) -> String {

    let word = word.chars().collect::<Vec<_>>();
    let length = word.len();
    let mut _new_word = String::from("");

    let mut skip_x = false;   // For skipping over x.
    for i in 0..length {
        if skip_x { skip_x = false; continue; }
        let ch1 = word[i];
        if accepts_hat!(ch1) {
            if i < (length - 1) {
                let ch2 = word[i + 1];
                if is_x!(ch2) {
                    _new_word.push(accent_letter!(ch1));
                    skip_x = true;
                }
                else {
                    _new_word.push(ch1);
                }
            }
            else {
                _new_word.push(ch1);
            }
        }
        else {
            _new_word.push(ch1);
        }
    }  // for

    return _new_word;
}  // x_to_accent


// remove_hyphens - Remove hyphens from string.
pub fn remove_hyphens(word: &str) -> String {
    return word.replace("-", "").replace("­", "");
}

// Capitalize the first letter of a word: kanado -> Kanado.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

// Convert an Esperanto word from x-format to Unicode, then print the result.
// Eg. 'cxirkaux' prints as 'ĉirkaŭ'. 
pub fn print_eo(s: &str) { print!("{}", x_to_accent(s)); }
pub fn println_eo(s: &str) { println!("{}", x_to_accent(s)); }


// restore_capitals
//
// The Esperanto dictionary (vortaro) has only lower case morphemes, so words
// are converted to lower case for dictionary lookups. It might be useful to
// convert words back to their original case after analysis. For example, an
// analysis of the word  'RIĈULO' will produce 'riĉ.ul.o'. This function will take
// 'RIĈULO' and 'riĉ.ul.o' to produce 'RIĈ.UL.O'.
// Params:
//      original word
//      result of analysis
// Return:
//      analyzed result with original case restored
//
pub fn restore_capitals(original: &str, analyzed: &str) -> String {
    let original_chars: Vec<_> = original.chars().collect();
    let analyzed_chars = analyzed.chars();
    let mut result = String::from("");
    let mut index = 0;  // index into original
    for ch in analyzed_chars {
        if ch == '.' {
            result.push(ch);
        }
        else {
            result.push(original_chars[index]);
            index += 1;
        }
    }
    return result;
}

