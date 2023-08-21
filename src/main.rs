use mongodb::bson::DateTime;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let new_doc = mongodb::bson::doc! {
        "title":"test",
        "name":"hello",
        "time":DateTime::now(),
    };
    println!("{new_doc:#?}");
    Ok(())
}
