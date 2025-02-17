use crate::{Error, Result};
use deunicode::deunicode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub mod entry;
use crate::library::entry::BibEntry;

mod pdfhandlers;
use pdfhandlers::pdf2doi;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    db: PathBuf,
    docs: PathBuf,
    notes: PathBuf,
}

#[derive(Debug)]
pub struct Library {
    entries: HashMap<String, BibEntry>,
    config: Config,
}

impl Library {
    pub fn new() -> Self {
        // TODO: Load the paths from some config
        Library {
            entries: HashMap::new(),
            config: Config {
                db: PathBuf::from("./testlib/db.toml"),
                docs: PathBuf::from("./testlib/docs"),
                notes: PathBuf::from("./testlib/notes"),
            },
        }
    }

    pub fn load(&mut self) -> Result<()> {
        let toml_string = fs::read_to_string(&self.config.db)?;
        self.entries = toml::from_str(&toml_string)?;
        Ok(())
    }
}

impl Library {
    // Methods pertaining to files

    pub fn add_file(&mut self, filepath: &Path, delete_after: bool) -> Result<String> {
        match delete_after {
            true => eprintln!("Adding file {filepath:#?} and deleting it..."),
            false => eprintln!("Adding file {filepath:#?} but not deleting it..."),
        }
        eprintln!("WARNING: addfile is not fully implemented yet!");

        // Set up the file preview
        let mut child = Command::new("zathura").arg(filepath).spawn()?;

        // Try to extract the DOI
        // TODO: Do we want to make the user aware about Error::CommandFailed?
        let doi = pdf2doi(filepath).ok();

        let key = self.add_entry(doi)?;
        if let Some(entry) = self.entries.get_mut(&key) {
            // Update the entry with the file_type / extension
            entry.file_type = Some(
                filepath
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|s| s.to_string())
                    .unwrap(),
            );
        } else {
            panic!("The key {key} was just inserted so should have been found!");
        }
        self.flush()?; // Flush to disk since we just updated the file_type

        // Copy the file to the appropriate location
        // Check if the file already exists before overwriting it
        let dest = self.get_uri(&key).unwrap();
        if fs::exists(&dest)? {
            return Err("Destination file {dest} already exists".into());
        }
        match fs::copy(&filepath, &dest) {
            Ok(_) => eprintln!("Copied file to library"),
            Err(e) => panic!("Failed to copy file to library. Error: {}", e),
        };

        // Delete original file if requested
        if delete_after {
            match fs::remove_file(filepath) {
                Ok(_) => {
                    eprintln!("Deleted original file");
                }
                Err(_) => {
                    eprintln!("Failed to delete original file");
                }
            }
        }

        // Close the file preview
        child.kill()?;

        Ok(key)
    }

    pub fn get_uri(&self, key: &str) -> Result<PathBuf> {
        let underscored_key = key.replace("-", "_");
        let extension: &str = self.entries[key].file_type.as_deref().unwrap();
        let entry_type = &self.entries[key].get_type();
        let mut filepath = self.config.docs.clone();
        filepath.push(entry_type);
        filepath.push(underscored_key);
        filepath.set_extension(extension);
        Ok(filepath)
    }
}

impl Library {
    // Methods for adding entries

    pub fn add_entry(&mut self, doi: Option<String>) -> Result<String> {
        let entry = match doi {
            Some(doi) => {
                // Check that the DOI doesn't already exist
                if let Some(_matched_entry) = self.entries.values().find(|e| e.doi == doi) {
                    eprintln!("Already found in database!");
                    eprintln!("We don't have logic for it here but hopefully it gets caught in the key generation step!");
                }
                BibEntry::from_doi(&doi)?
            }
            None => {
                let choices = vec![
                    "Enter DOI",
                    "Provide search terms",
                    "Enter information manually",
                ];
                match inquire::Select::new("Choose an option:", choices).prompt()? {
                    "Enter DOI" => {
                        BibEntry::from_doi(&inquire::Text::new("Enter DOI: ").prompt()?)?
                    }
                    "Provide search terms" => todo!("Not implemented yet"),
                    "Enter information manually" => {
                        let mut entry = BibEntry::new(None)?;
                        entry = entry.edit()?;
                        entry
                    }
                    s => panic!("The input '{s}' is not allowed"),
                }
            }
        };

        let (key, entry) = self.confirm_entry(entry)?;
        // TODO: Think about if we can get rid of this assert? Does it make sense to allow updates
        // through this mechanism (even though, as far as I understand, that's theoretically not
        // possible)?
        assert!(!self.entries.contains_key(&key));
        self.entries.insert(key.clone(), entry);
        self.flush()?;
        Ok(key)
    }

