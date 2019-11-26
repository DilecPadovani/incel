use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

fn option(data: serde_json::value::Value, args: Vec<String>) {
    if args[0] == "-f" {
        let args = args[1..].to_vec(); // not include -f
        italian_to_incel(data, args);
    } else {
        incel_to_italian(data, args);
    }
}
fn clear_screen() {
    io::stdout()
        .write_all("\x1b[2J\x1b[1;1H".as_bytes())
        .unwrap();
}

fn extract_json(file: &str) -> serde_json::value::Value {
    let file = fs::File::open(file).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    json
}

fn hashmapper(data: serde_json::value::Value) -> HashMap<String, (String, String)> {
    data.as_object()
        .unwrap()
        .iter()
        .map(|(key, value)| {
            (
                key.to_lowercase(),
                (key.clone(), value.as_str().unwrap().to_string()),
            )
        })
        .collect()
}

fn italian_to_incel(data: serde_json::value::Value, italian_words: Vec<String>) {
    let m = hashmapper(data);
    for word in italian_words {
        for (_, (keyword, definition)) in &m {
            if definition.to_lowercase().contains(&word.to_lowercase()) {
                println!("Found {} for {} -> {}", keyword.green(), word.blue(), definition.yellow());
            }
        }
    }
}

fn incel_to_italian(data: serde_json::value::Value, incel_words: Vec<String>) {
    let m = hashmapper(data);
    for keyword in incel_words {
        let results = &m.get(&keyword.to_lowercase());
        match results {
            Some((keyword, definition)) => println!("{} -> {}", keyword.green(), definition.blue()),
            None => println!("{} not found.", keyword.red()),
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    println!("{:?}", args);
  //  clear_screen();

    let data = extract_json("data.json");
    option(data, args);
}
