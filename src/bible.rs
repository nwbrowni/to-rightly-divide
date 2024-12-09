use std::{fs::File, io::{self, BufRead}};

use book::Book;
use reference::Reference;
use verse::Verse;

pub mod reference;
pub mod verse;
pub mod chapter;
pub mod book;

#[derive(Clone)]
pub struct Bible {
    version: String,
    books: Vec<Book>,
}

impl Bible {
    pub fn default<'b>(version: &'b str) -> Self {
        Self {
            version: version.to_string(),
            books: Vec::new(),
        }
    }

    pub fn new<'b>(version: &'b str, books: Vec<Book>) -> Self {
        Self {
            version: version.to_string(),
            books,
        }
    }

    pub fn new_from_file<'b>(version: &'b str, file: &'b str) -> Self {
        let mut bible2 = Bible::default(version);

        // read in and parse all verses from file
        match File::open(file) {
            Ok(file) => {
                for line in io::BufReader::new(file).lines().flatten() {
                    let pipes: Vec<&str> = line.split('|').collect();
                    let book = pipes[0].to_string();
                    let ch_v: Vec<&str> = pipes[1].split(':').collect();
                    let ch: u8;
                    match ch_v[0].to_string().parse::<u8>() {
                        Ok(n) => { ch = n; }
                        Err(_) => {
                            println!("could not parse chapter...");
                            panic!()
                        }
                    }
                    let v: u16;
                    match ch_v[1].to_string().parse::<u16>() {
                        Ok(n) => { v = n; }
                        Err(_) => {
                            println!("could not parse verse...");
                            panic!()
                        }
                    }
                    bible2.add_verse(&Verse::new(Reference::new(book.as_str(), ch, v), pipes[2]));
                }
            },
            Err(_) => {
                println!("could not open file {}...", file);
                panic!()
            }
        }
        Self {
            version: bible2.version.clone(),
            books: bible2.books.clone(),
        }
    }

    pub fn add_verse(&mut self, verse: &Verse) {
        let mut added = false;
        for book in &mut self.books {
            if book.name() == verse.reference().book() {
                book.add_verse(&verse);
                added = true;
                break;
            }
        }
        if !added {
            let mut book = Book::default(verse.reference().book().as_str());
            book.add_verse(&verse);
            self.books.push(book);
        }
    }

    pub fn search<'b>(&self, method: verse::SearchMethod, query: &'b str) -> Vec<Verse> {
        let mut found = Vec::new();
        for book in &self.books {
            for verse in book.search(method, query) {
                found.push(verse.clone());
            }
        }
        return found;
    }
}

// pub struct Bible {
//     version: String,
//     verses: Vec<Verse>,
// }

// impl Bible {
//     pub fn new<'b>(version: &'b str, verses: Vec<Verse>) -> Self {
//         Self {
//             version: version.to_string(),
//             verses
//         }
//     }

//     pub fn new_from_file<'b>(version: &'b str, file: &'b str) -> Self {
//         let mut verses = Vec::<Verse>::new();

//         // read in and parse all verses from file
//         match File::open(file) {
//             Ok(file) => {
//                 for line in io::BufReader::new(file).lines().flatten() {
//                     let pipes: Vec<&str> = line.split('|').collect();
//                     let book = pipes[0].to_string();
//                     let ch_v: Vec<&str> = pipes[1].split(':').collect();
//                     let ch: u8;
//                     match ch_v[0].to_string().parse::<u8>() {
//                         Ok(n) => { ch = n; }
//                         Err(_) => {
//                             println!("could not parse chapter...");
//                             panic!()
//                         }
//                     }
//                     let v: u16;
//                     match ch_v[1].to_string().parse::<u16>() {
//                         Ok(n) => { v = n; }
//                         Err(_) => {
//                             println!("could not parse verse...");
//                             panic!()
//                         }
//                     }
//                     verses.push(Verse::new(Reference::new(book.as_str(), ch, v), pipes[2]));
//                 }
//             },
//             Err(_) => {
//                 println!("could not open file {}...", file);
//                 panic!()
//             }
//         }
//         Bible::new(version, verses)
//     }

//     pub fn version(&self) -> String {
//         self.version.clone()
//     }
    
//     pub fn exact_search<'b>(&self, query: &'b str) -> Vec<Verse> {
//         let mut found = Vec::new();
//         for verse in &self.verses {
//             if verse.exact_search(query) {
//                 found.push(verse.clone());
//             }
//         }
//         return found;
//     }

//     pub fn caseless_search<'b>(&self, query: &'b str) -> Vec<Verse> {
//         let mut found = Vec::new();
//         for verse in &self.verses {
//             if verse.caseless_search(query) {
//                 found.push(verse.clone());
//             }
//         }
//         return found;
//     }

//     pub fn substring_search<'b>(&self, query: &'b str) -> Vec<Verse> {
//         let mut found = Vec::new();
//         for verse in &self.verses {
//             if verse.substring_search(query) {
//                 found.push(verse.clone());
//             }
//         }
//         return found;
//     }

//     pub fn reference_search<'b>(&self, query: &'b str) -> Vec<Verse> {
//         let mut found = Vec::new();
//         for verse in &self.verses {
//             if verse.reference_search(query) {
//                 found.push(verse.clone());
//             }
//         }
//         return found;
//     }

//     // break into subsearch categories
//     pub fn search<'b>(&self, query: &'b str) -> Vec<Verse> {
//         let mut found = Vec::new();
//         for verse in &self.verses {
//             if verse.search(query) {
//                 found.push(verse.clone());
//             }
//         }
//         return found;
//     }
// }