// src/lib.rs
use wasm_bindgen::prelude::*;
use std::collections::HashSet;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Hangman {
    word: String,
    guessed_letters: HashSet<char>,
    incorrect_guesses: u32,
    max_incorrect: u32,
}

#[wasm_bindgen]
impl Hangman {
    #[wasm_bindgen(constructor)]
    pub fn new(word: &str) -> Hangman {
        Hangman {
            word: word.to_lowercase(),
            guessed_letters: HashSet::new(),
            incorrect_guesses: 0,
            max_incorrect: 6,
        }
    }

    pub fn guess(&mut self, letter: char) -> Option<bool> {
        if self.guessed_letters.contains(&letter) {
            return None;
        }

        self.guessed_letters.insert(letter);

        let correct = self.word.contains(letter);
        if !correct {
            self.incorrect_guesses += 1;
        }

        Some(correct)
    }

    pub fn current_state(&self) -> String {
        self.word
            .chars()
            .map(|c| if self.guessed_letters.contains(&c) { c } else { '_' })
            .collect()
    }

    pub fn is_over(&self) -> bool {
        self.incorrect_guesses >= self.max_incorrect || self.is_won()
    }

    pub fn is_won(&self) -> bool {
        self.word
            .chars()
            .all(|c| self.guessed_letters.contains(&c))
    }

    pub fn incorrect_guesses(&self) -> u32 {
        self.incorrect_guesses
    }

    pub fn max_incorrect(&self) -> u32 {
        self.max_incorrect
    }
}