    fn confirm_entry(&self, entry: BibEntry) -> Result<(String, BibEntry)> {
        let key = self.create_key(&entry);
        let choices = vec!["Continue", "Edit", "Reset", "Quit"];
        let mut verified_entry = entry.clone(); // Keep the original entry in case we want to reset
        loop {
            eprintln!("Current entry:");
            eprintln!("{verified_entry:#?}");
            match inquire::Select::new("Does this look correct?", choices.clone()).prompt()? {
                "Continue" => return Ok((key, verified_entry)),
                "Edit" => verified_entry = verified_entry.edit()?,
                "Reset" => verified_entry = entry.clone(),
                "Quit" => return Err(Error::UserQuit),
                s => panic!("The input '{s}' is not allowed"),
            };
        }
    }

    fn _key_generator(first_name: &str, last_name: &str, year: u32) -> String {
        let first_char = deunicode(&first_name)
            .to_lowercase()
            .chars()
            .next()
            .unwrap();
        let cleaned_last_name = deunicode(&last_name)
            .to_lowercase()
            .replace("-", "")
            .replace(" ", "");
        format!("{}{}-{}", first_char, cleaned_last_name, year).to_string()
    }

    fn create_key(&self, entry: &BibEntry) -> String {
        // TODO: Make this configurable
        // TODO: Allow the user to override the key maybe?
        let first_author = &entry.authors[0];
        let year = entry.year;
        let mut key = Library::_key_generator(&first_author.first, &first_author.last, year);
        assert!(!key.is_empty());

        // Check if this is already in the database and get the user to confirm that it
        // doesn't match with any of them
        let similar_keys: Vec<&str> = self
            .entries
            .keys()
            .filter(|k| k.starts_with(&key))
            .map(|k| k.as_str())
            .collect();
        for matched_key in similar_keys {
            eprintln!("Found some matches. Do any of the following records match?");
            let matched_entry = &self.entries[matched_key];
            eprintln!("{matched_entry:#?}");
            loop {
                match inquire::Confirm::new("Does this match?").prompt() {
                    Ok(true) => todo!("What do we do if it matches?"),
                    Ok(false) => break,
                    Err(_) => continue,
                }
            }
        }

        // Ensure that the key is unique
        if self.entries.contains_key(&key) {
            key.push('a');
            while self.entries.contains_key(&key) {
                let c = key.pop().unwrap();
                key.push((c as u8 + 1) as char);
            }
        }

        key
    }

    fn flush(&self) -> Result<()> {
        let toml_string = toml::to_string_pretty(&self.entries)?;
        fs::write(&self.config.db, toml_string)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_key_simple() {
        let first_name = "John";
        let last_name = "Doe";
        let year = 2019;
        assert_eq!(
            Library::_key_generator(first_name, last_name, year),
            "jdoe-2019"
        );
    }

    #[test]
    fn creates_key_without_accents() {
        let first_name = "John";
        let last_name = "Doè";
        let year = 2019;
        assert_eq!(
            Library::_key_generator(first_name, last_name, year),
            "jdoe-2019"
        );
    }

    #[test]
    fn crates_key_without_dashes() {
        let first_name = "John";
        let last_name = "Le-Doè";
        let year = 2019;
        assert_eq!(
            Library::_key_generator(first_name, last_name, year),
            "jledoe-2019"
        );
    }

    #[test]
    fn creates_key_without_spaces() {
        let first_name = "John";
        let last_name = "Le Doè";
        let year = 2019;
        assert_eq!(
            Library::_key_generator(first_name, last_name, year),
            "jledoe-2019"
        );
    }
}
