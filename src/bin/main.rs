use std::error::Error;
use std::time::Instant;

use xmpp_lib::Field;
use xmpp_lib::XmppPubSubParser as Parser;
use xmpp_lib::XMPP_PUBSUB_PUBLISH_WITH_OPTIONS as Slice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Humble Xmpp pubsub protocol parser");

    let start_time = Instant::now();

    // Create a new Xmpp parser instance given a string slice as input
    let mut parser = Parser::from_str(Slice);

    // Parse the input Xmpp message and process parser's events asynchronously
    let fields = parser.parse_fields().await.unwrap();

    let end_time = Instant::now();
    println!("\nExecuted in {:?}\n", end_time.duration_since(start_time));

    print_fields(&fields);

    Ok(())
}

/// Print out field elements
fn print_fields(fields: &Vec<Field>) {
    for entry in fields {
        println!("{:?}", entry);
    }
}
