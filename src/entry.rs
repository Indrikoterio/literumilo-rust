// Make an entry for the Esperanto spell-checking dictionary.
// Klivo 2020-05-08

use literumilo::*;

// Part of Speech - also defines role in morphology.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum POS {
    Substantive,	// = noun
    SubstantiveVerb,
    Verb,
    Adjective,
    Number,
    Adverb,
    Pronoun,
    PronounAdjective,
    Preposition,
    Conjunction,
    Subjunction,
    Interjection,
    Prefix,
    TechPrefix,	// technical prefix,(hiper-, mega-) - not used independently
    Suffix,
    Article,
    Participle,
    Abbreviation,	// UEA, UNESKO
    Letter,
}

impl POS {
    pub fn new(s: &str) -> POS {
        match s {
            "SUBST" => POS::Substantive,
            "ADVERBO" => POS::Adverb,
            "INTERJEKCIO" => POS::Interjection,
            "MALLONGIGO" => POS::Abbreviation,
            "VERBO" => POS::Verb,
            "ADJ" => POS::Adjective,
            "PRONOMADJ" => POS::PronounAdjective,
            "SUFIKSO" => POS::Suffix,
            "PREPOZICIO" => POS::Preposition,
            "LITERO" => POS::Letter,
            "TEHXPREFIKSO" => POS::TechPrefix,
            "PRONOMO" => POS::Pronoun,
            "PARTICIPO" => POS::Participle,
            "KONJUNKCIO" => POS::Conjunction,
            "NUMERO" => POS::Number,
            "SUBJUNKCIO" => POS::Subjunction,
            "ARTIKOLO" => POS::Article,
            "PREFIKSO" => POS::Prefix,
            "SUBSTVERBO" => POS::SubstantiveVerb,
            _ => POS::Substantive,
        }
    }
}  // POS


 // Capitalization: miniscule, majuscule, entirely majuscule.
#[derive(Debug, Clone)]
pub enum Capitalization {
    Miniscule,		// butero
    Majuscule,		// Kanado
    AllCaps,		// UEA
}


// Determines the capitalization of a word.
impl Capitalization {
    pub fn new(s: &str) -> Capitalization {
        let mut iter = s.chars();
        let unua;
        let dua;
        match iter.next() {
            None => panic!("Capitalization::new : La literĉeno estas malplena."),
            Some(c) => { unua = c.is_uppercase(); }
        }
        match iter.next() {
            None => { dua = false; },
            Some(c) => { dua = c.is_uppercase(); }
        }
        if unua && dua { return Capitalization::AllCaps; }
        if unua { return Capitalization::Majuscule; }
        Capitalization::Miniscule
    }
}  // end of impl Capitalization 


// 'Meaning' defines the domain of a word. For example, 'bizon', 'cerv' and
// 'hipopotam' have a meaning 'Mamulo' (Mammal).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Meaning {
    NeKonata,	// unknown - most common domain
    Legomo,
    Boato,
    Krustulo,
    Insulo,
    Religio,
    Herbo,
    Koloro,
    Planto,
    Festo,
    Libro,
    Loko,
    Drogo,
    Lago,
    Pseuxdoscienco,
    ReligiaPosteno,
    Profesio,
    Gramatiko,
    Ehxinodermo,
    Medikamento,
    Regiono,
    Biologio,
    Birdo,
    Urbo,
    Veturilo,
    Lando,
    Etno,
    Kanto,
    Vestajxo,
    Titolo,
    Reganto,
    Rivero,
    Arto,
    Erao,
    Provinco,
    Muziko,
    Persono,
    Sxtato,
    Mamulo,
    Fisxo,
    Mezurunuo,
    Fungo,
    Kuracarto,
    Armilo,
    Algo,
    Koelentero,
    Nukso,
    Monto,
    Geografio,
    Tehxnologio,
    Monato,
    Arkitekturo,
    Insularo,
    Metio,
    Astronomio,
    Kredo,
    Molusko,
    Reptilio,
    Trinkajxo,
    Animalo,
    Insekto,
    Frukto,
    Arbusto,
    Araknido,
    Aviadilo,
    Sporto,
    Elemento,
    Alojo,
    ReligiaPersono,
    ReligiaProfesio,
    Kemiajxo,
    Filozofio,
    Sxtofo,
    Posteno,
    Parenco,
    Konstruajxo,
    Cerealo,
    Danco,
    Tago,
    Poemo,
    Sxipo,
    Ludilo,
    Poezio,
    Cxambro,
    Mangxajxo,
    Astro,
    Ilo,
    Mikrobo,
    Ludo,
    Dezerto,
    MitaBesto,
    Dramo,
    Vetero,
    Arbo,
    Scienco,
    Ornamajxo,
    Vermo,
    Mineralo,
    Spico,
    Masxino,
    Kontinento,
    Periodo,
    Lingvo,
    Mezurilo,
    Maro,
    Montaro,
    MitaPersono,
    Fonetiko,
    Monero,
    Matematiko,
    Rango,
    Anatomio,
    Studo,
    Optiko,
    Amfibio,
    Malsano,
    Muzikilo,
    Geometrio,
}

