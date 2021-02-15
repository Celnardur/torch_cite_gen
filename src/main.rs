use std::path::{PathBuf, Path};
use std::env;
use std::fs;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct Function {
    name: String,
    args: HashMap<String, String>,
}

impl Function {
    fn parse(block: &str) -> Function {
        let mut first = true;
        let mut name = String::new();
        let mut args = HashMap::new();
        for line in block.lines() {
            if let Some((param, arg)) = Function::parse_line(line) {
                if first {
                    name = param.clone();
                    first = false;
                }
                args.insert(param, arg);
            }
        }
        Function {
            name,
            args,
        }
    }

    fn parse_line(line: &str) -> Option<(String, String)> {
        let first_space = line.find(' ');
        if line.chars().next() != Some('#') || first_space == None {
            return None;
        }

        let (param, arg) = line.split_at(first_space.unwrap());
        Some((param[1..].to_owned(), arg[1..].to_owned()))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: {} src_dir target_dir", args[0]);
    }

    let src_dir = Path::new(&args[1]);
    let target_dir = Path::new(&args[2]);
    println!("To: {:?}", src_dir);
    println!("From: {:?}", target_dir);

    let properties_file = src_dir.join("head.txt");
    let global_head = if let Ok(properties) = fs::read_to_string(&properties_file) {
        Some(Function::parse(&properties))
    } else {
        None
    };

    println!("{:?}", global_head);

    let header_file = src_dir.join("header.html");
    let header = fs::read_to_string(&header_file).ok();

    for entry in fs::read_dir(src_dir)? {
        let path = entry?.path();
        if path.is_file() && path != properties_file && path != header_file {
            let target_path = target_dir.join(path.strip_prefix(src_dir)?);
            fs::copy(path, target_path)?;
        } else if path.is_dir() {
            
        }
    }

    Ok(())
}
