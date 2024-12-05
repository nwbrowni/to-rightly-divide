#[derive(Clone)]
pub struct Reference {
    book: String,
    chapter: u8,
    verse: u16,
}

impl Reference {
    pub fn new<'b>(book: &'b str, chapter: u8, verse: u16) -> Self {
        Self {
            book: book.to_string(),
            chapter,
            verse,
        }
    }

    pub fn book(&self) -> String {
        self.book.clone()
    }

    pub fn chapter(&self) -> u8 {
        self.chapter
    }

    pub fn verse(&self) -> u16 {
        self.verse
    }

    pub fn full(&self) -> (String, u8, u16) {
        (self.book(), self.chapter(), self.verse())
    }
}