impl Meaning {
    pub fn new(s: &str) -> Meaning {
        match s {
            "N" => Meaning::NeKonata,
            "LEGOMO" => Meaning::Legomo,
            "BOATO" => Meaning::Boato,
            "KRUSTULO" => Meaning::Krustulo,
            "INSULO" => Meaning::Insulo,
            "RELIGIO" => Meaning::Religio,
            "HERBO" => Meaning::Herbo,
            "KOLORO" => Meaning::Koloro,
            "PLANTO" => Meaning::Planto,
            "FESTO" => Meaning::Festo,
            "LIBRO" => Meaning::Libro,
            "LOKO" => Meaning::Loko,
            "DROGO" => Meaning::Drogo,
            "LAGO" => Meaning::Lago,
            "PSEUXDOSCI" => Meaning::Pseuxdoscienco,
            "RELPOSTENO" => Meaning::ReligiaPosteno,
            "PROFESIO" => Meaning::Profesio,
            "GRAMATIKO" => Meaning::Gramatiko,
            "EHXINODERMO" => Meaning::Ehxinodermo,
            "MEDIKAMENTO" => Meaning::Medikamento,
            "REGIONO" => Meaning::Regiono,
            "BIOLOGIO" => Meaning::Biologio,
            "BIRDO" => Meaning::Birdo,
            "URBO" => Meaning::Urbo,
            "VETURILO" => Meaning::Veturilo,
            "LANDO" => Meaning::Lando,
            "ETNO" => Meaning::Etno,
            "KANTO" => Meaning::Kanto,
            "VESTAJXO" => Meaning::Vestajxo,
            "TITOLO" => Meaning::Titolo,
            "REGANTO" => Meaning::Reganto,
            "RIVERO" => Meaning::Rivero,
            "ARTO" => Meaning::Arto,
            "ERAO" => Meaning::Erao,
            "PROVINCO" => Meaning::Provinco,
            "MUZIKO" => Meaning::Muziko,
            "PERSONO" => Meaning::Persono,
            "SXTATO" => Meaning::Sxtato,
            "MAMULO" => Meaning::Mamulo,
            "FISXO" => Meaning::Fisxo,
            "MEZURUNUO" => Meaning::Mezurunuo,
            "FUNGO" => Meaning::Fungo,
            "KURACARTO" => Meaning::Kuracarto,
            "ARMILO" => Meaning::Armilo,
            "ALGO" => Meaning::Algo,
            "KOELENTERO" => Meaning::Koelentero,
            "NUKSO" => Meaning::Nukso,
            "MONTO" => Meaning::Monto,
            "GEOGRAFIO" => Meaning::Geografio,
            "TEHXNOLOGIO" => Meaning::Tehxnologio,
            "MONATO" => Meaning::Monato,
            "ARKITEKTURO" => Meaning::Arkitekturo,
            "INSULARO" => Meaning::Insularo,
            "METIO" => Meaning::Metio,
            "ASTRONOMIO" => Meaning::Astronomio,
            "KREDO" => Meaning::Kredo,
            "MOLUSKO" => Meaning::Molusko,
            "REPTILIO" => Meaning::Reptilio,
            "TRINKAJXO" => Meaning::Trinkajxo,
            "ANIMALO" => Meaning::Animalo,
            "INSEKTO" => Meaning::Insekto,
            "FRUKTO" => Meaning::Frukto,
            "ARBUSTO" => Meaning::Arbusto,
            "ARAKNIDO" => Meaning::Araknido,
            "AVIADILO" => Meaning::Aviadilo,
            "SPORTO" => Meaning::Sporto,
            "ELEMENTO" => Meaning::Elemento,
            "ALOJO" => Meaning::Alojo,
            "RELPERSONO" => Meaning::ReligiaPersono,
            "RELPROFESIO" => Meaning::ReligiaProfesio,
            "KEMIAJXO" => Meaning::Kemiajxo,
            "FILOZOFIO" => Meaning::Filozofio,
            "SXTOFO" => Meaning::Sxtofo,
            "POSTENO" => Meaning::Posteno,
            "PARENCO" => Meaning::Parenco,
            "KONSTRUAJXO" => Meaning::Konstruajxo,
            "CEREALO" => Meaning::Cerealo,
            "DANCO" => Meaning::Danco,
            "TAGO" => Meaning::Tago,
            "POEMO" => Meaning::Poemo,
            "SXIPO" => Meaning::Sxipo,
            "LUDILO" => Meaning::Ludilo,
            "POEZIO" => Meaning::Poezio,
            "CXAMBRO" => Meaning::Cxambro,
            "MANGXAJXO" => Meaning::Mangxajxo,
            "ASTRO" => Meaning::Astro,
            "ILO" => Meaning::Ilo,
            "MIKROBO" => Meaning::Mikrobo,
            "LUDO" => Meaning::Ludo,
            "DEZERTO" => Meaning::Dezerto,
            "MITBESTO" => Meaning::MitaBesto,
            "DRAMO" => Meaning::Dramo,
            "VETERO" => Meaning::Vetero,
            "ARBO" => Meaning::Arbo,
            "SCIENCO" => Meaning::Scienco,
            "ORNAMAJXO" => Meaning::Ornamajxo,
            "VERMO" => Meaning::Vermo,
            "MINERALO" => Meaning::Mineralo,
            "SPICO" => Meaning::Spico,
            "MASXINO" => Meaning::Masxino,
            "KONTINENTO" => Meaning::Kontinento,
            "PERIODO" => Meaning::Periodo,
            "LINGVO" => Meaning::Lingvo,
            "MEZURILO" => Meaning::Mezurilo,
            "MARO" => Meaning::Maro,
            "MONTARO" => Meaning::Montaro,
            "MITPERSONO" => Meaning::MitaPersono,
            "FONETIKO" => Meaning::Fonetiko,
            "MONERO" => Meaning::Monero,
            "MATEMATIKO" => Meaning::Matematiko,
            "RANGO" => Meaning::Rango,
            "ANATOMIO" => Meaning::Anatomio,
            "STUDO" => Meaning::Studo,
            "OPTIKO" => Meaning::Optiko,
            "AMFIBIO" => Meaning::Amfibio,
            "MALSANO" => Meaning::Malsano,
            "MUZIKILO" => Meaning::Muzikilo,
            "GEOMETRIO" => Meaning::Geometrio,
            _ => Meaning::NeKonata,
        }
    }
}  // end of impl Meaning


