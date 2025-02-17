// The idea from this comes from Jeremy Chone's video at https://www.youtube.com/watch?v=j-VQCYP7wyw
use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;
//pub type Error = Box<dyn std::error::Error>; // for early dev

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    // PDF Handler Errors
    CommandFailed {
        cmd: String,
    },
    DOINotFound,

    // BibErrors
    UnknownBibType {
        bib_type: String,
    },
    MissingBibType,

    UserQuit,

    // Externals
    #[from]
    IoError(std::io::Error),
    #[from]
    ReqwestError(reqwest::Error),
    #[from]
    DeserializeError(serde_json::Error),
    #[from]
    TOMLSerError(toml::ser::Error),
    #[from]
    TOMLDeError(toml::de::Error),
    #[from]
    InquireError(inquire::InquireError),
}

impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}

// Error boilerplate

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
