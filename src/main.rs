use std::fs::write;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let executable_name = args[0]
        .replace("target", "")
        .replace("debug", "")
        .replace("release", "")
        .replace('\\', "");
    if args.len() != 2 {
        println!("Example usage: {} /path/to/file.txt", executable_name);
        panic!("No input txt file provided");
    }
    let mut cleaned_lines = vec![];
    let file_path = &args[1];
    let ammonia = ammonia::Builder::empty();

    let file = std::fs::File::open(file_path)
        .unwrap_or_else(|_| panic!("Failed to open input file {file_path}"));
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(v) = line {
            if v.is_empty() { continue; }
            let result = ammonia
                .clean(v.as_str()).to_string()
                .replace("&nbsp;", " ").to_string();
            cleaned_lines.push(result);
        } else {
            println!("Failed to read line with error: {:?}", line);
        }
    }

    let output_file = format!("{}-result.txt", executable_name.replace(".exe", ""));
    write(output_file.clone(), cleaned_lines.join("\n"))
        .unwrap_or_else(|_| panic!("Failed to write {output_file}"));

    // Report success
    println!("A {output_file} was written to the current path.")
}

