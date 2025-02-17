use crate::{Error, Result};
use regex::Regex;
use std::path::Path;
use std::process::Command;

#[allow(non_snake_case)]
fn _pdf2doi_extractDOI(stdout: String) -> Option<String> {
    let re = Regex::new(r"doi:\s*(?<doi>10\.\d{4,9}\/[-._\/:A-Za-z0-9]+)").unwrap();
    for line in stdout.lines() {
        let line = line.trim().to_lowercase();
        if line.contains("doi") {
            //println!("> {line}");
            let caps = re.captures(&line).unwrap();
            //println!("The doi number is: {}", &caps["doi"]);
            return Some(String::from(&caps["doi"]));
        }
    }

    None
}

fn _pdf2doi_pdfinfo(filepath: &Path) -> Result<String> {
    let output = match Command::new("pdfinfo").arg(filepath).output() {
        Ok(output) => output,
        Err(_) => {
            return Err(Error::CommandFailed {
                cmd: "pdfinfo".to_string(),
            })
        }
    };

    if !output.status.success() {
        return Err(Error::CommandFailed {
            cmd: "pdfinfo".to_string(),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    _pdf2doi_extractDOI(stdout).ok_or(Error::DOINotFound)
}

fn _pdf2doi_pdftotext(filepath: &Path) -> Result<String> {
    let output = match Command::new("pdftotext")
        .arg(filepath)
        .arg("-") // direct the output to stdout
        .output()
    {
        Ok(output) => output,
        Err(_) => {
            return Err(Error::CommandFailed {
                cmd: "pdftotext".to_string(),
            })
        }
    };

    if !output.status.success() {
        return Err(Error::CommandFailed {
            cmd: "pdftotext".to_string(),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    _pdf2doi_extractDOI(stdout).ok_or(Error::DOINotFound)
}

pub fn pdf2doi(filepath: &Path) -> Result<String> {
    _pdf2doi_pdfinfo(filepath).or_else(|_| _pdf2doi_pdftotext(filepath))
}
