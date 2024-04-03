use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    fn new(id: u32, name: &str, email: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

fn main() {
    let user = User::new(1, "Meherab Vai", "meherab@example.com");
    
    // Serialize the user instance to a JSON string
    let json_str = match serde_json::to_string(&user) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing user: {}", e);
            return;
        }
    };
    
    println!("Serialized user: {}", json_str);
    
    // Attempt to create the file and handle possible errors gracefully
    let mut file = match File::create("user.json") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to create file: {}", e);
            return;
        }
    };

    // Attempt to write the JSON string to the file and handle possible errors
    if let Err(e) = file.write_all(json_str.as_bytes()) {
        eprintln!("Unable to write data: {}", e);
    }
}
