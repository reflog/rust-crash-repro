use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct Name {
    field: String,
}

fn main() {
    let n = Name {
        field: "asd".to_string(),
    };

    println!("Hello, world! {:}", serde_json::to_string(&n));
}
