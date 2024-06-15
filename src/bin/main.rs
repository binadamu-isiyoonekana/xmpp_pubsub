use std::collections::HashMap;
use std::time::Instant;

use xmpp_lib::Field;
use xmpp_lib::Result;
use xmpp_lib::XmppPubSubParser as Parser;
use xmpp_lib::LAST_MESSAGE_BODY;
use xmpp_lib::LAST_MESSAGE_RECIPIENT;
use xmpp_lib::LAST_MESSAGE_SENDER;
use xmpp_lib::XMPP_PUBSUB_PUBLISH_WITH_OPTIONS as Slice;

#[tokio::main]
async fn main() {
    println!("Humble Xmpp pubsub protocol parser");

    let start_time = Instant::now();

    // Create a new Xmpp parser instance given a string slice as input
    let mut parser = Parser::from_str(Slice);

    // Parse the input Xmpp message and process parser's events asynchronously
    let fields = parser.parse_fields().await.unwrap();

    let end_time = Instant::now();
    println!("\nExecuted in {:?}\n", end_time.duration_since(start_time));

    let fields = extract_fields(&fields).unwrap();
    println!("Result: {:?}", fields)
}

// Extract requested fields
fn extract_fields(fields: &Vec<Field>) -> Result<HashMap<String, String>> {
    let mut map = HashMap::<String, String>::new();

    for field in fields {
        match field.var.as_str() {
            LAST_MESSAGE_SENDER | LAST_MESSAGE_RECIPIENT | LAST_MESSAGE_BODY => {
                map.insert(field.var.clone(), field.values[0].clone());
                println!("Field: {:?}", field);
            }

            // Skip over other field elements of no interest to us
            _ => (),
        }
    }

    Ok(map)
}
