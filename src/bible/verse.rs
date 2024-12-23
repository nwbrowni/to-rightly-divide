use super::reference::Reference;

#[derive(Clone, Copy)]
pub enum SearchMethod {
    Exact,
    Caseless,
    Substring,
    Reference,
    All,
}

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

    fn exact_search<'b>(&self, query: &'b str) -> bool {
        self.content.contains(query)
    }

    fn caseless_search<'b>(&self, query: &'b str) -> bool {
        self.content.to_lowercase().contains(query.to_lowercase().as_str())
    }

    fn substring_search<'b>(&self, query: &'b str) -> bool {
        let words: Vec<&str> = query.split(' ').collect();
        let mut subs_found = 0;
        let words_len = words.len();
        for word in words {
            if self.content.contains(word) { subs_found += 1 }
            else if self.content.contains(word.to_lowercase().as_str()) { subs_found += 1 }
        }
        if subs_found as f32 / words_len as f32 > 0.75 {
            return true
        }
        return false
    }

    fn get_chapter<'b>(&self, string: &'b str) -> u8 {
        match string.to_string().parse::<u8>() {
            Ok(n) => { return n; }
            Err(_) => { return 0; }
        }
    }

    fn get_verse<'b>(&self, string: &'b str) -> u16 {
        match string.to_string().parse::<u16>() {
            Ok(n) => { return n; }
            Err(_) => { return 0; }
        }
    }

    fn reference_search<'b>(&self, query: &'b str) -> bool {
        let spaces: Vec<&str> = query.split(" ").collect();
        if spaces.len() != 2 { return false; }  // this doesn't work for numbered books
        let book = spaces[0].to_string();
        let semi: Vec<&str> = spaces[1].split(":").collect();
        if semi.len() < 1 { return false; }  // might want to return whole chapter here
        let chapter = self.get_chapter(semi[0]);
        let mut all_good = true;
        all_good &= self.reference.book().contains(book.as_str()) || self.reference.book().to_lowercase().contains(book.as_str()) || self.reference.book().contains(book.to_lowercase().as_str());
        all_good &= self.reference.chapter() == chapter;
        if semi.len() == 1 { return all_good }
        if semi.len() != 2 { return false }
        let verse = self.get_verse(semi[1]);
        all_good &= self.reference.verse() == verse;
        return all_good
    }

    // pub fn search<'b>(&self, query: &'b str) -> bool {
    //     let mut found = false;
    //     found |= self.exact_search(query);
    //     if found { return found }
    //     found |= self.caseless_search(query);
    //     if found { return found }
    //     found |= self.substring_search(query);
    //     if found { return found }
    //     found |= self.reference_search(query);
    //     return found
    // }

    pub fn search<'b>(&self, method: SearchMethod, query: &'b str) -> bool {
        match method {
            SearchMethod::Exact => self.exact_search(query),
            SearchMethod::Caseless => self.caseless_search(query),
            SearchMethod::Substring => self.substring_search(query),
            SearchMethod::Reference => self.reference_search(query),
            SearchMethod::All => {
                let mut found = false;
                found |= self.exact_search(query);
                if found { return found }
                found |= self.caseless_search(query);
                if found { return found }
                found |= self.substring_search(query);
                if found { return found }
                found |= self.reference_search(query);
                return found
            }, 
        }
    }
}