/*
 * is_person()
 *
 * This function checks whether the given meaning represents a person.
 * Params:
 *     meaning
 * Return:
 *     true if person, false otherwise
 */
pub fn is_person(meaning: Meaning) -> bool {
    let person = match meaning {
        Meaning::Persono => true,
        Meaning::Parenco => true,
        Meaning::Etno => true,
        Meaning::Profesio => true,
        Meaning::Rango => true,
        Meaning::Reganto => true,
        Meaning::Titolo => true,
        Meaning::Posteno => true,
        Meaning::ReligiaPosteno => true,
        Meaning::ReligiaPersono => true,
        Meaning::ReligiaProfesio => true,
        Meaning::MitaPersono => true,
        _ => false,
    };
    return person;
}

/*
 * is_animal()
 *
 * This function checks whether the given meaning represents an animal.
 * Params:
 *     meaning
 * Return:
 *     true if person, false otherwise
 */
pub fn is_animal(meaning: Meaning) -> bool {
    let animal = match meaning {
        Meaning::Animalo => true,
        Meaning::Mamulo => true,
        Meaning::Birdo => true,
        Meaning::Fisxo => true,
        Meaning::Reptilio => true,
        Meaning::MitaBesto => true,
        Meaning::Insekto => true,
        Meaning::Araknido => true,
        Meaning::Molusko => true,
        Meaning::Amfibio => true,
        _ => false,
    };
    return animal;
}


