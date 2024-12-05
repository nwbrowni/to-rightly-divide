use super::reference::Reference;

#[derive(Clone)]
pub struct Verse {
    reference: Reference,
    content: String,
}

impl Verse {
    pub fn new<'b>(reference: Reference, content: &'b str) -> Self {
        Self {
            reference,
            content: content.to_string(),
        }
    }

    pub fn reference(&self) -> Reference {
        self.reference.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn exact_search<'b>(&self, query: &'b str) -> bool {
        self.content.contains(query)
    }

    pub fn caseless_search<'b>(&self, query: &'b str) -> bool {
        self.content.to_lowercase().contains(query.to_lowercase().as_str())
    }

    pub fn substring_search<'b>(&self, query: &'b str) -> bool {
        let words: Vec<&str> = query.split(' ').collect();
        let mut subs_found = 0;
        let words_len = words.len();
        for word in words {
            if self.content.contains(format!(" {} ", word).as_str()) { subs_found += 1 }
            else if self.content.contains(format!(" {} ", word).to_lowercase().as_str()) { subs_found += 1 }
        }
        if subs_found as f32 / words_len as f32 > 0.7 {
            return true
        }
        return false
    }

    pub fn reference_search<'b>(&self, query: &'b str) -> bool {
        let spaces: Vec<&str> = query.split(" ").collect();
        if spaces.len() < 2 { return false; }
        let book = spaces[0].to_string();
        let semi: Vec<&str> = spaces[1].split(":").collect();
        if semi.len() < 2 { return false; }
        let chapter;
        match semi[1].to_string().parse::<u8>() {
            Ok(n) => { chapter = n; }
            Err(_) => { return false; }
        }
        let verse;
        match semi[1].to_string().parse::<u16>() {
            Ok(n) => { verse = n; }
            Err(_) => { return false; }
        }
        //println!("{} {}:{} | {} {}:{}", book, chapter, verse, self.reference.book(), self.reference.chapter(), self.reference.verse());
        let mut all_good = true;
        all_good &= self.reference.book().contains(book.as_str()) || self.reference.book().to_lowercase().contains(book.as_str()) || self.reference.book().contains(book.to_lowercase().as_str());
        all_good &= self.reference.chapter() == chapter;
        all_good &= self.reference.verse() == verse;
        return all_good
    }

    pub fn search<'b>(&self, query: &'b str) -> bool {
        let mut found = false;
        found |= self.exact_search(query);
        if found { return found }
        found |= self.caseless_search(query);
        if found { return found }
        found |= self.substring_search(query);
        if found { return found }
        found |= self.reference_search(query);
        return found
    }
}