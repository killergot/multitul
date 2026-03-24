use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Attempt{
    pub word: String,
    // 0 - nothing, 1 - somewhere, 2 - in point
    // exaple: goal - ship, attempt - glip => marked - [2,1,0,0]
    pub marked: [u8; 5]
}

impl Attempt{
    pub fn new(goal_word: String, attempt_word: String) -> Attempt{
        let mut counter: HashMap<char, u8> = HashMap::new();
        let mut marked = [0;5];
        for c in goal_word.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        for (i,c) in attempt_word.chars().enumerate() {
            if goal_word.chars().nth(i) == Some(c){
                marked[i] = 2;
                *counter.get_mut(&c).unwrap() -= 1;
            }
        }
        for (i,c) in attempt_word.chars().enumerate() {
            if counter.contains_key(&c) && counter[&c] > 0 && marked[i] != 2{
                marked[i] = 1;
                *counter.get_mut(&c).unwrap() -= 1;
            }
        }

        Attempt{word: attempt_word, marked}
    }
}