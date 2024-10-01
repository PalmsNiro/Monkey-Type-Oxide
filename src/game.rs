use rand::seq::SliceRandom;
use rand::thread_rng;
use random_word::Lang;

pub fn get_random_sentence(words_amount: usize) -> String {
    let word_list: &[&str] = random_word::all(Lang::De);
    let mut rng = thread_rng();

    let selected_words: Vec<&str> = word_list
        .choose_multiple(&mut rng, words_amount)
        .cloned()
        .collect();

    let sentence = selected_words.join(" ");
    sentence.chars().nth(0).unwrap().to_uppercase().to_string() + &sentence[1..]
}