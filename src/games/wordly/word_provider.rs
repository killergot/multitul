use std::fs;
use std::fs::read_to_string;

struct WordProvider{
}

impl WordProvider{
    fn get_all_wards(&self) -> String {
        let nouns_path = std::path::PathBuf::from("assets/all_nouns_ru.txt");
        read_to_string(nouns_path).unwrap()
    }

    fn local_get_words_5_ru(&self) -> String {
        let nouns_path = std::path::PathBuf::from("assets/words_5_ru.txt");
        read_to_string(nouns_path).unwrap()
    }
}