/// Enumerates errors that can occur when processing configuration settings.
///
/// The following errors represent all problems that could arise while
/// working with configuration files
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Catch up errors occuring in quick-xml parser
    #[error(transparent)]
    XmlParsingError(#[from] quick_xml::Error),

    /// Catch up errors occuring in strum tool
    #[error(transparent)]
    EnumVariantParsingError(#[from] strum::ParseError),

    /// Cannot find XML input file
    #[error("Cannot find input file {0}")]
    CannotFindInputFile(String),

    /// Invalid Xmpp field element attribute
    #[error("Attribute {0} is not supported by field element")]
    UnknownFieldElementAttribute(String),

    /// Expecting a value sub-element in a field element
    #[error("A 'value' element is expected for field element, but {0} was found")]
    ExpectingFieldValueElement(String),

    /// Unexpected end of file
    #[error("Reach unexpected end of file")]
    UnexpectedEndOfFile()
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
