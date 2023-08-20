use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Message {
    text: String,
}


fn print_hello() -> Result<()> {
    let message = Message {
        text: "Hello world".to_owned(),
    };

    let json = serde_json::to_string(&message)?;

    println!("{}", json);

    Ok(())
}

fn main() {
    let _ = print_hello();
}
