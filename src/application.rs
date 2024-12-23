use crate::{bible::Bible, ui::cli::CLI};

pub struct Application {
    bible: Bible,
    cli: CLI,
}

impl Application {
    pub fn new_kjv() -> Self {
        Self {
            bible: Bible::new_from_file("kjv", "./KJV.bible"),
            cli: CLI(),
        }
    }

    pub fn process_arguments(&self, args: &Vec<String>) {
        self.cli.startup(&self.bible, args);
    }
}