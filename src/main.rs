use std::path::{PathBuf, Path};
use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Property {
    Title,
    Style,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: {} src_dir target_dir", args[0]);
    }

    let src_dir = Path::new(&args[1]);
    let target_dir = Path::new(&args[2]);
    println!("To: {:?}", src_dir);
    println!("From: {:?}", target_dir);

    let properties_file = src_dir.join("properties.txt");
    
    let universal_properties = if let Ok(properties) = fs::read_to_string(&properties_file) {
        get_properties(&properties)
    } else {
        HashMap::new()
    };

    println!("{:?}", universal_properties);

    let header = fs::read_to_string(&src_dir.join("header.html")).ok();
    println!("{:?}", header);

}

fn get_properties(properties_str: &str) -> HashMap<Property, String> {
    let mut properties = HashMap::new();
    for line in properties_str.split("\n") {
        let mut first_space = 0;
        for (i, c) in line.chars().enumerate() {
            if c == ' ' {
                first_space = i
            }
        }
        if line.chars().nth(0) != Some('#') || first_space == 0 {
            continue;
        }

        let (prop, arg) = line.split_at(first_space);
        let prop = &prop[1..];
        let arg = &arg[1..];

        match prop {
            "title" => properties.insert(Property::Title, arg.to_owned()),
            "style" => properties.insert(Property::Style, arg.to_owned()),
            _ => None,
        };
    }
    properties
}