// Transitivity - property of verbs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transitivity {
    Transitive,	// vidis
    Intransitive,	// dormas
    Both,		// ludas
}

impl Transitivity {
    pub fn new(s: &str) -> Transitivity {
        match s {
            "T" => Transitivity::Transitive,
            "N" => Transitivity::Intransitive,
            _ => Transitivity::Both,
        }
    }
}

// Without Ending
// This property indicates whether a morpheme can stand without an
// ending. For example, 'dum' doesn't require an ending, but 'vintr' does (vintro).
#[derive(Debug, Clone, PartialEq)]
pub enum WithoutEnding {
    Yes,
    No,
}

impl WithoutEnding {
    pub fn new(s: &str) -> WithoutEnding {
        match s {
            "SF" => WithoutEnding::Yes,
            _ => WithoutEnding::No,
        }
    }
}


// With Ending
// This property indicates whether a morpheme can accept an ending or not.
// For example, 'arb' takes an ending (generally 'o'), but 'bo-' generally does not.
// It must act as a prefix ('bopatro').
#[derive(Debug, Clone, PartialEq)]
pub enum WithEnding {
    Yes,
    No,
}

impl WithEnding {
    pub fn new(s: &str) -> WithEnding {
        match s {
            "KF" => WithEnding::Yes,
            _ => WithEnding::No,
        }
    }
}



// Synthesis - Defines limits on morphology.
// Difinas kiel radikoj kuniĝas en morfologio.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Synthesis {
    Suffix,	// The morpheme acts like a suffix.
    Prefix,	// The morpheme acts like a prefix.
    Participle,	// The morpheme acts like a participle ending (-int, -it, etc.)
    Limited,	// Limited combinability.
    UnLimited,
    No,   // Does not combine.
}

impl Synthesis {
    pub fn new(s: &str) -> Synthesis {
        match s {
            "S" => Synthesis::Suffix,
            "P" => Synthesis::Prefix,
            "PRT" => Synthesis::Participle,
            "LM" => Synthesis::Limited,
            "NLM" => Synthesis::UnLimited,
            _ => Synthesis::No,
        }
    }
}


// Flag - This flag id used to distinguish types of dictionary entries.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Flag {
    Simple,	// A simple root (morpheme). Eg. 'muzik'
    Compound,	// A compound word. Eg. 'muzik.il'
    Exclude,	// Exclude from dictionary. For reference only.
    Separator,	// Flags a separator between morphemes. Eg. 'fingr.o.montr.i'
}

impl Flag {
    pub fn new(s: &str) -> Flag {
        match s {
            "R" => Flag::Simple,
            "K" => Flag::Compound,
            _ => Flag::Exclude,
        }
    }
}


// Define a dictionary entry.
#[derive(Debug, Clone)]
pub struct Entry {
    pub word: String,
    pub length: usize,
    pub capitalization: Capitalization,
    pub part_of_speech: POS,
    pub meaning: Meaning,
    pub transitivity: Transitivity,		// transitivity
    pub without_ending: WithoutEnding,	// valid without ending
    pub with_ending: WithEnding,	// valid with ending
    pub synthesis: Synthesis,	// for constraints on word synthesis (Limigo)
    pub rarity: usize,			// 1 is common, 4 is rare
    pub flag: Flag,			// flag (R = radiko (morpheme), K = compound X = exclude)
}

impl Entry {	// A dictionary entry

