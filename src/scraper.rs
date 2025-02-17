use crate::Result;
use serde_json::Value;
use std::fs;
use std::io::BufReader;

pub fn doi2json(doi: &str) -> Result<Value> {
    // TODO: Remove the debug file from here before shipping
    // I've set it up this way so that I don't keep querying crossref while developing
    eprintln!("TODO: REMOVE THE DEBUG FILE FROM doi2json BEFORE SHIPPING");
    eprintln!("doi2json doi: {:#?}", doi);
    let debug_filename: &str = if doi == "10.1016/j.drudis.2020.01.020" {
        "test_files/bibjson_response_article.json"
    } else if doi == "10.2514/6.2017-0836" {
        "test_files/bibjson_response_inproceedings.json"
    } else {
        panic!("We're in debug mode right now so this is not valid DOI")
    };
    println!("Trying to get the bibjson for doi {doi}");
    match fs::exists(debug_filename) {
        Ok(true) => {
            println!("Found the debug bibjson, so using it");
            let file = fs::File::open(debug_filename)?;
            let reader = BufReader::new(file);
            let response: Value = serde_json::from_reader(reader)?;
            return Ok(response.get("message").unwrap().clone());
        }
        Ok(false) => {
            // Create it
            let file = fs::File::create(debug_filename)?;
            let url = format!("http://api.crossref.org/works/{doi}");
            println!("Querying crossref...");
            let response: Value = reqwest::blocking::get(url)?.json()?;
            //dbg!(response);
            serde_json::to_writer(file, &response)?;
            return Ok(response.get("message").unwrap().clone());
        }
        Err(..) => {
            panic!("Something really went wrong - debug file could neither be confirmed nor denied")
        }
    }
}
