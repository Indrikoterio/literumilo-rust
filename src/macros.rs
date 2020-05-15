// Macros for testing Esperanto letters.
// Klivo 2020-05-08

// accepts_hat!
// Test whether or not an Esperanto letter can accept a 'hat'.
#[allow(unused_macros)]
macro_rules! accepts_hat {
    ($ch:expr) => (($ch) == 'c' || ($ch) == 'g' || ($ch) == 's' ||
                            ($ch) == 'u' || ($ch) == 'h' || ($ch) == 'j' ||
                            ($ch) == 'C' || ($ch) == 'G' || ($ch) == 'S' ||
                            ($ch) == 'U' || ($ch) == 'H' || ($ch) == 'J') 
}

// is_x!
// Test whether character is x or X.
#[allow(unused_macros)]
macro_rules! is_x {
    ($ch:expr) => (($ch) == 'x' || ($ch) == 'X') 
}

// accent_letter
// Put a 'hat' on the given letter.
#[allow(unused_macros)]
macro_rules! accent_letter {
    ($ch:expr) =>  (match $ch {
        'c' => 'ĉ',
        'g' => 'ĝ',
        's' => 'ŝ',
        'u' => 'ŭ',
        'j' => 'ĵ',
        'h' => 'ĥ',
        'C' => 'Ĉ',
        'G' => 'Ĝ',
        'S' => 'Ŝ',
        'U' => 'Ŭ',
        'J' => 'Ĵ',
        'H' => 'Ĥ',
         _  => '?',
    })
}

// is_word_char
// This macro returns 'true' for word characters such as 'abc',
// and 'false' for others, such as punctuation and white space.
#[allow(unused_macros)]
macro_rules! is_word_char {
    ($ch:expr) => (
        ($ch) >= 'a' && ($ch) <= 'z' ||
        ($ch) >= 'A' && ($ch) <= 'Z' ||
        ($ch) >= 'À' && ($ch) <= 'ʯ' ||
        ($ch) == '-' ||
        ($ch) == '­'
    )
}

// is_hyphen
// This macro returns 'true' for hyphens (0x002D and 0x00AD), and 'false' otherwise.
#[allow(unused_macros)]
macro_rules! is_hyphen {
    ($ch:expr) => (($ch) == '-' || ($ch) == '­') 
}