    /* new()
     * Assumes that the items in 'fields' are valid.
     * Params: vector of dictionary data (strings)
     * Return: optional dictionary entry
     */
    pub fn new(fields: Vec<&str>) -> Option<Entry> {

        let _word = String::from(fields[0]);
        let _word2 = x_to_accent(&_word);
        let _length: usize = _word2.chars().count();
        if _length == 1 { return None; }
        let _flag = String::from(fields[8]);
        if _flag == "X" { return None; }    // X means exclude.
        let _vortspeco = String::from(fields[1]);
        let _meaning = String::from(fields[2]);
        let _transitivity = String::from(fields[3]);
        let _without_ending = String::from(fields[4]);
        let _with_ending = String::from(fields[5]);
        let _synthesis = String::from(fields[6]);
        let _rarity = fields[7].parse().unwrap();

        let entry = Entry {
            word: _word2.clone(),
            length: _length,
            part_of_speech: POS::new(&_vortspeco),
            capitalization: Capitalization::new(&_word2),
            meaning: Meaning::new(&_meaning),
            transitivity: Transitivity::new(&_transitivity),
            without_ending: WithoutEnding::new(&_without_ending),
            with_ending: WithEnding::new(&_with_ending),
            synthesis: Synthesis::new(&_synthesis),
            rarity: _rarity,
            flag: Flag::new(&_flag),
         };

        // println!("____ {}", _synthesis);
        return Some(entry)

    }  // end of pub fn new


    /* empty()
     *
     * Creates an empty dictionary entry.
     *
     * Return: dictionary entry
     */
    pub fn empty() -> Entry {

        let entry = Entry {
            word: String::new(),
            length: 0,
            part_of_speech: POS::Substantive,
            capitalization: Capitalization::Miniscule,
            meaning: Meaning::NeKonata,
            transitivity: Transitivity::Transitive,
            without_ending: WithoutEnding::No,
            with_ending: WithEnding::No,
            synthesis: Synthesis::No,
            rarity: 0,
            flag: Flag::Simple,
        };
        entry

    }  // pub fn empty


    /* new_separator()
     *
     * This function creates an entry to define a 'separator', that is, a grammatical
     * ending placed between morphemes to aid pronunciation. For example,
     * 'fingr.o.montr.i'; the 'o' is a separator. For the moment, a valid separator will
     * be 'o', 'a' or 'e'.
     *
     * Params: separator string
     * Return: dictionary entry for separator
     */
    pub fn new_separator(separator: &str) -> Option<Entry> {

        let pos = match separator {
            "o" => POS::Substantive,
            "a" => POS::Adjective,
            "e" => POS::Adverb,
            _ => return None,
        };

        let entry = Entry {
            word: format!("{}", separator),
            length: 1,
            part_of_speech: pos,
            capitalization: Capitalization::Miniscule,
            meaning: Meaning::NeKonata,
            transitivity: Transitivity::Intransitive,
            without_ending: WithoutEnding::No,
            with_ending: WithEnding::No,
            synthesis: Synthesis::No,
            rarity: 4,
            flag: Flag::Separator,
        };

        return Some(entry)

    }  // pub fn new_separator

}  // end of impl Entry



// Case, nominative or accusative
/*
#[derive(Debug, Clone)]
pub enum Case {
    Nominative,   // arbo
    Accusative,   // arbon
}

// If a substantive or adjective ends in 'n' or 'N', it is accusative.
// Otherwise assume it's nominative.
impl Case {
    pub fn new(s: &str) -> Case {
        if  s.len() < 1 { return Case::Nominative; }  // default
        let last = s.chars().last().unwrap();
        match last {
            'n' => Case::Accusative,
            'N' => Case::Accusative,
            _ => Case::Nominative,
        }
    }
}

// Plurality: singular or plural.
#[derive(Debug, Clone)]
pub enum Plurality {
    Singular,  // arbo
    Plural,  // arboj
}

// If a substantive or adjective ends in 'j', 'J', 'jn' or 'JN', it's plural.
// Otherwise assume it's singular.
// arbo - Singular
// arboj - Plural
// arbon - Singular
// arbojn - Plural
impl Plurality {
    pub fn new(s: &str) -> Plurality {
        if  s.len() < 2 { return Plurality::Singular; }
        let mut iter = s.chars().rev();
        let last = iter.next().unwrap();
        let _second_last = iter.next().unwrap();
        if last == 'j' || last == 'J' { return Plurality::Plural; }
        else if last == 'n' || last == 'N' {
            match _second_last {
                'j' => Plurality::Plural,
                'J' => Plurality::Plural,
                _ => Plurality::Singular,
            }
        }
        else {
            Plurality::Singular
        }
    }
}

*/
