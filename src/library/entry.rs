use crate::{Error, Result};
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::scraper::doi2json;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BibEntry {
    pub title: String,

    //TODO: Figure out how to format these so that they show up in the same table
    #[serde(
        serialize_with = "serialize_author_list",
        deserialize_with = "deserialize_author_list",
        flatten
    )]
    pub authors: Vec<Author>,

    pub year: u32,
    pub read_status: bool,
    pub tags: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub projects: Option<Vec<String>>,
    pub reading_folders: Option<Vec<String>>,
    pub doi: String,
    pub url: Option<String>,
    pub date_added: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,

    #[serde(flatten)]
    pub entry_type: BibType,

    pub file_type: Option<String>, // The extension of the associated file
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Author {
    pub first: String,
    pub last: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum BibType {
    #[serde(rename = "article")]
    Article {
        journal: String,
        volume: Option<u16>,
        number: Option<String>,
        pages: Option<String>,
        publisher: Option<String>,
        month: Option<u32>,
    },
    #[serde(rename = "inproceedings")]
    InProceedings {
        booktitle: String,
        publisher: Option<String>,
        month: Option<u32>,
    },
    //Book {
    //    publisher: Option<String>,
    //    issn: Option<String>,
    //    isbn: Option<String>,
    //},
}

fn serialize_author_list<S>(
    authors: &Vec<Author>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = HashMap::new();
    for (i, author) in authors.iter().enumerate() {
        map.insert(format!("person_{}_first_name", i + 1), &author.first);
        map.insert(format!("person_{}_last_name", i + 1), &author.last);
    }
    map.serialize(serializer)
}

fn deserialize_author_list<'de, D>(deserializer: D) -> std::result::Result<Vec<Author>, D::Error>
where
    D: Deserializer<'de>,
{
    // Thank you ChatGPT
    let raw_map: HashMap<String, toml::Value> = HashMap::deserialize(deserializer)?;
    let mut authors = Vec::new();
    let mut i = 1;
    loop {
        let first_name_key = format!("person_{}_first_name", i);
        let last_name_key = format!("person_{}_last_name", i);

        if let (Some(toml::Value::String(first_name)), Some(toml::Value::String(last_name))) =
            (raw_map.get(&first_name_key), raw_map.get(&last_name_key))
        {
            authors.push(Author {
                first: first_name.clone(),
                last: last_name.clone(),
            });
            i += 1;
        } else {
            break;
        }
    }
    Ok(authors)
}

impl BibType {
    fn variants() -> Vec<String> {
        vec!["Article".to_string(), "InProceedings".to_string()]
    }

    fn new(entry_type: &str) -> BibType {
        match entry_type {
            "Article" => BibType::Article {
                journal: "".to_string(),
                volume: None,
                number: None,
                pages: None,
                publisher: None,
                month: None,
            },
            "InProceedings" => BibType::InProceedings {
                booktitle: "".to_string(),
                publisher: None,
                month: None,
            },
            s => panic!("Don't know how to create defaults for {s}"),
        }
    }
}

impl BibEntry {
    pub fn new(entry_type: Option<String>) -> Result<BibEntry> {
        // Create a new blank entry
        eprintln!("===============");
        dbg!(&entry_type);
        let entry_type = match entry_type {
            Some(entry_type) => entry_type,
            None => inquire::Select::new(
                "What type of entry would you like to create?",
                BibType::variants(),
            )
            .prompt()?,
        };
        let now = Utc::now();
        let now = Utc
            .with_ymd_and_hms(
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
            )
            .unwrap();
        Ok(BibEntry {
            title: "".to_string(),
            authors: vec![],
            year: 0,
            read_status: false,
            tags: None,
            keywords: None,
            projects: None,
            reading_folders: None,
            doi: "".to_string(),
            url: None,
            date_added: now,
            date_updated: now,
            entry_type: BibType::new(&entry_type),
            file_type: None,
        })
    }

    pub fn from_doi(doi: &str) -> Result<BibEntry> {
        let json = doi2json(doi)?;
        BibEntry::from_bibjson(json)
    }

    pub fn from_bibjson(json: Value) -> Result<BibEntry> {
        // Figure out what type of entry this is and dispatch to the appropriate method
        //eprintln!("{:#?}", json);
        let entry_type = match json["type"].as_str() {
            Some(entry_type) => entry_type,
            None => return Err(Error::MissingBibType),
        };
        // Build and return the appropriate entry
        match entry_type {
            "journal-article" => BibEntry::article_from_bibjson(json),
            "proceedings-article" => BibEntry::inproceedings_from_bibjson(json),
            t => Err(Error::UnknownBibType {
                bib_type: t.to_string(),
            }),
        }
    }

    pub fn edit(&self) -> Result<BibEntry> {
        let original_toml_string = toml::to_string_pretty(&self)?;
        loop {
            let edited_toml_string =
                inquire::Editor::new("Please edit the entry and save the temporary file")
                    .with_file_extension(".toml")
                    .with_predefined_text(&original_toml_string)
                    .prompt()?;
            match toml::from_str(&edited_toml_string) {
                Ok(new_entry) => {
                    return Ok(new_entry);
                }
                Err(e) => {
                    eprintln!("There was an error: {e}");
                }
            };
        }
    }

    pub fn get_type(&self) -> String {
        match self.entry_type {
            BibType::Article { .. } => "article".to_string(),
            BibType::InProceedings { .. } => "inproceedings".to_string(),
        }
    }

    fn article_from_bibjson(json: Value) -> Result<BibEntry> {
        let now = Utc::now();
        let now = Utc
            .with_ymd_and_hms(
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
            )
            .unwrap();
        Ok(BibEntry {
            title: json["title"][0].as_str().unwrap().to_string(),
            authors: json["author"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|v| Author {
                            first: v["given"].as_str().unwrap().to_string(),
                            last: v["family"].as_str().unwrap().to_string(),
                        })
                        .collect()
                })
                .unwrap_or_else(Vec::new),
            year: json["issued"]["date-parts"][0].as_array().unwrap()[0]
                .as_u64()
                .unwrap() as u32,
            read_status: false,
            tags: None,
            keywords: None,
            projects: None,
            reading_folders: None,
            doi: json["DOI"].as_str().unwrap().to_string(),
            url: match json.get("URL") {
                Some(url) => Some(url.as_str().unwrap().to_string()),
                None => None,
            },
            date_added: now,
            date_updated: now,
            entry_type: BibType::Article {
                journal: match json["container-title"][0].as_str() {
                    Some(journal) => journal.to_string(),
                    None => String::from(""),
                },
                month: Some(
                    json["issued"]["date-parts"][0].as_array().unwrap()[1]
                        .as_u64()
                        .unwrap() as u32,
                ),
                volume: match json["volume"].as_str() {
                    Some(volume) => Some(volume.parse::<u16>().unwrap()),
                    None => None,
                },
                number: match json["number"].as_str() {
                    Some(number) => Some(number.to_string()),
                    None => None,
                },
                pages: match json["page"].as_str() {
                    Some(pages) => Some(pages.to_string()),
                    None => None,
                },
                publisher: match json["publisher"].as_str() {
                    Some(publisher) => Some(publisher.to_string()),
                    None => None,
                },
            },
            file_type: None,
        })
    }

    fn inproceedings_from_bibjson(json: Value) -> Result<BibEntry> {
        let now = Utc::now();
        let now = Utc
            .with_ymd_and_hms(
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
            )
            .unwrap();
        Ok(BibEntry {
            title: json["title"][0].as_str().unwrap().to_string(),
            authors: json["author"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|v| Author {
                            first: v["given"].as_str().unwrap().to_string(),
                            last: v["family"].as_str().unwrap().to_string(),
                        })
                        .collect()
                })
                .unwrap_or_else(Vec::new),
            year: json["issued"]["date-parts"][0].as_array().unwrap()[0]
                .as_u64()
                .unwrap() as u32,
            read_status: false,
            tags: None,
            keywords: None,
            projects: None,
            reading_folders: None,
            doi: json["DOI"].as_str().unwrap().to_string(),
            url: match json.get("URL") {
                Some(url) => Some(url.as_str().unwrap().to_string()),
                None => None,
            },
            date_added: now,
            date_updated: now,
            entry_type: BibType::InProceedings {
                booktitle: json["container-title"][0].as_str().unwrap().to_string(),
                publisher: match json["publisher"].as_str() {
                    Some(publisher) => Some(publisher.to_string()),
                    None => None,
                },
                month: Some(
                    json["issued"]["date-parts"][0].as_array().unwrap()[1]
                        .as_u64()
                        .unwrap() as u32,
                ),
            },
            file_type: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::BufReader;

    #[test]
    fn can_create_blank_article() {
        let new_entry = BibEntry::new(Some("Article".to_string())).unwrap();
        let now = Utc::now();
        let now = Utc
            .with_ymd_and_hms(
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
            )
            .unwrap();
        let expected_entry = BibEntry {
            title: "".to_string(),
            authors: vec![],
            year: 0,
            read_status: false,
            tags: None,
            keywords: None,
            projects: None,
            reading_folders: None,
            doi: "".to_string(),
            url: None,
            date_added: now,
            date_updated: now,
            entry_type: BibType::Article {
                journal: "".to_string(),
                volume: None,
                number: None,
                pages: None,
                publisher: None,
                month: None,
            },
            file_type: None,
        };
        assert_eq!(new_entry, expected_entry);
    }

    #[test]
    fn can_create_blank_inproceedings() {
        let new_entry = BibEntry::new(Some("InProceedings".to_string())).unwrap();
        let now = Utc::now();
        let now = Utc
            .with_ymd_and_hms(
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
            )
            .unwrap();
        let expected_entry = BibEntry {
            title: "".to_string(),
            authors: vec![],
            year: 0,
            read_status: false,
            tags: None,
            keywords: None,
            projects: None,
            reading_folders: None,
            doi: "".to_string(),
            url: None,
            date_added: now,
            date_updated: now,
            entry_type: BibType::InProceedings {
                booktitle: "".to_string(),
                publisher: None,
                month: None,
            },
            file_type: None,
        };
        assert_eq!(new_entry, expected_entry);
    }
}
