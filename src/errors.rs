/// Enumerates errors that can occur when processing configuration settings.
///
/// The following errors represent all problems that could arise while
/// working with configuration files
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Catch up errors occuring in quick-xml parser
    #[error(transparent)]
    XmlParsingError(#[from] quick_xml::Error),

    /// Cannot find XML input file
    #[error("Cannot find input file {0}")]
    CannotFindInputFile(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
