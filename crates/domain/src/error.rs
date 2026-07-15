use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid mjlog XML: {0}")]
    Xml(#[from] roxmltree::Error),
    #[error("missing {attribute} attribute on {tag}")]
    MissingAttribute { tag: String, attribute: String },
    #[error("invalid {attribute} attribute on {tag}: {value}")]
    InvalidAttribute {
        tag: String,
        attribute: String,
        value: String,
    },
    #[error("missing mjlog {0}")]
    MissingMetadata(&'static str),
    #[error("kyoku {round} has no result")]
    IncompleteKyoku { round: u8 },
    #[error("kyoku {round} contains conflicting results")]
    ConflictingResult { round: u8 },
}

pub type Result<T> = std::result::Result<T, DomainError>;
