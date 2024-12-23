use crate::bible::{verse::SearchMethod, Bible};

use super::UI;

pub struct CLI();

impl UI for CLI {
    fn launch(&self) {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}

impl CLI {
    fn print_main_menu(&self) {
        println!("Main Menu:");
        println!("1) Search");
        println!("2) Close");
    }

    fn process_main_menu(&self) -> u8 {
        todo!();
    }

    fn print_search_menu(&self) {
        println!("Search Menu:");
        println!("1) General");
        println!("2) Exact");
        println!("3) Caseless");
        println!("4) Substring");
        println!("5) Reference");
        println!("6) Close");
    }

    fn process_search_menu(&self) -> u8 {
        todo!();
    }

    fn help(&self) {
        println!("'To Rightly Divide' Help:");
        println!("'To Rightly Divide' is a Bible search tool integrated into your computer's CLI.\n");
        println!("It incorporates several different search options.");
        println!("\tThere is a general search. This is the default option. It will run all search options and return all results. The general search can also be explicitly requested using the '--search' tag");
        println!("\tThe subsets of search include an exact match search ('--exact'), a case indifferent search ('--caseless'), a substring search ('--substring'), and a reference search ('--reference')");
        println!("\t\tThe exact match search looks for scriptures containing an exact mactch, including case, to the input string provided.");
        println!("\t\tThe caseless search is similar to the exact match search except it does not consider case.");
        println!("\t\tThe substring search will look for matches of 70% or greater of the independent words in the provided string.");
        println!("\t\tThe reference search will see if the provided string is a match for bible reference.\n");
        println!("Here are some example queries to get you started:");
        println!(".\\to-rightly-divide.exe \"In the Beginning\"");
        println!(".\\to-rightly-divide.exe --exact \"In the Beginning\"");
        println!(".\\to-rightly-divide.exe --reference \"Genesis 1:1\"");
        println!(".\\to-rightly-divide.exe --reference \"gen 1:1\"\n");
        println!("Thank you for using 'To Rightly Divide'");
    }

    fn search(&self, bible: &Bible, method: SearchMethod, query: &str) {
        for verse in bible.search(method, query) {
            println!("{} {}:{} \"{}\"", verse.reference().book(), verse.reference().chapter(), verse.reference().verse(), verse.content());
        }
    }

    pub fn startup(&self, bible: &Bible, args: &Vec<String>) {
        if args.len() == 1 { self.launch(); }  // this will turn into the "lauch" option
        else if args.len() == 2 {
            match args[1].as_str() {
                "--help" => self.help(),
                _ => self.search(bible, SearchMethod::All, args[1].as_str()),
            }
        }
        else if args.len() == 3 {
            let method;
            match args[1].as_str() {
                "--caseless" => method = SearchMethod::Caseless,
                "--exact" => method = SearchMethod::Exact,
                "--reference" => method = SearchMethod::Reference,
                "--search" => method = SearchMethod::All,
                "--substring" => method = SearchMethod::Substring,
                _ => { 
                    println!("Invalid tag provided. Use tag '--help' for information regarding valid tags.");
                    return
                },
            }
            self.search(bible, method, args[2].as_str());
        }
        else { println!("Too many arguments provided. Max number of arguments is two."); }
    }
}

