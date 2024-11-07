use rand::seq::SliceRandom;
use rand::thread_rng;
use random_word::Lang;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::app_options::*;

const TEXT_LEN: usize = 30;
const BASE_PATH: &str = "util"; 

enum WordSource<'a> {
    Dictionary(&'a [&'a str]),
    CustomList(Vec<String>),
}

#[derive(Deserialize)]
struct LanguageWords {
    words: Vec<String>,
    #[serde(skip)]
    name: String,
    #[serde(skip)]
    bcp47: String,
    #[serde(skip)]
    additional_accents: Vec<Vec<String>>,
}

#[derive(Debug)]
pub enum WordSetSize {
    Base,
    OneK,
    TenK,
}

impl WordSetSize {
    fn to_suffix(&self) -> &str {
        match self {
            WordSetSize::Base => "",
            WordSetSize::OneK => "_1k",
            WordSetSize::TenK => "_10k",
        }
    }
}

pub struct WordSetConfig {
    base_path: PathBuf,
}
impl WordSetConfig {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    pub fn get_words(&self, language: &str, size: WordSetSize) -> Result<Vec<String>, Box<dyn Error>> {
        let filename = format!("{}{}.json", language, size.to_suffix());
        let file_path = self.base_path.join(filename);
        self.read_words_from_file(file_path)
    }

    fn read_words_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<String>, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let language: LanguageWords = serde_json::from_reader(reader)?;
        Ok(language.words)
    }
}

pub fn get_sentence(lan: Language, test_type: TestType) -> String {
    let lang_str = match lan {
        Language::De => "german",
        Language::En => "english",
    };
    
    let language = match lan {
        Language::De => Lang::De,
        Language::En => Lang::En,
    };

    match test_type {
        TestType::RandomWords => {
            get_random_sentence(language, WordSource::Dictionary(random_word::all(language)))
        }
        TestType::RandomWords1K => {
            let config = WordSetConfig::new(BASE_PATH);
            match config.get_words(lang_str, WordSetSize::OneK) {
                Ok(words) => get_random_sentence(language, WordSource::CustomList(words)),
                Err(e) => format!("Error loading words: {}", e),
            }
        }
        TestType::RandomWords10K => {
            let config = WordSetConfig::new(BASE_PATH);
            match config.get_words(lang_str, WordSetSize::TenK) {
                Ok(words) => get_random_sentence(language, WordSource::CustomList(words)),
                Err(e) => format!("Error loading words: {}", e),
            }
        }
        TestType::Quotes => String::from("Quotes feature not implemented yet"),
        TestType::TimeRace => String::from("Time Race feature not implemented yet"),
        TestType::Jokes => String::from("Jokes feature not implemented yet"),
        TestType::Hardcore => String::from("Hardcore feature not implemented yet"),
    }
}

fn get_random_sentence(language: Lang, source: WordSource) -> String {
    let mut rng = thread_rng();

    // Choose words based on source
    let selected_words = match source {
        WordSource::Dictionary(word_list) => {
            word_list
                .choose_multiple(&mut rng, TEXT_LEN)
                .cloned()
                .collect::<Vec<&str>>()
                .join(" ")
        }
        WordSource::CustomList(words) => {
            words
                .choose_multiple(&mut rng, TEXT_LEN)
                .cloned()
                .collect::<Vec<String>>()
                .join(" ")
        }
    };

    // Handle umlauts 
    if language == Lang::De {
        replace_umlauts(selected_words)
    } else {
        selected_words
    }
}

fn replace_umlauts(sentence: String) -> String {
    sentence
        .replace("ä", "ae")
        .replace("ö", "oe")
        .replace("ü", "ue")
        .replace("Ä", "Ae")
        .replace("Ö", "Oe")
        .replace("Ü", "Ue")
        .replace("ß", "ss")
}