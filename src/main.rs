use std::env;

use to_rightly_divide::bible::Bible;

/*
try: cargo run -- --reference "Genesis 1:1" (example)
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let b = Bible::new_from_file("kjv", "./KJV.bible");
    //for verse in b.search("In the beginning") { println!("{} {}:{} \"{}\"", verse.reference().book(), verse.reference().chapter(), verse.reference().verse(), verse.content()); }
    //for verse in b.search("Genesis 1:1") { println!("{} {}:{} \"{}\"", verse.reference().book(), verse.reference().chapter(), verse.reference().verse(), verse.content()); }

    if args.len() == 1 { println!("No arguments provided."); }  // this will turn into the "lauch" option
    else if args.len() == 2 {
        match args[1].as_str() {
            "--help" => {
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
            },
            _ => {
                for verse in b.search(args[1].as_str()) {
                    println!("{} {}:{} \"{}\"", verse.reference().book(), verse.reference().chapter(), verse.reference().verse(), verse.content());
                }
            }
        }
    }
    else if args.len() == 3 {
        match args[1].as_str() {
            "--caseless" => {
                for verse in b.caseless_search(args[2].as_str()) {
                    println!("{} {}:{} \"{}\"", verse.reference().book(), verse.reference().chapter(), verse.reference().verse(), verse.content());
                }
            },
            "--exact" => {
                for verse in b.exact_search(args[2].as_str()) {
                    println!("{} {}:{} \"{}\"", verse.reference().book(), verse.reference().chapter(), verse.reference().verse(), verse.content());
                }
            },
            "--reference" => {
                for verse in b.reference_search(args[2].as_str()) {
                    println!("{} {}:{} \"{}\"", verse.reference().book(), verse.reference().chapter(), verse.reference().verse(), verse.content());
                }
            },
            "--search" => {
                for verse in b.search(args[2].as_str()) {
                    println!("{} {}:{} \"{}\"", verse.reference().book(), verse.reference().chapter(), verse.reference().verse(), verse.content());
                }
            },
            "--substring" => {
                for verse in b.substring_search(args[2].as_str()) {
                    println!("{} {}:{} \"{}\"", verse.reference().book(), verse.reference().chapter(), verse.reference().verse(), verse.content());
                }
            },
            _ => { println!("Invalid tag provided. Use tag '--help' for information regarding valid tags."); },
        }
    }
    else { println!("Too many arguments provided. Max number of arguments is two."); }

    
}

/*
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
*/
