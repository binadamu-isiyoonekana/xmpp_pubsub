//! Xmpp data types

use quick_xml::{events::BytesStart, Reader};

use std::fmt;
use std::str::FromStr;
use strum::EnumString;

use crate::errors::{Error, Result};

/// Data form's field element
#[derive(Clone)]
pub struct Field {
    pub var: String,
    pub type_: FieldType,
    pub label: Option<String>,
    pub required: bool,
    pub options: Vec<Option_>,
    pub values: Vec<String>,
    pub media: Vec<MediaElement>,
}

impl fmt::Debug for Field {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Field")
            .field("var", &self.var)
            .field("type_", &self.type_)
            .field("required", &self.required)
            .field("options", &self.options)
            .field("values", &self.values)
            .field("media", &self.media)
            .finish()
    }
}

impl Default for Field {
    fn default() -> Self {
        Self {
            var: Default::default(),
            type_: FieldType::Fixed,
            label: Default::default(),
            required: Default::default(),
            options: Default::default(),
            values: Default::default(),
            media: Default::default(),
        }
    }
}

impl Field {
    /// Builds a new field data type from a given xml description
    pub fn from_element(element: &BytesStart, reader: &mut Reader<&[u8]>) -> Result<Self> {
        // Create a default field instance
        let field = &mut Field::default();

        // Parse all attributes of the field element
        for attribute in element.attributes() {
            match attribute {
                Ok(attribute) => match attribute.key.as_ref() {
                    b"var" => {
                        let value = attribute
                            .decode_and_unescape_value(reader)
                            .or_else(|error| Err(Error::XmlParsingError(error)))
                            .unwrap()
                            .to_string();

                        field.var = value;
                    }

                    b"type" => {
                        let value = attribute
                            .decode_and_unescape_value(reader)
                            .or_else(|error| Err(Error::XmlParsingError(error)))
                            .unwrap();

                        field.type_ = FieldType::from_str(value.as_ref())
                            .or_else(|error| Err(Error::EnumVariantParsingError(error)))
                            .unwrap();
                    }

                    b"label" => {}

                    b"required" => {}

                    b"options" => {}

                    b"values" => {}

                    b"media" => {}

                    // Should not reach this branch
                    _ => (),
                },

                Err(error) => {
                    return Err(Error::XmlParsingError(error.into()));
                }
            }
        }

        // Read field's value sub-elements

        Ok(field.to_owned())
    }
}

/// Represents a of the possible values for a list field.
#[derive(Clone, Debug, Default)]
pub struct Option_ {
    pub label: Option<String>,
    pub value: String,
}

/// Types of data form field
#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum FieldType {
    #[strum(serialize = "boolean")]
    Boolean,

    #[strum(serialize = "fixed")]
    Fixed,

    #[strum(serialize = "hidden")]
    Hidden,

    #[strum(serialize = "jid-multi")]
    JidMulti,

    #[strum(serialize = "jid-single")]
    JidSingle,

    #[strum(serialize = "list-multi")]
    ListMulti,

    #[strum(serialize = "list-single")]
    ListSingle,

    #[strum(serialize = "text-multi")]
    TextMulti,

    #[strum(serialize = "text-private")]
    TextPrivate,

    #[strum(serialize = "text-single")]
    TextSingle,
}

/// Media element
#[derive(Clone, Debug, Default)]
pub struct MediaElement {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub uris: Vec<URI>,
}

/// URI media element
#[derive(Clone, Debug, Default)]
pub struct URI {
    pub type_: String,
    pub uri: String,
}
