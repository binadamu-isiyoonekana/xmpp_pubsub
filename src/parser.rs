use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;

use crate::Field;
use crate::errors::{Error, Result};


/// Fast Xmpp pubsub protocol parser
///
/// This fast and lean reader processes events triggered by the embedded [quick_xml::reader::Reader] asynchronously and without
/// excessive data movements.
/// It is worth pointing out that this tiny parser aims at extractinf [Field] elements only, by calling the [XmppPubSubParser::parse_fields]
/// methods. Of course, it can easily be extended.
///
/// ## Examples
///
/// ```rust
/// use xmpp_lib::constants::XMPP_PUBSUB_PUBLISH_EMPTY as Slice;
///
/// let mut parser = XmppPubSubParser::from_str(Slice);
///
/// parser.parse()
/// ``````
pub struct XmppPubSubParser<R> {
    /// Inner data source reader
    reader: Reader<R>,
}

// Implement associated functions for the Xmpp reader
impl<R> XmppPubSubParser<R> {
    /// Builds a new Xmpp parser instance given a data source reader
    ///
    /// Currently, this Xmpp reader supports string slice or file contents.
    pub fn from_reader(reader: Reader<R>) -> Self {
        Self { reader }
    }
}

/// Implement a flavor of Xmpp pubsub parser for string slice
///
/// This parser is an implementation for reading from a `&[u8]` as underlying byte stream.
impl<'a> XmppPubSubParser<&'a [u8]> {
    /// Builds an Xmpp parser from a given input string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(slice: &'a str) -> Self {
        Self::from_reader(Reader::from_str(slice))
    }

    /// Parses field elements and triggers events
    ///
    /// This parsing process is only parsing field elements of Xmpp pubsub data form, as this is what interest us for now.
    /// Of course, other Xmpp pubsub elements can be parsed in the future if needed.
    pub async fn parse_fields(&mut self) -> Result<Vec<Field>> {
        // Read event into a buffer for faster element processing
        let mut buffer = Vec::new();

        // Create a hash map where to store requested fields
        let mut fields: Vec<Field> = Vec::new();

        loop {
            let event = self.reader.read_event_into_async(&mut buffer).await;

            match event {
                Ok(Event::Start(ref element)) => match element.name() {
                    // Parse field element
                    QName(b"field") => {
                        let field: Field = Field::from_element(element, &mut self.reader)?;
                        fields.push(field);
                    }

                    // Skip over other elements (only field element is of interest for us for now)
                    _ => (),
                },

                // Exit the loop when reaching the end of the text slice or file
                Ok(Event::Eof) => break,

                Err(error) => return Err(Error::XmlParsingError(error)),

                // Skip over events that don't interest us
                _ => (),
            }

            buffer.clear();
        }

        Ok(fields)
    }
}
