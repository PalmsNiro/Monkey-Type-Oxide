use rand::seq::SliceRandom;
use rand::thread_rng;
use random_word::Lang;
use ratatui::text::ToText;

use serde_json::Deserializer;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::app_options::*;

// Text Length for simple sentences
const TEXT_LEN: usize = 30;

#[derive(Deserialize)]
struct LanguageWords {
    words: Vec<String>,
    #[serde(skip)]  // Diese Felder werden übersprungen
    name: String,
    #[serde(skip)]
    bcp47: String,
    #[serde(skip)]
    additional_accents: Vec<Vec<String>>,
}

pub fn get_sentence(lan:Language, test_type:TestType) -> String{
    let lan = match lan{
        Language::De => Lang::De,
        Language::En => Lang::En,
    };
    let test_text = match test_type {
        TestType::RandomWords => get_random_sentence( lan),
        TestType::RandomWords1K => get_random_sentence1k(lan),
        TestType::RandomWords10K => todo!(),
        TestType::Quotes => todo!(),
        TestType::TimeRace => todo!(),
        TestType::Jokes => todo!(),
        TestType::Hardcore => todo!(),
    };
    test_text
}


fn get_random_sentence(language: Lang) -> String {

    let word_list: &[&str] = random_word::all(language);
    let mut rng = thread_rng();

    let selected_words: Vec<&str> = word_list
        .choose_multiple(&mut rng, TEXT_LEN)
        .cloned()
        .collect();

    let mut sentence = selected_words.join(" ");

    if language == Lang::De{
        sentence = replace_umlauts(sentence);
    };
    sentence
}

fn get_random_sentence1k (language: Lang) ->String{
    match read_words_from_file("util/german.json"){
        Ok(words)=>{choose_random_words(language, words)

        }
        Err(e) => format!("failure: {}",e),
    } 
}

fn replace_umlauts(sentence: String)->String{
    sentence
    .replace("ä", "ae")
    .replace("ö", "oe")
    .replace("ü", "ue")
    .replace("Ä", "Ae")
    .replace("Ö", "Oe")
    .replace("Ü", "Ue")
    .replace("ß", "ss")
}




fn read_words_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance 
    let languge: LanguageWords = serde_json::from_reader(reader)?;

    Ok(languge.words)
}


fn choose_random_words(language: Lang, words: Vec<String>) -> String{
    let mut rng = thread_rng();
    let selected_words: Vec<String> = words.choose_multiple(&mut rng, TEXT_LEN).cloned().collect();

    let mut sentence = selected_words.join(" ");

    if language == Lang::De{
        sentence = replace_umlauts(sentence);
    };
    sentence
}