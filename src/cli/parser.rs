use clap::Parser;
use clap::CommandFactory;

use crate::modes::ContentManager;
use crate::modes::dictionary::Dictionary;
use crate::modes::hasher::{Hasher, HashFunction};
use crate::modes::passwords::Passwords;
use crate::modes::retriver::Retriver;

#[derive(Debug)]
#[derive(Parser)]
pub struct CLI {
    #[arg(short, long)]
    pub dictionary: Option<String>, 

    #[arg(long)]
    pub hash: Option<String>,

    #[arg(short, long)]
    pub passwords: Option<String>,
}

impl CLI {
    pub fn run_parser() {
        let args = Self::parse();

        let dictionary_flag = args.dictionary.is_some();
        let hash_flag = args.hash.is_some();
        let passwords_flag = args.passwords.is_some();

        if dictionary_flag && !hash_flag && !passwords_flag {
            if let Some(ref dictionary_path) = args.dictionary {
                let mut dictionary = Dictionary::new(dictionary_path.clone());
                dictionary.load();
                dictionary.display();
                return;
            }
        }

        if hash_flag && !dictionary_flag && !passwords_flag {
            if let Some(ref hash_function) = args.hash {
                if let Some(hash_fn_enum) = HashFunction::from_str(hash_function) {
                    let dummy_dictionary = Dictionary::new(String::new()); 
                    let mut hasher = Hasher::new(dummy_dictionary, hash_fn_enum);
                    hasher.load();
                    hasher.display();
                    return;
                } else {
                    println!("Unsupported hash function: {}", hash_function);
                    return;
                }
            }
        }

        if passwords_flag && !dictionary_flag && !hash_flag {
            if let Some(ref passwords_path) = args.passwords {
                let mut passwords = Passwords::new(passwords_path.clone());
                passwords.load();
                passwords.display();
                return;
            }
        }

        if dictionary_flag && hash_flag && passwords_flag {
            if let (Some(ref dictionary_path), Some(ref hash_function), Some(ref passwords_path)) =
                (args.dictionary, args.hash, args.passwords)
            {
                let mut dictionary = Dictionary::new(dictionary_path.clone());
                dictionary.load();

                let mut passwords = Passwords::new(passwords_path.clone());
                passwords.load();

                if let Some(hash_fn_enum) = HashFunction::from_str(hash_function) {
                    let mut hasher = Hasher::new(dictionary, hash_fn_enum);
                    hasher.load();

                    let retriver = Retriver::new(hasher, passwords);
                    retriver.run();
                    return;
                } else {
                    println!("Unsupported hash function: {}", hash_function);
                    return;
                }
            }
        }

        println!("Error: Wrong flags combination.");
        CLI::command().print_help().unwrap();
        println!(); 
    }
}

