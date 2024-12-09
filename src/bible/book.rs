/*
Future feature
*/

use super::{chapter::Chapter, verse::{self, Verse}};

#[derive(Clone)]
pub struct Book {
    name: String,
    pub chapters: Vec<Chapter>
}

impl Book {
    pub fn default<'b>(name: &'b str) -> Self {
        Self {
            name: name.to_string(),
            chapters: Vec::new(),
        }
    }

    pub fn new<'b>(name: &'b str, chapters: Vec<Chapter>) -> Self {
        Self {
            name: name.to_string(),
            chapters
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn add_verse(&mut self, verse: &Verse) {
        let mut added = false;
        for chapter in &mut self.chapters {
            if chapter.number() == verse.reference().chapter() {
                chapter.add_verse(verse);
                added = true;
                break;
            }
        }
        if !added {
            let mut chapter = Chapter::default(verse.reference().chapter());
            chapter.add_verse(verse);
            self.chapters.push(chapter);
        }
    }

    pub fn search<'b>(&self, method: verse::SearchMethod, query: &'b str) -> Vec<Verse> {
        let mut found = Vec::new();
        for chapter in &self.chapters {
            for verse in chapter.search(method, query) {
                found.push(verse.clone());
            }
        }
        return found;
    }
}