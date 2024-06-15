/// Xmmp pubsub data types definition
use std::{borrow::Cow, convert::Infallible, fmt};

use crate::errors::{Error, Result};
use quick_xml::{events::BytesStart, Reader};

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
        // Instantiate a default field object
        let field = &mut Field::default();

        // Parse all attributes of the field element
        for attribute in element.attributes() {
            match attribute {
                Ok(attribute) => match attribute.key.as_ref() {
                    b"var" => {
                        field.var = attribute
                            .decode_and_unescape_value(reader)
                            .or_else(|error| {
                                dbg!("unable to read 'var' field attribute: {:?}", error);
                                Ok::<Cow<'_, str>, Infallible>(std::borrow::Cow::from(""))
                            })
                            .unwrap()
                            .to_string();
                    }

                    b"type_" => {
                        field.var = attribute
                            .decode_and_unescape_value(reader)
                            .or_else(|error| {
                                dbg!("unable to read 'type_' field attribute, {:?}", error);
                                Ok::<Cow<'_, str>, Infallible>(std::borrow::Cow::from(""))
                            })
                            .unwrap()
                            .to_string();
                    }

                    b"label" => {
                        field.var = attribute
                            .decode_and_unescape_value(reader)
                            .or_else(|error| {
                                dbg!("unable to read 'label' field attribute {:?}", error);
                                Ok::<Cow<'_, str>, Infallible>(std::borrow::Cow::from(""))
                            })
                            .unwrap()
                            .to_string();
                    }

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
#[derive(Clone, Debug)]
pub enum FieldType {
    Boolean,
    Fixed,
    Hidden,
    JidMulti,
    JidSingle,
    ListMulti,
    ListSingle,
    TextMulti,
    TextPrivate,
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
