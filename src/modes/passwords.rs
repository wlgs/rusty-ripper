use std::fs::File;
use std::io::{self};
use csv::ReaderBuilder;
use crate::modes::ContentManager;

#[derive(Debug)]
pub struct Passwords {
    pub path: String,
    pub content: String,
    pub logins: Vec<String>,
    pub passwords: Vec<String>,
}

impl Passwords {
    pub fn new(path: String) -> Self {
        let mut passwords: Self = Self {
            path,
            content: String::new(),
            logins: Vec::new(),
            passwords: Vec::new(),
        };

        if passwords.load_content().is_ok() {
            if !passwords.validate() {
                 println!("Error: The number of logins and passwords do not match.");
            }
        } else {
            println!("Failed to load the file.");
        }

        passwords
    }

    pub fn load_content(&mut self) -> io::Result<()> {
        let file = File::open(&self.path)?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        for result in rdr.records() {
            let record = result?;
            if record.len() == 2 {
                self.logins.push(record[0].to_string());
                self.passwords.push(record[1].to_string());
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> bool {
        !self.logins.is_empty() && self.logins.len() == self.passwords.len()
    }
}

impl ContentManager for Passwords {
    fn display(&self) {
        if self.passwords.is_empty() {
            println!("The CSV file could not be loaded due to wrong formatting.");
        } else {
            println!("Password-login pairs loaded successfully with {} pairs.", self.passwords.len());
        }
    }
}

