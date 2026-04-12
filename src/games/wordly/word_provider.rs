use rand::Rng;
use std::fs;
use std::fs::read_to_string;

pub struct WordProvider {}

impl WordProvider {
    pub fn get_all_wards() -> Vec<String> {
        let nouns_path = std::path::PathBuf::from("assets/all_nouns_ru.txt");
        read_to_string(nouns_path)
            .unwrap()
            .split("\n")
            .map(|s| s.trim().to_string())
            .collect()
    }

    pub fn get_local_words_5_ru() -> Vec<String> {
        let nouns_path = std::path::PathBuf::from("assets/words_5_ru.txt");
        read_to_string(nouns_path)
            .unwrap()
            .split("\n")
            .map(|s| s.trim().to_string())
            .collect()
    }

    pub fn get_one_word_5_ru() -> String {
        let words = Self::get_local_words_5_ru();
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen_range(0..words.len() as i32);
        words[x as usize].clone()
    }
}
