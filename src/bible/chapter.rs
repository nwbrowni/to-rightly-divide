/*
Future feature
*/

use super::verse::{self, Verse};

#[derive(Clone)]
pub struct Chapter {
    number: u8,
    pub verses: Vec<Verse>,
}

impl Chapter {
    pub fn default(number: u8) -> Self {
        Self {
            number,
            verses: Vec::new(),
        }
    }
    
    pub fn new(number: u8, verses: Vec<Verse>) -> Self {
        Self {
            number,
            verses,
        }
    }

    pub fn number(&self) -> u8 {
        self.number
    }

    pub fn add_verse(&mut self, verse: &Verse) {
        let mut found = false;
        for local_verse in &mut self.verses {
            if local_verse.reference().verse() == verse.reference().verse() {
                found = true;
                break;
            }
        }
        if !found {
            self.verses.push(verse.clone());
        }
    }

    pub fn search<'b>(&self, method: verse::SearchMethod, query: &'b str) -> Vec<Verse> {
        let mut found = Vec::new();
        for verse in &self.verses {
            if verse.search(method, query) {
                found.push(verse.clone());
            }
        }
        return found;
    }
}