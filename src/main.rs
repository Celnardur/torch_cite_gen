use std::path::{PathBuf, Path};
use std::env;
use std::fs;
use std::collections::HashMap;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: {} src_dir target_dir", args[0]);
    }

    let src_dir = PathBuf::from(&args[1]);
    let target_dir = PathBuf::from(&args[2]);
    println!("To: {:?}", src_dir);
    println!("From: {:?}", target_dir);

    if !target_dir.exists() {
        fs::create_dir(&target_dir)?
    }

    let properties_file = src_dir.join("head.txt");
    let global_head = if let Ok(properties) = fs::read_to_string(&properties_file) {
        Some(Function::parse(&properties))
    } else {
        None
    };

    println!("{:?}", global_head);

    let header_file = src_dir.join("header.html");
    let header = fs::read_to_string(&header_file).ok();
    let mut categories = Vec::new();

    // copy files from src dir and get a list of the sub directories
    for entry in fs::read_dir(&src_dir)? {
        let path = entry?.path();
        if path.is_file() && path != properties_file && path != header_file {
            let target_path = target_dir.join(path.strip_prefix(&src_dir)?);
            fs::copy(path, target_path)?;
        } else if path.is_dir() {
            categories.push(path);
        }
    }

    // generate main page
    let head = gen_head(&global_head.unwrap().args);
    let list = gen_link_list(&categories, &src_dir)?;
    let mut main_page = String::from("<!DOCTYPE html>\n<html>\n");
    main_page.push_str(&head);
    main_page.push_str("<body>\n");
    if let Some(header) = header {
        main_page.push_str(&header);
    }
    main_page.push_str(&list);
    main_page.push_str("</body>\n</html>\n");
    fs::write(target_dir.join("index.html"), main_page)?;

    Ok(())
}

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

fn gen_head(args: &HashMap<String, String>) -> String {
    let mut head = String::new();
    head.push_str("<head>\n");
    for (param, arg) in args {
        match param.as_str() {
            "title" => {
                head.push_str("<title>");
                head.push_str(arg);
                head.push_str("</title>\n");
            },
            "style" => {
                head.push_str("<link rel=\"stylesheet\" href=\"");
                head.push_str(arg);
                head.push_str("\"/>\n");
            },
            _ => (),
        }
    }
    head.push_str("<meta charset=\"UTF-8\"/>\n");
    head.push_str(
        "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"/>\n");
    head.push_str("</head>\n");
    head
}

fn gen_link_list(paths: &Vec<PathBuf>, strip_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut html = String::from("<div class=\"link_list\">\n");
    for src_path in paths {
        html.push_str("<p><h3><a href=\"/");
        html.push_str(src_path.strip_prefix(strip_path)?.to_str().unwrap());
        html.push_str("\">");
        html.push_str(src_path.file_name().unwrap().to_str().unwrap());
        html.push_str("</a></h3></p>\n");
    }
    html.push_str("</div>\n");
    Ok(html)
}
