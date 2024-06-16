//! Xmpp data types

use quick_xml::{
    events::{BytesStart, Event},
    name::QName,
    Reader,
};

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

        // Parse field element attributes
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

                    // Skip over these attributes (not of interest for us now)
                    b"label" => {}
                    b"required" => {}
                    b"options" => {}
                    b"values" => {}
                    b"media" => {}

                    // Invalid field element attribute
                    _ => {
                        return Err(Error::UnknownFieldElementAttribute(
                            String::from_utf8(attribute.key.as_ref().to_vec()).unwrap(),
                        ))
                    }
                },

                Err(error) => {
                    return Err(Error::XmlParsingError(error.into()));
                }
            }
        }

        // Parse field values
        //
        // A field element can contain zero or more '<value>' sub-elements, as shown in the following XML snippet
        // where a 'text-multi' field type contains several lines:
        //
        // ...
        // <x xmlns='jabber:x:data' type='submit'>
        //   <field type='text-multi' var='description'>
        //     <value>First text line</value>
        //     <value>Second text line</value>
        //     <value>Third text line</value>
        //   </field>
        // </x>
        // ...
        let mut buffer = Vec::new();

        loop {
            let event = reader.read_event_into(&mut buffer);

            match event {
                Ok(Event::Start(ref element)) => match element.name() {
                    // Expecting a '<value>' sub-element
                    QName(b"value") => {
                        let value = reader.read_text(element.name())?;
                        field.values.push(value.as_ref().into());
                    }

                    // Exit the loop when and unexpected child element for a field element
                    _ => {
                        let element_name = element.name();
                        println!("  => Element: {:?}", element_name);

                        break;
                        /*
                        let element_name = element.name().as_ref();
                        let unexpected_element = reader.decoder().decode(element_name().into()).unwrap();

                        return Err(Error::ExpectingFieldValueElement(unexpected_element.into()));
                        */
                    }
                },

                // End of field element (i.e. '</field>' tag is reached)
                Ok(Event::End(ref element)) => match element.name() {
                    QName(b"field") => break,

                    _ => {
                        dbg!(
                            "Weird end event triggered in field element at position: {}",
                            reader.buffer_position()
                        );
                    }
                },

                Err(error) => return Err(Error::XmlParsingError(error)),

                // Unexpected start event (only start events for '<value>' elements are awaited)
                _ => {
                    dbg!(
                        "Weird start event triggered in field element at position: {}",
                        reader.buffer_position()
                    );
                }
            }

            buffer.clear();
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
